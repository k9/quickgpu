use quote::quote as q;
use std::collections::HashMap;

use discover_exports::{
    analysis::Analysis, crate_graph::for_each_node, process::parse_crate, resolve::PathType,
};

use crate::{
    generate::struct_entry::{Output, filter_struct, output_struct},
    utils::{relative_path, rustfmt, without_args},
};

pub mod builder;
pub mod nested;
mod struct_entry;

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

    let mut builders = vec![Output {
        builder_comment: "".to_string(),
        builder_code: q!().to_string(),
        nested_impl: q!(
            use crate::Nested;
        )
        .to_string(),
    }];

    let mut builder_entries = HashMap::new();

    for_each_node(
        &wgpu,
        |(index, path)| {
            if filter_struct(&wgpu, index, &path).is_some() {
                let idents = without_args(&path);
                builder_entries.insert(q!(#idents).to_string(), (index, path.clone()));
            }
        },
        PathType::TopLevelPublicOnly,
    )
    .unwrap();

    for (_, (index, path)) in builder_entries.iter() {
        builders.push(output_struct(&wgpu, *index, path.clone(), &builder_entries));
    }

    let combined = builders
        .iter()
        .map(
            |Output {
                 builder_comment,
                 builder_code,
                 ..
             }| format!("{builder_comment}\n{builder_code}\n"),
        )
        .collect::<Vec<String>>()
        .join("\n");

    let output_path = relative_path("quickgpu/src/builders.rs");
    std::fs::write(output_path.clone(), combined)?;
    rustfmt(output_path)?;

    let combined = builders
        .iter()
        .map(|Output { nested_impl, .. }| format!("{nested_impl}\n"))
        .collect::<Vec<String>>()
        .join("\n");

    let output_path = relative_path("quickgpu/src/nested.rs");
    std::fs::write(output_path.clone(), combined)?;
    rustfmt(output_path)?;

    Ok(())
}
