use anyhow::Context;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote as q};
use scraper::{Element, ElementRef, Html};
use std::{collections::HashMap, fs};
use syn::Item;

use crate::{
    customize::{DefaultValue, FieldConfig, StructConfig, customize_config},
    utils::{self, doc_path, id},
};

const MIN_FIELDS: usize = 1;

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

pub struct Generator {
    html: Html,
    idl_hints: HashMap<String, String>,
}

impl Generator {
    pub fn new(html: Html, idl_hints: HashMap<String, String>) -> Self {
        let html = utils::prepare(html);
        Self { html, idl_hints }
    }

    pub fn generate(&mut self) -> anyhow::Result<Vec<(String, TokenStream)>> {
        let root = self.html.root_element();

        let mut struct_configs = vec![];

        let structs = self.select(&root, "#structs")?;
        let structs = structs.first().context("No struct results")?;
        let structs = structs.next_sibling_element().unwrap();
        let structs = self.select(&structs, "a.struct")?;

        for struct_info in structs {
            if let Some(config) = self.process_struct(struct_info)? {
                let mut config = customize_config(config);
                config.idl_hint = self.get_idl_hint(config.name.to_string());

                struct_configs.push(config);
            };
        }

        let type_aliases = self.select(&root, "#types")?;
        let type_aliases = type_aliases.first().context("No struct results")?;
        let type_aliases = type_aliases.next_sibling_element().unwrap();
        let type_aliases = self.select(&type_aliases, "a.type")?;

        for type_alias_info in type_aliases {
            if let Some(config) = self.process_type_alias(type_alias_info)? {
                let config = customize_config(config);
                struct_configs.push(config);
            };
        }

        let mut builders = vec![(
            "".to_string(),
            q!(
                use std::borrow::Cow;
                use std::num::{NonZero, NonZeroU32};
                use std::ops::Range;
                use wgpu::{wgt::TextureSelector, *};
            ),
        )];

        for config in struct_configs.iter() {
            builders.push(emit_builder(config.clone()));
        }

        Ok(builders)
    }

    fn get_idl_hint(&self, config_name: String) -> Option<String> {
        let config_name = match config_name.as_str() {
            "TexelCopyBufferInfoBase" => "TexelCopyBufferInfo",
            "TexelCopyTextureInfoBase" => "TexelCopyTextureInfo",
            "RequestAdapterOptionsBase" => "RequestAdapterOptions",
            "DepthBiasState" => "DepthStencilState",
            "StencilState" => "DepthStencilState",
            "RenderBundleDepthStencil" => "RenderBundleEncoderDescriptor",
            _ => config_name.as_str(),
        };

        let config_name = format!("GPU{config_name}");
        self.idl_hints.get(&config_name).cloned()
    }

    fn process_struct(&self, struct_info: ElementRef<'_>) -> anyhow::Result<Option<StructConfig>> {
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

        process_struct_decl(item)
    }

    fn process_type_alias(
        &self,
        struct_info: ElementRef<'_>,
    ) -> anyhow::Result<Option<StructConfig>> {
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

        process_struct_decl(item)
    }

    pub fn select<'s>(
        &'s self,
        within: &'s ElementRef,
        selector: &'s str,
    ) -> anyhow::Result<Vec<ElementRef<'s>>> {
        let selector = scraper::Selector::parse(selector).unwrap();
        Ok(within.select(&selector).collect())
    }
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
            idl_hint: None,
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

fn emit_builder(struct_config: StructConfig) -> (String, TokenStream) {
    let StructConfig {
        fields,
        name,
        fn_name,
        generics,
        idl_hint,
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

    let idl_hint = match idl_hint {
        Some(hint) => format!("/*\n\n{hint}\n\n*/"),
        None => "".to_string(),
    };

    (
        idl_hint,
        q!(
            #[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
            pub fn #fn_name #generics(
                #(#fn_params),*
            ) -> #name #generics {
                #name {
                    #(#return_fields),*
                }
            }
        ),
    )
}
