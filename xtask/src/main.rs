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
    Skip,
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

        let mut struct_configs = vec![];

        let structs = self.select(&root, "#structs")?;
        let structs = structs.first().context("No struct results")?;
        let structs = structs.next_sibling_element().unwrap();
        let structs = self.select(&structs, "a.struct")?;

        for struct_info in structs {
            if let Some(config) = self.process_struct(struct_info)? {
                let config = self.customize_config(config);
                struct_configs.push(config);
            };
        }

        let type_aliases = self.select(&root, "#types")?;
        let type_aliases = type_aliases.first().context("No struct results")?;
        let type_aliases = type_aliases.next_sibling_element().unwrap();
        let type_aliases = self.select(&type_aliases, "a.type")?;

        for type_alias_info in type_aliases {
            if let Some(config) = self.process_type_alias(type_alias_info)? {
                let config = self.customize_config(config);
                struct_configs.push(config);
            };
        }

        let mut builders = vec![q!(
            use std::borrow::Cow;
            use std::num::{NonZero, NonZeroU32};
            use std::ops::Range;
            use wgpu::{wgt::TextureSelector, *};
        )];

        for config in struct_configs.iter() {
            builders.push(self.emit_builder(config.clone()));
        }

        output(&builders, true);

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

        process_struct_decl(item)
    }

    fn process_type_alias(
        &self,
        struct_info: ElementRef<'_>,
    ) -> anyhow::Result<Option<StructConfig>> {
        let name = utils::text_content(struct_info);
        let struct_path = path(&format!("type.{name}.html"))?;
        let struct_html = utils::prepare(Html::parse_document(&fs::read_to_string(struct_path)?));
        let root = &struct_html.root_element();

        let type_alias = self.select(root, "#aliased-type")?;
        let Some(type_alias) = type_alias.first() else {
            return Ok(None);
        };

        let type_alias = type_alias.next_sibling_element().unwrap();

        let type_alias_decl = utils::text_content(type_alias);
        if type_alias_decl.contains("private fields") {
            return Ok(None);
        }

        let file = syn::parse_file(&type_alias_decl)?;
        let item = file.items.first().context("Couldn't parse struct")?;

        if let Item::Struct(item) = item
            && item.fields.len() > MIN_FIELDS
            && !SKIP.contains(&item.ident.to_string().as_str())
        {
            let id = &item.ident;
            println!("{} {}", name, q!(#id));
        }

        process_struct_decl(item)
    }

    fn customize_config(&self, struct_config: StructConfig) -> StructConfig {
        let mut struct_config = struct_config.clone();
        struct_config.fields = struct_config
            .fields
            .iter()
            .map(|field| customize_default(&struct_config, field))
            .map(|field| customize_into(&field))
            .collect();

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
                DefaultValue::Skip => (),
                DefaultValue::Default => derives.push(q!(default)),
                DefaultValue::Value(ref s) => derives.push(q!(default=#s)),
            };

            if f.into {
                derives.push(q!(into));
            }

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

fn customize_default(struct_config: &StructConfig, field: &FieldConfig) -> FieldConfig {
    let mut field = field.clone();
    let ty = &field.ty;

    if ty.to_string() == q!(Label<'a>).to_string() {
        field.default_value = DefaultValue::Value(q!(None));
    }

    if ty.to_string() == q!(PowerPreference).to_string() {
        field.default_value = DefaultValue::Default
    }

    if struct_config.name == "Operations" && field.name == "load" {
        field.default_value = DefaultValue::Value(q!(LoadOp::Load))
    }

    if struct_config.name == "RequestAdapterOptionsBase" && field.name == "force_fallback_adapter" {
        field.default_value = DefaultValue::Value(q!(false));
    }

    field
}

fn customize_into(field: &FieldConfig) -> FieldConfig {
    let mut field = field.clone();
    let ty = &field.ty;

    if ty.to_string().starts_with("& 'a [") {
        field.into = false;
    }

    field
}

fn process_struct_decl(item: &Item) -> Result<Option<StructConfig>, anyhow::Error> {
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
                default_value: DefaultValue::Skip,
                into: true,
            });
        }

        Ok(Some(config))
    } else {
        Ok(None)
    }
}
