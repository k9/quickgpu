use std::{fs, path::Path, process};

use convert_case::Casing;
use ra_ap_edition::Edition;
use ra_ap_hir::{
    Adt, Crate, DisplayTarget, HasVisibility, HirDisplay, Impl, ModuleDef, ScopeDef, Semantics,
    StructKind, Visibility, db::DefDatabase,
};
use ra_ap_ide_db::RootDatabase;
use ra_ap_load_cargo::{LoadCargoConfig, ProcMacroServerChoice, load_workspace};
use ra_ap_paths::{AbsPathBuf, Utf8PathBuf};
use ra_ap_project_model::{CargoConfig, ProjectManifest, ProjectWorkspace, RustLibSource};

use crate::{
    default_overrides::{DefaultValue, get_default_value},
    utils::lines,
};

const EDITION: Edition = Edition::Edition2024;

pub fn process() -> anyhow::Result<()> {
    let (path, crate_name) = (
        Path::new("/Users/work/src/ratesttest").to_path_buf(),
        "ratesttest",
    );

    /*let (path, crate_name) = (
        Path::new(env!("CARGO_WORKSPACE_DIR")).join("placeholder"),
        "placeholder",
    );*/

    let mut code = vec![
        "
use bon::builder;
use wgpu::*;
use wgpu::util::*;
    "
        .to_string(),
    ];

    let manifest = ProjectManifest::discover_single(&AbsPathBuf::assert(
        Utf8PathBuf::from_path_buf(path).unwrap(),
    ))
    .unwrap();

    let no_progress = &|_| {};

    let cargo_config = CargoConfig {
        sysroot: Some(RustLibSource::Discover),
        ..Default::default()
    };

    let mut workspace = ProjectWorkspace::load(manifest, &cargo_config, no_progress)?;
    let bs = workspace.run_build_scripts(&cargo_config, no_progress)?;
    workspace.set_build_scripts(bs);

    let load_cargo_config = LoadCargoConfig {
        load_out_dirs_from_check: true,
        prefill_caches: false,
        with_proc_macro_server: ProcMacroServerChoice::Sysroot,
    };

    let (db, _vfs, _) = load_workspace(workspace, &Default::default(), &load_cargo_config).unwrap();
    let semantics = Semantics::new(&db);
    let all_crates = Crate::all(&db);

    let krate = all_crates
        .iter()
        .find(|krate| {
            krate
                .display_name(&db)
                .is_some_and(|name| name.to_string() == crate_name)
        })
        .unwrap();

    let display_target = krate.to_display_target(&db);

    let module = krate.root_module();

    for (name, def) in module.scope(&db, None) {
        if let ScopeDef::ModuleDef(ModuleDef::Adt(Adt::Struct(def))) = def {
            if let Some(value) = struct_builder(&db, display_target, &semantics, module, name, def)
            {
                code.push(lines(value, false));
            }
        } else if let ScopeDef::ModuleDef(ModuleDef::TypeAlias(def)) = def {
            //
            // @todo: try a simple typedef and look at generics
            //

            // rhs of alias expression?
            println!("\nplaceholders");
            let x = def.ty_placeholders(&db); //.generic_params(&db);
            println!("{:?}", x.display_source_code(&db, module.into(), true));
            for ta in x.type_arguments() {
                println!("{:?}", ta.display(&db, display_target).to_string());
            }

            // which parameters are set in rhs of alias
            println!("\nty");
            let x = def.ty(&db);
            println!("{:?}", x.display_source_code(&db, module.into(), true));
            for ta in x.type_arguments() {
                println!("{:?}", ta.display(&db, display_target).to_string());
            }

            println!("\nfields");
            for f in x.fields(&db) {
                let ff = f.0;
                println!("{}", ff.display(&db, display_target));
            }

            // lhs of alias
            println!("\nparams");
            let x = db.type_alias_signature(def.into());
            println!("{}", x.name.display(&db, EDITION));

            let x = x.generic_params.iter_type_or_consts();
            for (_, x) in x {
                println!("{:?}", x);
            }

            if let Some(target) = def.ty(&db).as_adt().and_then(|adt| adt.as_struct()) {
                // struct which alias points to
                println!("\nTarget {}", target.display(&db, display_target));

                for mut ta in target.ty(&db).type_arguments() {
                    println!("{:?}", ta.display(&db, display_target).to_string());
                }

                if let Some(value) =
                    struct_builder(&db, display_target, &semantics, module, name, target)
                {
                    code.push(lines(value, false));
                }
            } else {
                println!("Not adt {:?}", def.name(&db).as_str())
            }
        }
    }

    let code = lines(code, true);

    let output_path = Path::new(env!("CARGO_WORKSPACE_DIR")).join("quickgpu/src/builders.rs");
    fs::write(&output_path, code)?;

    process::Command::new("rustfmt")
        .args(output_path.to_str())
        .output()?;

    Ok(())
}

