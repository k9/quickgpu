mod utils;

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::{Parser, Subcommand};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote as q};
use scraper::{Element, ElementRef, Html};
use syn::{Ident, Item};

use crate::utils::{id, output};

const MIN_FIELDS: usize = 1;

#[derive(Clone, Debug)]
pub enum DefaultValue {
    None,
    Default,
    Value(TokenStream),
}

#[derive(Clone, Debug)]
struct FieldConfig {
    pub default_value: DefaultValue,
    pub into: bool,
    pub name: Ident,
    pub ty: TokenStream,
}

#[derive(Clone, Debug)]
struct StructConfig {
    pub fields: Vec<FieldConfig>,
    pub name: Ident,
    pub fn_name: Ident,
    pub generics: TokenStream,
}

const SKIP: &[&str] = &[
    "AdapterInfo",
    "AllocatorReport",
    "BackendOptions",
    "BufferTextureCopyInfo",
    "BufferTransition",
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

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Generate => {}
    };

    let path = path("index.html")?;
    let mut generator = Generator::new(Html::parse_document(&fs::read_to_string(path)?));

    generator.generate()?;

    Ok(())
}

fn path(file: &str) -> anyhow::Result<PathBuf> {
    Ok(Path::new(env!("CARGO_WORKSPACE_DIR"))
        .join("wgpu/target/doc/wgpu/")
        .join(file)
        .canonicalize()?)
}

pub struct Generator {
    html: Html,
}

impl Generator {
    pub fn new(html: Html) -> Self {
        let html = utils::prepare(html);
        Self { html }
    }

    pub fn generate(&mut self) -> anyhow::Result<()> {
        let root = self.html.root_element();

        let structs = self.select(&root, "#structs")?;
        let structs = structs.first().context("No struct results")?;
        let structs = structs.next_sibling_element().unwrap();
        let structs = self.select(&structs, "a.struct")?;

        let mut struct_configs = vec![];
        for struct_info in structs {
            if let Some(config) = self.process_struct(struct_info)? {
                let config = self.customize_config(config);
                struct_configs.push(config);
            };
        }

        let mut builders = vec![q!(
            use std::num::{NonZero, NonZeroU32};
            use std::ops::Range;
            use wgpu::{wgt::TextureSelector, *};
        )];

        for config in struct_configs.iter() {
            builders.push(self.emit_builder(config.clone()));
        }

        output(&builders, true);

        println!("{struct_configs:#?}");

        Ok(())
    }

    fn process_struct(&self, struct_info: ElementRef<'_>) -> anyhow::Result<Option<StructConfig>> {
        let name = utils::text_content(struct_info);
        let struct_path = path(&format!("struct.{name}.html"))?;
        let struct_html = utils::prepare(Html::parse_document(&fs::read_to_string(struct_path)?));
        let root = &struct_html.root_element();

        let struct_decl = self.select(root, ".item-decl")?;
        let struct_decl = utils::text_content(struct_decl[0]);
        if struct_decl.contains("private fields") {
            return Ok(None);
        }

        let file = syn::parse_file(&struct_decl)?;
        let item = file.items.first().context("Couldn't parse struct")?;

        if let Item::Struct(item) = item
            && item.fields.len() > MIN_FIELDS
            && !SKIP.contains(&item.ident.to_string().as_str())
        {
            let fn_ident = &item.ident.to_string().to_case(Case::Snake);
            let fn_ident = fn_ident.replace("_2_d", "_2d");
            let fn_ident = fn_ident.replace("_3_d", "_3d");
            let fn_ident = id(&fn_ident);
            let generics = item.generics.clone();

            let value_ident = &item.ident;

            let mut config = StructConfig {
                name: value_ident.clone(),
                fn_name: fn_ident,
                fields: vec![],
                generics: generics.to_token_stream(),
            };

            for f in item.fields.iter() {
                let field_ident = f.ident.clone().unwrap();
                let field_type = f.ty.clone();

                config.fields.push(FieldConfig {
                    name: field_ident,
                    ty: field_type.to_token_stream(),
                    default_value: DefaultValue::Default,
                    into: true,
                });
            }

            Ok(Some(config))
        } else {
            Ok(None)
        }
    }

