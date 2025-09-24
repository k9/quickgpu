use std::{fs, path::Path};

use convert_case::Casing;
use ra_ap_edition::Edition;
use ra_ap_hir::{
    Adt, Crate, DisplayTarget, HasVisibility, HirDisplay, Impl, ModuleDef, ScopeDef, Semantics,
    StructKind, Visibility,
};
use ra_ap_ide_db::RootDatabase;
use ra_ap_load_cargo::{LoadCargoConfig, ProcMacroServerChoice, load_workspace};
use ra_ap_paths::{AbsPathBuf, Utf8PathBuf};
use ra_ap_project_model::{CargoConfig, ProjectManifest, ProjectWorkspace, RustLibSource};

use crate::default_overrides::{DefaultValue, get_default_value};

const EDITION: Edition = Edition::Edition2024;

pub fn process() -> anyhow::Result<()> {
    /*let (path, crate_name) = (
        Path::new("/Users/work/src/ratesttest").to_path_buf(),
        "ratesttest",
    );*/

    let (path, crate_name) = (
        Path::new(env!("CARGO_WORKSPACE_DIR")).join("placeholder"),
        "placeholder",
    );

    let mut code = vec![
        "
use bon::builder;
use wgpu::*;
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
            if def.kind(&db) == StructKind::Record
                && def.visibility(&db) == Visibility::Public
                && def
                    .fields(&db)
                    .iter()
                    .all(|f| f.visibility(&db) == Visibility::Public)
            {
                code.extend_from_slice(&struct_builder(
                    &db,
                    display_target,
                    &semantics,
                    module,
                    name,
                    def,
                ));
            }
        }
    }

    //let code = prettify(&code);
    let code = code.join("\n").replace("\n\n", "\n");

    fs::write(
        Path::new(env!("CARGO_WORKSPACE_DIR")).join("quickgpu/src/builders.rs"),
        code,
    )?;

    Ok(())
}

fn struct_builder(
    db: &RootDatabase,
    display_target: DisplayTarget,
    semantics: &Semantics<'_, RootDatabase>,
    module: ra_ap_hir::Module,
    name: ra_ap_hir::Name,
    def: ra_ap_hir::Struct,
) -> Vec<String> {
    let mut code = vec![];
    let ty = def.ty(db);
    if let Some(default_impl) = get_default_impl(db, def.ty(db)) {
        let text = default_impl_text(semantics, db, default_impl);
        code.push(format!(
            "
/*
    {text}
*/"
        ));
    };

    let placeholders = def.ty_placeholders(db);
    let type_arguments = placeholders
        .type_and_const_arguments(db, display_target)
        .map(|t| t.to_string());

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
        "
#[builder(state_mod(vis = \"pub(crate)\"))]
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
    {struct_name} {{
"
    ));

    for f in def.fields(db) {
        let name = f.name(db).display(db, EDITION).to_string();
        code.push(format!(
            "
        {name},
"
        ));
    }

    code.push(
        "
    }
}
"
        .to_string(),
    );

    code
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
