use clap::ValueEnum;
use quote::{ToTokens, quote as q};
use std::collections::HashMap;
use syn::{Ident, ReturnType, TypePath};
use xshell::Shell;

use discover_exports::{
    analysis::Analysis,
    crate_graph::for_each_node,
    process::parse_crate,
    resolve::{PathType, resolve_impls, resolve_path},
    utils::path_from_string,
};

use crate::{
    generate::{
        shared::{binder, cargo_toml, intro},
        struct_entry::{Output, filter_struct, output_struct},
    },
    utils::{relative_path, rustfmt, without_args},
};

pub mod base;
pub mod builder;
pub mod docs;
pub mod nested;
pub mod setter;
pub mod shared;
pub mod state;
mod struct_entry;
pub mod tests;
pub mod types;

const SKIP: &[&str] = &[
    "AdapterInfo",
    "AllocatorReport",
    "BufferTextureCopyInfo",
    "CompilationMessage",
    "CoreCounters",
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

#[derive(Copy, Clone, ValueEnum)]
pub enum Version {
    V27,
    V28,
}

pub struct CreateWithDevice {
    pub path: TypePath,
    pub name: Ident,
    pub output: ReturnType,
    pub use_reference: bool,
}

pub fn generate(version: Version) -> anyhow::Result<()> {
    let mut analysis = Analysis::default();

    let wgpu_source = match version {
        Version::V27 => "wgpu_v27",
        Version::V28 => "wgpu_v28",
    };

    let crate_name = match version {
        Version::V27 => "quickgpu27",
        Version::V28 => "quickgpu",
    };

    let wgpu = {
        parse_crate(
            &mut analysis,
            relative_path("expanded/wgpu_types.rs"),
            relative_path(format!("{wgpu_source}/wgpu-types")),
            "wgpu_types",
            vec![],
        )
        .unwrap();

        parse_crate(
            &mut analysis,
            relative_path("expanded/wgpu_core.rs"),
            relative_path(format!("{wgpu_source}/wgpu-core")),
            "wgpu_core",
            vec!["wgpu_types".to_string()],
        )
        .unwrap();

        parse_crate(
            &mut analysis,
            relative_path("expanded/wgpu_hal.rs"),
            relative_path(format!("{wgpu_source}/wgpu-hal")),
            "wgpu_hal",
            vec!["wgpu_core".to_string()],
        )
        .unwrap();

        parse_crate(
            &mut analysis,
            relative_path("expanded/naga.rs"),
            relative_path(format!("{wgpu_source}/naga")),
            "wgpu_hal",
            vec![],
        )
        .unwrap();

        parse_crate(
            &mut analysis,
            relative_path("expanded/wgpu.rs"),
            relative_path(format!("{wgpu_source}/wgpu")),
            "wgpu",
            vec![],
        )
        .unwrap()
    };

    let intro_path = relative_path(format!("{crate_name}/INTRO.md"));
    std::fs::write(intro_path.clone(), intro(version))?;

    let cargo_path = relative_path(format!("{crate_name}/Cargo.toml"));
    std::fs::write(cargo_path.clone(), cargo_toml(version))?;

    binder(version)?;

    let builders_path = relative_path(format!("{crate_name}/src/builders/"));
    let sh = Shell::new()?;
    sh.remove_path(builders_path.clone())?;
    sh.create_dir(builders_path)?;

    let mut builders = vec![];
    let mut builder_entries = HashMap::new();

    let mut create_with_device = vec![];

    let resolution =
        resolve_path(&wgpu, wgpu.crate_root, &path_from_string("wgpu::Device")).unwrap();

    resolve_impls(&wgpu, resolution)
        .unwrap()
        .iter()
        .for_each(|impl_| {
            impl_.items.iter().for_each(|item| {
                if let Some(create_with) = should_create_with(item) {
                    create_with_device.push(create_with);
                }
            });
        });

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

    let mut builder_entries_sorted = builder_entries
        .iter()
        .map(|(_, (index, path))| (index, path))
        .collect::<Vec<_>>();

    builder_entries_sorted.sort_by(|(_, a_path), (_, b_path)| {
        a_path
            .segments
            .last()
            .into_token_stream()
            .to_string()
            .cmp(&b_path.segments.last().into_token_stream().to_string())
    });

    for (index, path) in builder_entries_sorted {
        builders.push(output_struct(
            &wgpu,
            *index,
            path.clone(),
            &builder_entries,
            &create_with_device,
        ));
    }

    for Output {
        name,
        comment,
        code,
        ..
    } in builders.iter()
    {
        let combined = format!(
            "
// The code in this file is generated by a script. Do not edit it directly.
//

{comment}
{code}
        "
        );

        let output_path =
            relative_path(format!("{crate_name}/src/builders/")).join(format!("{}.rs", name));

        std::fs::write(output_path.clone(), combined)?;
        rustfmt(output_path)?;
    }

    let builder_mods = builders
        .iter()
        .map(|Output { builder_mod, .. }| {
            format!(
                "
{builder_mod}
"
            )
        })
        .collect::<String>();

    let output_path = relative_path(format!("{crate_name}/src/builders/mod.rs"));
    std::fs::write(output_path.clone(), builder_mods)?;
    rustfmt(output_path)?;

    let use_statements = builders
        .iter()
        .map(|Output { use_statement, .. }| {
            format!(
                "
{use_statement}
"
            )
        })
        .collect::<String>();

    let use_statements = format!(
        r#"
#![doc=include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/INTRO.md"))]

pub mod custom;
pub use custom::*;
pub mod builders;
pub mod binder;
pub use binder_macros::*;

{use_statements}
"#
    );

    let output_path = relative_path(format!("{crate_name}/src/lib.rs"));
    std::fs::write(output_path.clone(), use_statements)?;
    rustfmt(output_path)?;

    Ok(())
}

fn should_create_with(item: &syn::ImplItem) -> Option<CreateWithDevice> {
    if let syn::ImplItem::Fn(item_fn) = item
        && item_fn.sig.unsafety.is_none()
        && let name = item_fn.sig.ident.to_string()
        && name.starts_with("create_")
        && item_fn.sig.inputs.len() == 2
        && let Some(syn::FnArg::Receiver(reciever)) = item_fn.sig.inputs.get(0)
        && ([
            quote::quote!(&wgpu::Device).to_string(),
            quote::quote!(wgpu::util::DeviceExt).to_string(),
        ]
        .contains(&reciever.ty.clone().into_token_stream().to_string()))
        && let Some(syn::FnArg::Typed(desc)) = item_fn.sig.inputs.get(1)
        && desc
            .ty
            .clone()
            .into_token_stream()
            .to_string()
            .contains("Descriptor")
    {
        let ty = desc.ty.clone();
        match *ty {
            syn::Type::Path(path) => {
                return Some(CreateWithDevice {
                    path,
                    name: item_fn.sig.ident.clone(),
                    use_reference: false,
                    output: item_fn.sig.output.clone(),
                });
            }
            syn::Type::Reference(type_reference) => {
                match *type_reference.elem {
                    syn::Type::Path(path) => {
                        return Some(CreateWithDevice {
                            path,
                            name: item_fn.sig.ident.clone(),
                            use_reference: true,
                            output: item_fn.sig.output.clone(),
                        });
                    }
                    _ => (),
                };
            }
            _ => (),
        };
    }

    None
}