fn struct_builder(
    db: &RootDatabase,
    display_target: DisplayTarget,
    semantics: &Semantics<'_, RootDatabase>,
    module: ra_ap_hir::Module,
    name: ra_ap_hir::Name,
    def: ra_ap_hir::Struct,
) -> Option<Vec<String>> {
    if def.kind(db) != StructKind::Record
        || def.visibility(db) != Visibility::Public
        || def
            .fields(db)
            .iter()
            .any(|f| f.visibility(db) != Visibility::Public)
    {
        return None;
    };

    let mut code = vec![];
    if let Some(default_impl) = get_default_impl(db, def.ty(db)) {
        let text = default_impl_text(semantics, db, default_impl);
        code.push(format!(
            "
/*
    Default impl on type to build:

    {text}
*/"
        ));
    };

    let placeholders = def.ty_placeholders(db);
    let type_arguments = placeholders
        .type_and_const_arguments(db, display_target)
        .map(|t| t.to_string());

    let ty = def.ty(db);
    let lifetimes = ty
        .generic_parameters(db, display_target)
        .map(|p| p.to_string())
        .filter(|p| p != "{unknown}");

    let generics = type_arguments
        .chain(lifetimes)
        .collect::<Vec<_>>()
        .join(", ");

    let generics = if generics.is_empty() {
        "".to_string()
    } else {
        format!("<{generics}>")
    };

    let struct_name = name.display(db, EDITION).to_string();

    let struct_name_fn = struct_name.to_case(convert_case::Case::Snake);

    code.push(format!(
        "#[builder(state_mod(vis = \"pub(crate)\"))]
        pub fn {struct_name_fn}{generics}(",
    ));

    for f in def.fields(db) {
        let has_default = get_default_impl(db, f.ty(db)).is_some();

        let field_name = f.name(db).display(db, EDITION).to_string();
        let field_type = f
            .ty(db)
            .display_source_code(db, module.into(), true)
            .unwrap();

        let default_attr =
            match get_default_value(&struct_name, &field_name, &field_type, has_default) {
                DefaultValue::None => "".to_string(),
                DefaultValue::Default => "default".to_string(),
                DefaultValue::Value(value) => format!("default={value}"),
            };

        let builder_args = default_attr;
        let builder_args = if builder_args.is_empty() {
            "".to_string()
        } else {
            format!("#[builder({builder_args})]")
        };

        code.push(format!(
            "
    {builder_args}
    {field_name}: {field_type},",
        ));
    }

    code.push(format!(
        "
) -> {struct_name}{generics} {{
    {struct_name} {{"
    ));

    for f in def.fields(db) {
        let name = f.name(db).display(db, EDITION).to_string();
        code.push(format!("{name},"));
    }

    code.push(
        "
    }
}"
        .to_string(),
    );

    Some(code)
}

fn get_default_impl(db: &RootDatabase, ty: ra_ap_hir::Type<'_>) -> Option<Impl> {
    let impls = Impl::all_for_type(db, ty);

    impls.into_iter().find(|i| {
        i.trait_(db)
            .is_some_and(|t| t.name(db).as_str() == "Default")
    })
}

pub fn default_impl_text<'db>(
    semantics: &Semantics<'db, RootDatabase>,
    db: &RootDatabase,
    impl_block: Impl,
) -> String {
    let source = semantics.source(impl_block).unwrap();

    let maybe_macro = source
        .syntax()
        .parent_ancestors_with_macros(db)
        .next()
        .unwrap()
        .file_id
        .macro_file();

    if let Some(source) = maybe_macro {
        source.call_node(db).value.text().to_string()
    } else {
        source.syntax().value.text().to_string()
    }
}
