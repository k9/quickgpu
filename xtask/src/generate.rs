use anyhow::bail;
use convert_case::{Case, Casing};
use discover_exports::{
    Analysis, AnalysisStruct, discover, parse_crate,
    utils::{id, path_refs_string, path_string},
};
use quote::quote as q;
use syn::{Fields, FieldsNamed, Visibility};

use crate::utils::{relative_path, rustfmt};

type AResult<T> = anyhow::Result<T>;

const SKIP: &[&str] = &[
    "AdapterInfo",
    "AllocatorReport",
    "BufferTextureCopyInfo",
    "CompilationMessage",
    "DownlevelCapabilities",
    "Features",
    "GLBackendOptions",
    "HalCounters",
    "InternalCounters",
    "Limits",
    "SourceLocation",
    "SurfaceCapabilities",
    "TextureFormatFeatures",
];

pub fn generate() -> anyhow::Result<()> {
    let mut analysis = Analysis::default();

    let root_index = parse_crate(
        &mut analysis,
        relative_path("expanded/wgpu.rs"),
        relative_path("wgpu/wgpu"),
        "wgpu",
    )?;

    parse_crate(
        &mut analysis,
        relative_path("expanded/wgpu_types.rs"),
        relative_path("wgpu/wgpu-types"),
        "wgpu_types",
    )
    .unwrap();

    parse_crate(
        &mut analysis,
        relative_path("expanded/wgpu_core.rs"),
        relative_path("wgpu/wgpu-core"),
        "wgpu_core",
    )
    .unwrap();

    parse_crate(
        &mut analysis,
        relative_path("expanded/wgpu_hal.rs"),
        relative_path("wgpu/wgpu-hal"),
        "wgpu_hal",
    )
    .unwrap();

    parse_crate(
        &mut analysis,
        relative_path("expanded/naga.rs"),
        relative_path("wgpu/naga"),
        "naga",
    )
    .unwrap();

    let exports = discover(&mut analysis, root_index).unwrap();

    let structs = exports
        .structs
        .into_iter()
        .map(|struct_item| filter_struct(struct_item))
        .collect::<AResult<Vec<_>>>()?
        .into_iter()
        .filter_map(|x| x)
        .collect::<Vec<_>>();

    let mut builders = vec![(
        "".to_string(),
        "
use std::ops::Range;
use std::num::NonZeroU32;
"
        .to_string(),
    )];

    for struct_item in structs {
        builders.push(output_struct(struct_item)?);
    }

    let combined = builders
        .iter()
        .map(|(comment, code)| format!("{comment}\n{code}\n"))
        .collect::<Vec<String>>()
        .join("\n");

    let output_path = relative_path("quickgpu/src/inner.rs");
    std::fs::write(output_path.clone(), combined)?;
    rustfmt(output_path)?;

    Ok(())
}

fn output_struct(entry: AnalysisStruct) -> AResult<(String, String)> {
    let comment = "".to_string();
    let item = &entry.item;
    let ident = &item.ident;
    let fn_ident = id(ident.to_string().to_case(Case::Snake).as_str());
    let path = &entry.path;
    let generics = &item.generics;

    let fn_params = struct_fields(&entry)?.named.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        q!(#ident: #ty)
    });

    let struct_values = struct_fields(&entry)?.named.iter().map(|f| {
        let ident = &f.ident;
        q!(#ident)
    });

    let code = q! {
        pub fn #fn_ident #generics(
            #(#fn_params),*
        ) -> #(#path)::* #generics {
            #(#path)::* {
                #(#struct_values),*
            }
        }
    };

    Ok((comment, code.to_string()))
}

pub fn filter_struct(exported: AnalysisStruct) -> AResult<Option<AnalysisStruct>> {
    if SKIP.contains(&exported.item.ident.to_string().as_str()) {
        log::debug!("Skipping {} since it's in skip list", exported.item.ident);

        return Ok(None);
    }

    let Ok(fields) = &struct_fields(&exported) else {
        log::debug!(
            "Skipping {} since it doesn't have named fields",
            exported.item.ident
        );

        return Ok(None);
    };

    if fields
        .named
        .iter()
        .any(|f| !matches!(f.vis, Visibility::Public(_)))
    {
        log::debug!(
            "Skipping {} since it has non-public fields",
            exported.item.ident
        );

        return Ok(None);
    };

    log::debug!(
        "{} {}",
        path_string(&exported.path),
        exported
            .impls
            .iter()
            .map(|i| i
                .trait_
                .as_ref()
                .map_or("".to_string(), |t| { path_refs_string(&t.1) }))
            .collect::<Vec<String>>()
            .join("\n")
    );

    for f in &fields.named {
        let ident = &f.ident;
        let ty = &f.ty;
        log::debug!("    {}", q!(#ident: #ty));
    }

    Ok(Some(exported))
}

pub fn struct_fields(entry: &AnalysisStruct) -> AResult<&FieldsNamed> {
    let Fields::Named(named) = &entry.item.fields else {
        bail!("Struct doesn't have named fields {}", entry.item.ident);
    };

    Ok(named)
}
