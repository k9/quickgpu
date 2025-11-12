use quote::quote as q;
use std::collections::HashMap;

use discover_exports::{
    analysis::Analysis, crate_graph::for_each_node, process::parse_crate, resolve::PathType,
};

use crate::{
    generate::struct_entry::{filter_struct, output_nested_impl, output_struct, without_args},
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
        q!(
            use crate::Nested;
            use std::ops::Range;
            use std::num::NonZeroU32;
            use std::borrow::Cow;
        )
        .to_string(),
    )];

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
        .map(|(comment, code)| format!("{comment}\n{code}\n"))
        .collect::<Vec<String>>()
        .join("\n");

    let output_path = relative_path("quickgpu/src/builders.rs");
    std::fs::write(output_path.clone(), combined)?;
    rustfmt(output_path)?;

    let mut nested_impls = vec![
        q!(
            use crate::Nested;
        )
        .to_string(),
    ];

    for (_, (index, path)) in builder_entries.iter() {
        nested_impls.push(output_nested_impl(
            &wgpu,
            *index,
            path.clone(),
            &builder_entries,
        ));
    }

    let combined = nested_impls
        .iter()
        .map(|code| format!("{code}\n"))
        .collect::<Vec<String>>()
        .join("\n");

    let output_path = relative_path("quickgpu/src/nested.rs");
    std::fs::write(output_path.clone(), combined)?;
    rustfmt(output_path)?;

    Ok(())
}
