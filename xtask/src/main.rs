mod utils;

use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    sync::{LazyLock, RwLock},
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

const SKIP: &[&str] = &["AllocatorReport", "HalCounters"];
const SKIP_DEFAULT: &[&str] = &[
    "DeviceType",
    "Backend",
    "& 'a BindGroupLayout",
    "ShaderStages",
    "BindingType",
    "& 'a BlasTriangleGeometrySizeDescriptor",
    "& 'a Buffer",
    "BlendFactor",
    "TextureFormat",
    "CompilationMessageType",
    "& 'a QuerySet",
    "& 'a ShaderModule",
    "T",
    "B",
    "VertexState < 'a >",
    "PredefinedColorSpace",
    "CompareFunction",
    "ShaderModel",
    "DownlevelFlags",
    "& 'tex TextureView",
    "TextureUses",
    "TextureUsages",
    "VertexFormat",
    "ShaderSource < 'a >",
    "TextureFormatFeatureFlags",
    "BufferUses",
    "BindingResource < 'a >",
    "& 'a Blas",
    "BlasGeometries < 'a >",
    "LoadOp < V >",
    // @todo provide a default
    /*
    "u32",
    "u64",
    "f32",
    "f64",
    "i32",
    "i64",
    */
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

pub static FIELD_TYPE_SET: LazyLock<RwLock<HashSet<String>>> =
    LazyLock::new(|| RwLock::new(HashSet::new()));

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

        let fts = FIELD_TYPE_SET.read().unwrap();
        for field in fts.iter() {
            println!("{field}");
        }

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

                let fn_params = item.fields.iter().map(|f| {
                    let field_ident = &f.ident;
                    let field_type = &f.ty;

                    let ty = q!(#field_type).to_string();

                    {
                        FIELD_TYPE_SET.write().unwrap().insert(ty.clone());
                    }

                    if ty.starts_with("Option <") || SKIP_DEFAULT.contains(&ty.as_str()) {
                        q!(
                            #field_ident: #field_type
                        )
                    } else {
                        q!(
                            #[builder(default)]
                            #field_ident: #field_type
                        )
                    }
                });

                let value_ident = &item.ident;

                let value_fields = item.fields.iter().map(|f| {
                    let field_ident = f.ident.as_ref().unwrap();

                    println!("{value_ident}::{field_ident}");
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
