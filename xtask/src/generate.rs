use clap::ValueEnum;
use quote::{ToTokens, format_ident, quote as q};
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
        shared::{custom, intro},
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

impl Version {
    pub fn wgpu_source(&self) -> String {
        match self {
            Version::V27 => "wgpu_27",
            Version::V28 => "wgpu_28",
        }
        .to_string()
    }

    pub fn wgpu_ident(&self) -> Ident {
        format_ident!("{}", self.wgpu_source())
    }
}

pub struct CreateWithDevice {
    pub path: TypePath,
    pub name: Ident,
    pub output: ReturnType,
    pub use_reference: bool,
}

pub fn generate(version: Version) -> anyhow::Result<()> {
    let mut analysis = Analysis::default();

    let wgpu_source = version.wgpu_source();
    let wgpu = {
        parse_crate(
            &mut analysis,
            relative_path("expanded/wgpu_types.rs"),
            relative_path(format!("{}/wgpu-types", version.wgpu_source())),
            "wgpu_types",
            vec![],
        )
        .unwrap();

        parse_crate(
            &mut analysis,
            relative_path("expanded/wgpu_core.rs"),
            relative_path(format!("{}/wgpu-core", version.wgpu_source())),
            "wgpu_core",
            vec!["wgpu_types".to_string()],
        )
        .unwrap();

        parse_crate(
            &mut analysis,
            relative_path("expanded/wgpu_hal.rs"),
            relative_path(format!("{}/wgpu-hal", version.wgpu_source())),
            "wgpu_hal",
            vec!["wgpu_core".to_string()],
        )
        .unwrap();

        parse_crate(
            &mut analysis,
            relative_path("expanded/naga.rs"),
            relative_path(format!("{}/naga", version.wgpu_source())),
            "wgpu_hal",
            vec![],
        )
        .unwrap();

        parse_crate(
            &mut analysis,
            relative_path("expanded/wgpu.rs"),
            relative_path(format!("{}/wgpu", version.wgpu_source())),
            &wgpu_source,
            vec![],
        )
        .unwrap()
    };

    let intro_path = relative_path("quickgpu/INTRO.md".to_string());
    std::fs::write(intro_path.clone(), intro(version))?;

    let base_path = relative_path(format!("quickgpu/src/{}/", version.wgpu_source()));
    let sh = Shell::new()?;
    sh.remove_path(base_path.clone())?;
    sh.create_dir(base_path)?;

    let builders_path = relative_path(format!("quickgpu/src/{}/builders/", version.wgpu_source()));
    let sh = Shell::new()?;
    sh.create_dir(builders_path)?;

    let custom = custom(version);

    let mut builders = vec![];
    let mut builder_entries = HashMap::new();

    let mut create_with_device = vec![];

    let resolution = resolve_path(
        &wgpu,
        wgpu.crate_root,
        &path_from_string(&format!("{}::Device", version.wgpu_source())),
    )
    .unwrap();

    resolve_impls(&wgpu, resolution)
        .unwrap()
        .iter()
        .for_each(|impl_| {
            impl_.items.iter().for_each(|item| {
                if let Some(create_with) = should_create_with(item, version) {
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
            version,
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
            relative_path(format!("quickgpu/src/{}/builders/", version.wgpu_source()))
                .join(format!("{}.rs", name));

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

    let output_path = relative_path(format!(
        "quickgpu/src/{}/builders/mod.rs",
        version.wgpu_source()
    ));

    std::fs::write(output_path.clone(), builder_mods)?;
    rustfmt(output_path)?;

    let builder_uses = builders
        .iter()
        .map(|Output { builder_use, .. }| {
            format!(
                "
{builder_use}
"
            )
        })
        .collect::<String>();

    let mod_src = format!(
        r#"
{custom}

{builder_uses}
"#
    );

    let output_path = relative_path(format!("quickgpu/src/{}/mod.rs", version.wgpu_source()));
    std::fs::write(output_path.clone(), mod_src)?;
    rustfmt(output_path)?;

    Ok(())
}

fn should_create_with(item: &syn::ImplItem, version: Version) -> Option<CreateWithDevice> {
    let wgpu_ident = version.wgpu_ident();
    if let syn::ImplItem::Fn(item_fn) = item
        && item_fn.sig.unsafety.is_none()
        && let name = item_fn.sig.ident.to_string()
        && name.starts_with("create_")
        && item_fn.sig.inputs.len() == 2
        && let Some(syn::FnArg::Receiver(reciever)) = item_fn.sig.inputs.get(0)
        && ([
            quote::quote!(&#wgpu_ident::Device).to_string(),
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
                if let syn::Type::Path(path) = *type_reference.elem {
                    return Some(CreateWithDevice {
                        path,
                        name: item_fn.sig.ident.clone(),
                        use_reference: true,
                        output: item_fn.sig.output.clone(),
                    });
                };
            }
            _ => (),
        };
    }

    None
}
