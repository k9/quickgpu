use anyhow::Context;
use rustdoc_types::{ItemEnum, Type};

use crate::{
    analyze::core::{StructAnalysis, report},
    data::{self, DataItem},
    output::core::{UseInstance, output_struct},
    type_alias_helpers::{TypeAliasMap, get_type_alias_map},
    utils::{final_path, parse_docs, relative_path, rustfmt},
};

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
    let doc_path = rustdoc_json::Builder::default()
        .toolchain("nightly")
        .manifest_path(relative_path("doc_target/Cargo.toml"))
        .build()?;

    let doc_dir = doc_path.parent().context("Couldn't get doc dir")?;

    let data = data::Data::new(
        parse_docs(doc_dir.join("wgpu.json"))?,
        parse_docs(doc_dir.join("wgpu_types.json"))?,
    );

    /*
        wgpu_types: struct TexelCopyBufferInfo
        wgpu: use wgt::TexelCopyBufferInfo as TexelCopyBufferInfoBase
          exported as struct with "Base" and original generics
        wgpu: type TexelCopyBufferInfo = TexelCopyBufferInfoBase
          exported as type alias w/ new generics
    */

    let mut use_items_wgt = vec![];
    for DataItem { item, .. } in data.iter_base() {
        if let rustdoc_types::ItemEnum::Use(u) = &item.inner
            && !u.is_glob
            && u.source.starts_with("wgt::")
        {
            use_items_wgt.push(UseInstance {
                source: final_path(&u.source.clone())?,
                name: u.name.clone(),
            });
        }
    }

    let mut structs = vec![];
    for DataItem { item, .. } in data.iter_wgt() {
        if let Some(name) = &item.name
            && let Some(wgt) = use_items_wgt.iter().find(|wgt| &wgt.source == name)
        {
            let mut item = item.clone();
            item.name = Some(wgt.name.clone());

            let analysis = StructAnalysis::analyze(&item, &data.wgt, &data, TypeAliasMap::None);
            report(&item, &analysis);

            if let StructAnalysis::Parts(parts) = analysis {
                structs.push(parts);
            }
        }
    }

    for DataItem { item, .. } in data.iter_base() {
        let analysis = StructAnalysis::analyze(item, &data.base, &data, TypeAliasMap::None);
        report(item, &analysis);
        if let StructAnalysis::Parts(parts) = analysis {
            structs.push(parts);
        }
    }

    for DataItem { item, .. } in data.iter_base() {
        if let ItemEnum::TypeAlias(ta) = &item.inner
            && let Type::ResolvedPath(path) = &ta.type_
        {
            let mut ta_path = final_path(&path.path)?;
            if let Some(use_item) = use_items_wgt.iter().find(|u| u.name == ta_path) {
                ta_path = final_path(&use_item.source)?;
            };

            let target = data.iter_both().find(|DataItem { item, .. }| {
                item.name.as_deref() == Some(&ta_path)
                    && !matches!(item.inner, ItemEnum::TypeAlias(_))
            });

            if let Some(target) = target {
                let map = get_type_alias_map(item, target.item, ta, path);
                let analysis = StructAnalysis::analyze(target.item, target.krate, &data, map);
                report(target.item, &analysis);

                if let StructAnalysis::Parts(parts) = analysis {
                    structs.push(parts);
                }
            }
        };
    }

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

    rustfmt(output_path)?;

    Ok(())
}
