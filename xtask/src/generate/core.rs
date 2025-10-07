use std::cmp::Ordering;

use crate::{
    analyze::core::{StructAnalysis, StructParts, report},
    data::{self, DataItem},
    generate::{
        field::{field_details, field_fn_param, field_return_param},
        types::generic_params,
    },
    type_alias_helpers::{TypeAliasMap, get_type_alias_map},
    utils::{final_path, ident, parse_docs, relative_path, rustfmt},
};
use convert_case::{Case, Casing};
use quote::quote as q;
use rustdoc_types::{ItemEnum, Type};

#[derive(Debug)]
pub struct UseInstance {
    pub source: String,
    pub name: String,
}

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
    let doc_path = relative_path("wgpu/wgpu");

    std::process::Command::new("cargo")
        .current_dir(doc_path)
        .env("RUSTDOCFLAGS", "-Z unstable-options --output-format=json")
        .arg("+nightly")
        .arg("doc")
        .output()?;

    let data = data::Data::new(
        parse_docs("wgpu/target/doc/wgpu.json")?,
        parse_docs("wgpu/target/doc/wgpu_types.json")?,
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
                let map = get_type_alias_map(target.item, ta, path);
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
use wgpu::wgt::TextureSelector;
"
        .to_string(),
    )];

    for struct_item in structs {
        builders.push(generate_struct(struct_item)?);
    }

    let combined = builders
        .iter()
        .map(|(comment, code)| format!("{comment}\n{code}\n"))
        .collect::<Vec<String>>()
        .join("\n");

    let combined = rustfmt(combined)?;

    let output_path = relative_path("quickgpu/src/builders.rs");
    std::fs::write(output_path, combined)?;

    Ok(())
}

fn generate_struct(struct_item: StructParts) -> anyhow::Result<(String, String)> {
    let comment = struct_item.default_value.get_comment();
    let name = &struct_item.name;
    let struct_ident = ident(name);
    let struct_generics = generic_params(&struct_item)?;
    let fn_ident = ident(&name.to_case(Case::Snake));
    let comment = format!("/*\n{comment}\n*/");

    let mut details = vec![];
    for f in &struct_item.fields {
        details.push(field_details(f, &struct_item.type_alias_map)?)
    }

    details.sort_by(|a, _b| {
        if a.start_fn.is_some() {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });

    let mut fn_params = vec![];
    for f in details {
        fn_params.push(field_fn_param(f)?)
    }

    let mut return_params = vec![];
    for f in &struct_item.fields {
        return_params.push(field_return_param(f)?)
    }

    let code = q!(
        #[bon::builder(state_mod(vis="pub(crate)"))]
        #[builder(derive(Into))]
        pub fn #fn_ident #struct_generics(
            #(#fn_params),*
        ) -> #struct_ident #struct_generics {
            #struct_ident {
                #(#return_params),*
            }
        }
    );

    let code = code.to_string();
    Ok((comment, code))
}
