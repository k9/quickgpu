use discover_exports::{Analysis, crate_graph::for_each_node, discover, parse_crate};

use crate::{
    generate::struct_entry::output_struct,
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

    let wgpu_types = parse_crate(
        &mut analysis,
        relative_path("expanded/wgpu_types.rs"),
        relative_path("wgpu/wgpu-types"),
        "wgpu_types",
        vec![],
    )
    .unwrap();
    analysis.root_index = wgpu_types;
    discover(&mut analysis).unwrap();

    let wgpu_core = parse_crate(
        &mut analysis,
        relative_path("expanded/wgpu_core.rs"),
        relative_path("wgpu/wgpu-core"),
        "wgpu_core",
        vec![("wgpu_types".to_string(), wgpu_types)],
    )
    .unwrap();
    analysis.root_index = wgpu_core;
    discover(&mut analysis).unwrap();

    let wgpu_hal = parse_crate(
        &mut analysis,
        relative_path("expanded/wgpu_hal.rs"),
        relative_path("wgpu/wgpu-hal"),
        "wgpu_hal",
        vec![("wgpu_core".to_string(), wgpu_core)],
    )
    .unwrap();
    analysis.root_index = wgpu_hal;
    discover(&mut analysis).unwrap();

    let naga = parse_crate(
        &mut analysis,
        relative_path("expanded/naga.rs"),
        relative_path("wgpu/naga"),
        "wgpu_hal",
        vec![],
    )
    .unwrap();
    analysis.root_index = naga;
    discover(&mut analysis).unwrap();

    let wgpu = parse_crate(
        &mut analysis,
        relative_path("expanded/wgpu.rs"),
        relative_path("wgpu/wgpu"),
        "wgpu",
        vec![],
    )
    .unwrap();
    analysis.root_index = wgpu;
    discover(&mut analysis).unwrap();

    let mut builders = vec![(
        "".to_string(),
        "
use std::ops::Range;
use std::num::NonZeroU32;
"
        .to_string(),
    )];

    for_each_node(&analysis, |index| {
        if let Some(output) = output_struct(&analysis, index).unwrap() {
            builders.push(output);
        }
    });

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