    fn customize_config(&self, struct_config: StructConfig) -> StructConfig {
        let mut struct_config = struct_config.clone();
        for field in struct_config.fields.iter_mut() {
            let ty = field.ty.to_string();
            if ty.starts_with("Option <") || ty.starts_with("& ") {
                field.default_value = DefaultValue::None;
            }

            if ty.starts_with("Label <") {
                field.default_value = DefaultValue::Value(q!(None));
            }

            // Shut off defaults for primitives including bools, unless in IDL

            match (
                struct_config.name.to_string().as_str(),
                field.name.to_string().as_str(),
            ) {
                ("AdapterInfo", "backend")
                | ("AdapterInfo", "device_type")
                | ("BindGroupEntry", "resource")
                | ("BindGroupLayoutEntry", "visibility")
                | ("BindGroupLayoutEntry", "ty")
                | ("BlasBuildEntry", "geometry")
                | ("BlendComponent", "src_factor")
                | ("BlendComponent", "dst_factor")
                | ("BufferTransition", "buffer")
                | ("BufferTransition", "state")
                | ("ColorTargetState", "format")
                | ("CompilationMessage", "message_type")
                | ("CopyExternalImageDestInfo", "texture")
                | ("CopyExternalImageDestInfo", "color_space")
                | ("DepthStencilState", "format")
                | ("DepthStencilState", "depth_compare")
                | ("DownlevelCapabilities", "flags")
                | ("DownlevelCapabilities", "shader_model")
                | ("PushConstantRange", "stages")
                | ("RenderBundleDepthStencil", "format")
                | ("RenderPipelineDescriptor", "vertex")
                | ("ShaderModeulDescriptor", "source")
                | ("StencilFaceState", "compare")
                | ("ShaderModuleDescriptor", "source")
                | ("SurfaceCapabilities", "usages")
                | ("TexelCopyBufferInfoBase", "buffer")
                | ("TexelCopyTextureInfoBase", "texture")
                | ("TextureFormatFeatures", "allowed_usages")
                | ("TextureFormatFeatures", "flags")
                | ("TextureTransition", "texture")
                | ("TextureTransition", "state")
                | ("VertexAttribute", "format")
                // comment
                 => field.default_value = DefaultValue::None,
                _ => (),
            };

            if (struct_config.name == "Operations" && field.name == "load") {
                field.default_value = DefaultValue::Value(q!(LoadOp::Load))
            }
        }

        struct_config
    }

    fn emit_builder(&self, struct_config: StructConfig) -> TokenStream {
        let StructConfig {
            fields,
            name,
            fn_name,
            generics,
        } = struct_config;

        let fn_params = fields.iter().map(|f| {
            let name = f.name.clone();
            let ty = f.ty.clone();

            let mut derives = vec![];

            match f.default_value {
                DefaultValue::None => (),
                DefaultValue::Default => derives.push(q!(default)),
                DefaultValue::Value(ref s) => derives.push(q!(default=#s)),
            };

            let attrs = if derives.is_empty() {
                q!()
            } else {
                q!(#[builder(#(#derives),*)])
            };

            q!(
                #attrs
                #name: #ty
            )
        });

        let return_fields = fields.iter().map(|f| {
            let name = f.name.clone();
            q!(#name)
        });

        q!(
            #[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
            pub fn #fn_name #generics(
                #(#fn_params),*
            ) -> #name #generics {
                #name {
                    #(#return_fields),*
                }
            }
        )
    }

    pub fn select<'a>(
        &'a self,
        within: &'a ElementRef,
        selector: &'a str,
    ) -> anyhow::Result<Vec<ElementRef<'a>>> {
        let selector = scraper::Selector::parse(selector).unwrap();
        Ok(within.select(&selector).collect())
    }
}
