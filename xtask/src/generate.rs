use discover_exports::{
    analysis::Analysis, crate_graph::for_each_node, process::parse_crate, resolve::PathType,
};

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
    let wgpu = {
        parse_crate(
            &mut analysis,
            relative_path("expanded/wgpu_types.rs"),
            relative_path("wgpu/wgpu-types"),
            "wgpu_types",
            vec![],
        )
        .unwrap();

        parse_crate(
            &mut analysis,
            relative_path("expanded/wgpu_core.rs"),
            relative_path("wgpu/wgpu-core"),
            "wgpu_core",
            vec!["wgpu_types".to_string()],
        )
        .unwrap();

        parse_crate(
            &mut analysis,
            relative_path("expanded/wgpu_hal.rs"),
            relative_path("wgpu/wgpu-hal"),
            "wgpu_hal",
            vec!["wgpu_core".to_string()],
        )
        .unwrap();

        parse_crate(
            &mut analysis,
            relative_path("expanded/naga.rs"),
            relative_path("wgpu/naga"),
            "wgpu_hal",
            vec![],
        )
        .unwrap();

        let wgpu = parse_crate(
            &mut analysis,
            relative_path("expanded/wgpu.rs"),
            relative_path("wgpu/wgpu"),
            "wgpu",
            vec![],
        )
        .unwrap();

        wgpu
    };

    let mut builders = vec![(
        "".to_string(),
        "
use std::ops::Range;
use std::num::NonZeroU32;
use std::borrow::Cow;
"
        .to_string(),
    )];

    for_each_node(
        &wgpu,
        |index| {
            if let Some(output) = output_struct(&wgpu, index).unwrap() {
                builders.push(output);
            }
        },
        PathType::PublicOnly,
    )
    .unwrap();

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
