use anyhow::bail;
use discover_exports::{
    Analysis, AnalysisStruct, discover, parse_crate,
    utils::{path_refs_string, path_segments, path_string},
};
use quote::quote as q;
use syn::{Fields, FieldsNamed, Path, Visibility};

use crate::utils::relative_path;

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

    let _structs = exports
        .structs
        .into_iter()
        .map(|struct_item| parse_struct(struct_item))
        .collect::<AResult<Vec<_>>>()?
        .into_iter()
        .filter_map(|x| x)
        .collect::<Vec<_>>();

    /*
    let structs = structs
        .into_iter()
        .filter(|p| !SKIP.contains(&p.name.as_str()));

    let mut builders = vec![(
        "".to_string(),
        "
use std::borrow::Cow;
use std::ops::Range;
use std::num::NonZeroU32;

use wgpu::*;
use wgpu::util::*;
use wgpu::wgt::{Dx12SwapchainKind, Dx12UseFrameLatencyWaitableObject, TextureSelector};
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

    let output_path = relative_path("quickgpu/src/builders.rs");
    std::fs::write(output_path.clone(), combined)?;
    rustfmt(output_path)?;*/

    Ok(())
}

pub fn parse_struct(exported: AnalysisStruct) -> AResult<Option<AnalysisStruct>> {
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
