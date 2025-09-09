mod utils;

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::{Parser, Subcommand};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote as q;
use scraper::{Element, ElementRef, Html};
use syn::Item;

use crate::utils::id;

const MIN_FIELDS: usize = 1;

#[derive(Clone, Copy, Debug)]
struct FieldConfig<'a> {
    pub default: bool,
    pub default_value: Option<&'a str>,
    pub into: bool,
}

impl<'a> Default for FieldConfig<'a> {
    fn default() -> Self {
        Self {
            default: true,
            default_value: None,
            into: true,
        }
    }
}

fn get_field_config<'a>((item, field): (impl Into<String>, impl Into<String>)) -> FieldConfig<'a> {
    #[allow(clippy::match_single_binding)]
    match (item.into().as_str(), field.into().as_str()) {
        ("ComputePassTimestampWrites", "beginning_of_pass_write_index")
        | ("ComputePassTimestampWrites", "end_of_pass_write_index")
        | ("ComputePassDescriptor", "timestamp_writes")
        | ("ComputePipelineDescriptor", "entry_point")
        | ("ComputePipelineDescriptor", "cache")
        | ("ComputePipelineDescriptor", "layout")
        | ("CompilationMessage", "location")
        | ("ColorTargetState", "blend")
        | ("BufferBinding", "size")
        | ("BlasTriangleGeometry", "index_buffer")
        | ("BlasTriangleGeometry", "first_index")
        | ("BlasTriangleGeometry", "transform_buffer")
        | ("BlasTriangleGeometry", "transform_buffer_offset")
        | ("BindGroupLayoutEntry", "count")
        | ("FragmentState", "entry_point")
        | ("ImageSubresourceRange", "mip_level_count")
        | ("ImageSubresourceRange", "array_layer_count")
        | ("MemoryBudgetThresholds", "for_resource_creation")
        | ("MemoryBudgetThresholds", "for_device_loss")
        | ("PipelineCacheDescriptor", "data")
        | ("PrimitiveState", "strip_index_format")
        | ("PrimitiveState", "cull_mode") => FieldConfig {
            default: false,
            ..Default::default()
        },
        _ => FieldConfig::default(),
    }
}

const SKIP: &[&str] = &["AllocatorReport", "HalCounters"];

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
        let mut code: Vec<TokenStream> = vec![q!(
            use std::num::{NonZero, NonZeroU32};
            use std::ops::Range;
            use wgpu::{wgt::TextureSelector, *};
        )];
        let root = self.html.root_element();

        let structs = self.select(&root, "#structs")?;
        let structs = structs.first().context("No struct results")?;
        let structs = structs.next_sibling_element().unwrap();
        let structs = self.select(&structs, "a.struct")?;

        for struct_info in structs {
            self.process_struct(&mut code, struct_info)?;
        }

        utils::output(code, true);

        Ok(())
    }

    fn process_struct(
        &self,
        code: &mut Vec<TokenStream>,
        struct_info: ElementRef<'_>,
    ) -> anyhow::Result<()> {
        let name = utils::text_content(struct_info);
        let struct_path = path(&format!("struct.{name}.html"))?;
        let struct_html = utils::prepare(Html::parse_document(&fs::read_to_string(struct_path)?));
        let root = &struct_html.root_element();

        let struct_decl = self.select(root, ".item-decl")?;
        let struct_decl = utils::text_content(struct_decl[0]);
        if struct_decl.contains("private fields") {
            return Ok(());
        }

        for item in syn::parse_file(&struct_decl)?.items {
            if let Item::Struct(item) = item
                && item.fields.len() > MIN_FIELDS
                && !SKIP.contains(&item.ident.to_string().as_str())
            {
                let fn_ident = &item.ident.to_string().to_case(Case::Snake);
                let fn_ident = fn_ident.replace("_2_d", "_2d");
                let fn_ident = fn_ident.replace("_3_d", "_3d");
                let fn_ident = id(&fn_ident);
                let generics = item.generics;

                let value_ident = &item.ident;

                let fn_params = item.fields.iter().map(|f| {
                    let field_ident = f.ident.as_ref().unwrap();
                    let field_type = &f.ty;

                    let config =
                        get_field_config((value_ident.to_string(), field_ident.to_string()));

                    let mut derives = vec![];

                    if config.default {
                        derives.push(q!(default));
                    };

                    let derives = if derives.is_empty() {
                        q!()
                    } else {
                        q!(#[builder(#(#derives),*)])
                    };

                    q!(
                        #derives
                        #field_ident: #field_type
                    )
                });

                let value_fields = item.fields.iter().map(|f| {
                    let field_ident = f.ident.as_ref().unwrap();

                    println!("(\"{value_ident}\", \"{field_ident}\")");
                    q!(#field_ident)
                });

                let builder = q!(
                    #[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
                    pub fn #fn_ident #generics(
                        #(#fn_params),*
                    ) -> #value_ident #generics {
                        #value_ident {
                            #(#value_fields),*
                        }
                    }
                );

                code.push(builder);
            }
        }

        Ok(())
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
