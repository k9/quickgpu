use anyhow::Context;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote as q};
use scraper::{Element, ElementRef, Html};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};
use syn::{
    Expr, Fields, GenericArgument, Item, PathArguments, PathSegment, Type, TypePath,
    punctuated::Punctuated,
    token::{Comma, PathSep},
};

use crate::{
    customize::{DefaultValue, FieldConfig, StructConfig, customize_config},
    utils::{self, doc_path, id},
};

const MIN_FIELDS: usize = 0;

const SKIP: &[&str] = &[
    "AdapterInfo",
    "AllocatorReport",
    "BackendOptions",
    "BlasBuildEntry",
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

pub struct StructOutput {
    pub config: StructConfig,
    pub struct_types: HashSet<String>,
    pub field_types: HashSet<String>,
}

pub struct Generator {
    html: Html,
}

type KeyValuePairs = Vec<(String, String)>;

impl Generator {
    pub fn new(html: Html) -> Self {
        let html = utils::prepare(html);
        Self { html }
    }

    pub fn generate(&mut self) -> anyhow::Result<Vec<TokenStream>> {
        let root = self.html.root_element();
        let mut struct_types = HashSet::new();
        let mut field_types = HashSet::new();

        let mut struct_configs = vec![];

        let structs = self.select(&root, "#structs")?;
        let structs = structs.first().context("No struct results")?;
        let structs = structs.next_sibling_element().unwrap();
        let structs = self.select(&structs, "a.struct")?;
        let structs = structs
            .into_iter()
            .map(|s| self.process_struct(s))
            .collect::<Vec<_>>();

        let type_aliases = self.select(&root, "#types")?;
        let type_aliases = type_aliases.first().context("No struct results")?;
        let type_aliases = type_aliases.next_sibling_element().unwrap();
        let type_aliases = self.select(&type_aliases, "a.type")?;
        let type_aliases = type_aliases
            .into_iter()
            .map(|s| self.process_type_alias(s))
            .collect::<Vec<_>>();

        for struct_info in structs.into_iter().chain(type_aliases.into_iter()) {
            if let Some(StructOutput {
                config,
                struct_types: inner_struct_types,
                field_types: inner_field_types,
            }) = struct_info?
            {
                struct_types.extend(inner_struct_types.into_iter());
                field_types.extend(inner_field_types.into_iter());
                struct_configs.push(customize_config(config));
            };
        }

        let mut struct_entries = vec![];
        for ty in struct_types.iter() {
            add_default_check(&mut struct_entries, q!(structs), ty);
        }

        let mut field_entries = vec![];
        for ty in field_types.iter() {
            add_default_check(&mut field_entries, q!(fields), ty);
        }

        let code = q!(
            #![feature(specialization)]

            mod maybe_default;
            use std::ops::Range;
            use wgpu::*;

            use crate::maybe_default::{ DummyStruct, MaybeDefault };

            pub fn main() -> Result::<(), Box<dyn std::error::Error>> {
                let mut structs: Vec<(String, String)> = vec![];
                let mut fields: Vec<(String, String)> = vec![];

                #(#struct_entries;)*
                #(#field_entries;)*

                println!("{:?}", (structs.clone(), fields.clone()));
                Ok(())
            }
        );

        let dest_path = Path::new(env!("CARGO_WORKSPACE_DIR")).join("default_check/src/main.rs");
        fs::write(&dest_path, code.to_string()).unwrap();

        let stdout = duct::cmd!("cargo", "+nightly", "run", "-p", "default_check").read()?;
        let (struct_entries, field_entries): (KeyValuePairs, KeyValuePairs) =
            ron::de::from_str(&stdout)?;

        let struct_entries = struct_entries
            .into_iter()
            .collect::<HashMap<String, String>>();

        let field_entries = field_entries
            .into_iter()
            .collect::<HashMap<String, String>>();

        let mut builders = vec![q!(
            use std::borrow::Cow;
            use std::num::{NonZero, NonZeroU32};
            use std::ops::Range;
            use wgpu::{wgt::TextureSelector, *};
        )];

        for config in struct_configs.iter() {
            builders.push(emit_builder(
                config.clone(),
                &struct_entries,
                &field_entries,
            ));
        }

        Ok(builders)
    }

    fn process_struct(&self, struct_info: ElementRef<'_>) -> anyhow::Result<Option<StructOutput>> {
        let name = utils::text_content(struct_info);
        let struct_path = doc_path(&format!("struct.{name}.html"))?;
        let struct_html = utils::prepare(Html::parse_document(&fs::read_to_string(struct_path)?));
        let root = &struct_html.root_element();

        let struct_decl = self.select(root, ".item-decl")?;
        let struct_decl = utils::text_content(struct_decl[0]);
        if struct_decl.contains("private fields") {
            return Ok(None);
        }

        let file = syn::parse_file(&struct_decl)?;
        let item = file.items.first().context("Couldn't parse struct")?;

        Ok(self.process_struct_decl(item))
    }

    fn process_type_alias(
        &self,
        struct_info: ElementRef<'_>,
    ) -> anyhow::Result<Option<StructOutput>> {
        let name = utils::text_content(struct_info);
        let struct_path = doc_path(&format!("type.{name}.html"))?;
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

        Ok(self.process_struct_decl(item))
    }

    pub fn select<'s>(
        &'s self,
        within: &'s ElementRef,
        selector: &'s str,
    ) -> anyhow::Result<Vec<ElementRef<'s>>> {
        let selector = scraper::Selector::parse(selector).unwrap();
        Ok(within.select(&selector).collect())
    }

    pub fn process_struct_decl(&self, item: &Item) -> Option<StructOutput> {
        let mut struct_types = HashSet::new();
        let mut field_types = HashSet::new();

        let Item::Struct(item) = item else {
            return None;
        };

        if !(matches!(item.fields, Fields::Named(_))
            && item.fields.len() > MIN_FIELDS
            && !SKIP.contains(&item.ident.to_string().as_str()))
        {
            return None;
        };

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

        struct_types.insert(q!(#value_ident #generics).to_string());

        for f in item.fields.iter() {
            let field_ident = f.ident.clone().unwrap();
            let field_type = f.ty.clone();

            field_types.insert(field_type.to_token_stream().to_string());

            config.fields.push(FieldConfig {
                name: field_ident,
                ty: field_type.to_token_stream(),
                default_value: DefaultValue::Skip,
                into: true,
            });
        }

        Some(StructOutput {
            config,
            struct_types,
            field_types,
        })
    }
}

fn add_default_check(entries: &mut Vec<TokenStream>, container: TokenStream, ty: &str) {
    if ty.len() == 1 || ty.starts_with("Option") || ty.starts_with("Cow") {
        return;
    }

    let Ok(pty) = syn::parse_str::<Type>(ty) else {
        return;
    };

    match pty.clone() {
        Type::Path(mut ty) => {
            process_default_generics(&mut ty);

            if let Ok(expr) = syn::parse_str::<Expr>(q!(#ty::maybe_default()).to_string().as_str())
            {
                let ty_string = pty.to_token_stream().to_string();
                entries.push(q!(
                    if let Some(expr) = #expr {
                        #container.push((#ty_string.to_string(), format!("{:?}", expr),));
                    }
                ));
            } else {
                println!("failed {}", q!(#ty::maybe_default()).to_string().as_str());
            }
        }
        _ => println!("other {}", pty.to_token_stream()),
    };
}

fn process_default_generics(ty: &mut TypePath) {
    let Some(x) = ty.path.segments.last_mut() else {
        return;
    };

    let PathArguments::AngleBracketed(ref mut x) = x.arguments else {
        return;
    };

    x.colon2_token = Some(PathSep::default());

    let mut type_params = Punctuated::<GenericArgument, Comma>::new();
    x.args
        .iter()
        .filter(|p| matches!(p, GenericArgument::Type(_)))
        .for_each(|_| {
            let mut segments = Punctuated::new();
            segments.push(PathSegment {
                ident: id("DummyStruct"),
                arguments: PathArguments::None,
            });
            type_params.push(GenericArgument::Type(Type::Path(TypePath {
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments,
                },
            })));
        });

    x.args = type_params;
}

fn emit_builder(
    struct_config: StructConfig,
    struct_entries: &HashMap<String, String>,
    field_entries: &HashMap<String, String>,
) -> TokenStream {
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

    let defaults = if let Some(entry) = struct_entries.get(&q!(#name #generics).to_string()) {
        let expr = syn::parse_str::<Expr>(entry);
        if let Ok(Expr::Struct(expr)) = expr {
            let f = expr.fields.iter().next().unwrap();
            println!("{}", q!(#f));
        }

        "".to_string()
    } else {
        "".to_string()
    };

    q!(
        #[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
        pub fn #fn_name #generics(
            #(#fn_params),*
        ) -> #name #generics {
            let zzz = #defaults;
            #name {
                #(#return_fields),*
            }
        }
    )
}
