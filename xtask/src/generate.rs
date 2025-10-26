use anyhow::Context;
use quote::quote as q;
use rustdoc_types::{ItemEnum, Type};

use crate::{
    AResult,
    analyze::core::{StructAnalysis, report},
    data::{self, DataItem},
    output::{
        core::{UseInstance, output_struct},
        types::generic_params,
    },
    type_alias_helpers::{TypeAliasMap, get_type_alias_map},
    utils::{final_path, ident, parse_docs, relative_path, rustfmt},
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

pub enum BuilderOutput {
    Code(String),
    Comment(String),
}

pub fn generate() -> AResult<()> {
    let doc_path = rustdoc_json::Builder::default()
        .toolchain("nightly")
        .manifest_path(relative_path("wgpu/wgpu/Cargo.toml"))
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
                let map = get_type_alias_map(item.name.clone().unwrap(), target.item, ta, path);
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
        .filter(|p| !SKIP.contains(&p.name.as_str()))
        .collect::<Vec<_>>();

    let mut builders: Vec<BuilderOutput> =
        vec![
            BuilderOutput::Code(
                q!(
                    use std::borrow::Cow;
                    use std::ops::Range;
                    use std::num::NonZeroU32;

                    use wgpu::*;
                    use wgpu::util::*;
                    use wgpu::wgt::{
                        Dx12SwapchainKind, Dx12UseFrameLatencyWaitableObject, TextureSelector,
                    };

                    use crate::Nested;
                )
                .to_string(),
            ),
        ];

    let mut initializers = vec![];
    let mut builder_structs = vec![];
    let builder_types = structs
        .iter()
        .map(|s| {
            let name = ident(&s.name);
            let generics = generic_params(&s, &[]).unwrap();
            [
                q!(#name #generics).to_string(),
                //q!(Option<#name #generics>).to_string(),
            ]
        })
        .flatten()
        .collect::<Vec<_>>();

    for struct_item in structs {
        let output = output_struct(struct_item, &builder_types)?;
        initializers.push(output.initializer);
        builder_structs.push(output.builder_struct);
        builders.push(BuilderOutput::Comment(output.comment));
        builders.push(BuilderOutput::Code(output.code));
    }

    builders.push(BuilderOutput::Code(
        q!(
            pub mod initializers {
                pub use super::{
                    #(#initializers),*
                };
            }

            pub mod builders {
                pub use super::{
                    #(#builder_structs),*
                };
            }
        )
        .to_string(),
    ));

    let combined = builders
        .iter()
        .map(|item| match item {
            BuilderOutput::Code(code) => format!("{code}\n"),
            BuilderOutput::Comment(comment) => format!("{comment}\n"),
        })
        .collect::<Vec<String>>()
        .join("\n");

    let output_path = relative_path("quickgpu/src/inner.rs");
    std::fs::write(output_path.clone(), combined)?;

    rustfmt(output_path)?;

    Ok(())
}
