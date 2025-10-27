use discover_exports::{Analysis, AnalysisEdge, AnalysisStruct, Exported, discover, parse_crate};
use syn::Fields;

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
        "crate",
    )?;

    let root_types_index = parse_crate(
        &mut analysis,
        relative_path("expanded/wgpu_types.rs"),
        relative_path("wgpu/wgpu-types"),
        "wgt",
    )
    .unwrap();

    analysis
        .graph
        .update_edge(root_index, root_types_index, AnalysisEdge::Normal);

    let exports = discover(analysis, root_index).unwrap();

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

pub fn parse_struct(
    exported: Exported<AnalysisStruct>,
) -> AResult<Option<Exported<AnalysisStruct>>> {
    if SKIP.contains(&exported.item.item.ident.to_string().as_str()) {
        log::debug!(
            "Skipping {} since it's in skip list",
            exported.item.item.ident
        );

        return Ok(None);
    }

    if !matches!(exported.item.item.fields, Fields::Named(_)) {
        log::debug!(
            "Skipping {} since it doesn't have named fields",
            exported.item.item.ident
        );

        return Ok(None);
    };

    Ok(Some(exported))
}
