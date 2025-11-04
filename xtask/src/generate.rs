use discover_exports::{
    Analysis, AnalysisEnum,
    analysis::{AnalysisEntry, AnalysisRef},
    crate_graph::filter_map_nodes,
    discover, parse_crate,
};

use crate::{
    generate::struct_entry::filter_struct,
    utils::{relative_path, rustfmt},
};

mod struct_entry;

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

    let structs = filter_map_nodes(&analysis, root_index, |node_index| {
        let Ok(AnalysisRef::Struct(entry)) = AnalysisEntry::node_index_ref(&analysis, node_index)
        else {
            return None;
        };

        filter_struct(entry)
    });

    exports
        .structs
        .into_iter()
        .map(|struct_item| struct_entry::filter_struct(struct_item))
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
        builders.push(struct_entry::output_struct(struct_item)?);
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
