use crate::{
    analyze::{FieldParts, StructAnalysis, StructParts, report},
    analyze_default::DefaultValue,
    data,
    utils::{ident, parse_docs, relative_path, rustfmt},
};
use anyhow::Context;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote as q};
use rustdoc_types::{GenericArg, GenericArgs, GenericParamDef, GenericParamDefKind, Type};
use syn::{Expr, Lifetime, parse_str};

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

    let mut wgt_structs = vec![];

    for (_, v) in data.iter_base() {
        if let rustdoc_types::ItemEnum::Use(u) = &v.inner
            && !u.is_glob
            && u.source.starts_with("wgt::")
        {
            wgt_structs.push(u.name.to_string());
        }
    }

    let mut structs = vec![];
    for (_, v) in data.iter_wgt() {
        if let Some(name) = &v.name
            && wgt_structs.contains(name)
        {
            let analysis = StructAnalysis::analyze(v, &data.wgt, &data);
            report(v, &analysis);
            if let StructAnalysis::Parts(parts) = analysis {
                structs.push(parts);
            }
        }
    }

    for (_, v) in data.iter_base() {
        let analysis = StructAnalysis::analyze(v, &data.base, &data);
        report(v, &analysis);
        if let StructAnalysis::Parts(parts) = analysis {
            structs.push(parts);
        }
    }

    let structs = structs
        .into_iter()
        .filter(|p| !SKIP.contains(&p.name.as_str()));

    let mut builders = vec![(
        "".to_string(),
        "
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

    let mut fn_params = vec![];
    for f in &struct_item.fields {
        fn_params.push(field_fn_param(f)?)
    }

    let mut return_params = vec![];
    for f in &struct_item.fields {
        return_params.push(field_return_param(f)?)
    }

    let code = q!(
        #[bon::builder(state_mod(vis="pub(crate)"))]
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

pub struct GeneratedField {
    pub fn_param: TokenStream,
    pub return_param: TokenStream,
}

fn field_fn_param(field: &FieldParts) -> anyhow::Result<TokenStream> {
    let field_name = ident(&field.name);

    let mut attrs = vec![];
    match &field.default_value {
        DefaultValue::None { msg: _ } => (),
        DefaultValue::Default { source: _ } => attrs.push(q!(default)),
        DefaultValue::Value { source: _, value } => attrs.push(q!(default=#value)),
    };

    let field_type = &field.ty;

    if let Type::BorrowedRef { type_, .. } = &field.ty
        && matches!(**type_, Type::Slice(_))
    {
        // Into conversion doesn't work on &[...]
    } else {
        attrs.push(q!(into));
    };

    let field_type = type_tokens(field_type)?;

    let attrs = if attrs.is_empty() {
        q!()
    } else {
        q!(
            #[builder(#(#attrs),*)]
        )
    };

    Ok(q!(
        #attrs
        #field_name: #field_type
    ))
}

fn field_return_param(field: &FieldParts) -> anyhow::Result<TokenStream> {
    let field_name = ident(&field.name);
    Ok(q!(#field_name))
}

fn type_tokens(field_type: &Type) -> anyhow::Result<TokenStream> {
    match field_type {
        Type::ResolvedPath(path) => {
            let args = generic_args(path.args.clone())?;
            let path = path
                .path
                .split("::")
                .last()
                .context("Problem parsing path")?;

            let path = parse_str::<Expr>(path)?;
            Ok(q!(#path #args))
        }
        Type::Primitive(p) => Ok(ident(p).to_token_stream()),
        Type::Generic(g) => Ok(ident(g).to_token_stream()),
        Type::BorrowedRef {
            lifetime,
            is_mutable,
            type_,
        } => {
            let mut tokens = vec![q!(&)];
            if let Some(lifetime) = lifetime {
                let lifetime = parse_str::<Lifetime>(lifetime)?;
                tokens.push(q!(#lifetime));
            }

            if *is_mutable {
                tokens.push(q!(mut));
            }

            tokens.push(type_tokens(type_)?);

            Ok(q!(#(#tokens)*))
        }
        Type::Slice(ty) => {
            let inner = type_tokens(ty)?;
            Ok(q!([#inner]))
        }
        Type::Tuple(tuple) => {
            let mut tokens = vec![];
            for item in tuple {
                tokens.push(type_tokens(item)?);
            }

            Ok(q!(
                (#(#tokens),*)
            ))
        }
        ty => panic!("Failed to handle type {:?}", ty),
    }
}

fn generic_params(struct_item: &StructParts) -> anyhow::Result<TokenStream> {
    let mut struct_generics = vec![];

    for GenericParamDef { name, kind } in &struct_item.generics.params {
        let mut tokens = vec![];
        match kind {
            GenericParamDefKind::Lifetime { outlives: _ } => {
                tokens.push(parse_str::<Lifetime>(name)?.to_token_stream());
            }
            GenericParamDefKind::Type {
                bounds: _,
                default: _,
                is_synthetic: _,
            } => {
                tokens.push(ident(name).to_token_stream());
            }
            _ => (),
        };

        struct_generics.push(q!(#(#tokens)*));
    }

    let struct_generics = if struct_generics.is_empty() {
        q!()
    } else {
        q!(<#(#struct_generics),*>)
    };

    Ok(struct_generics)
}

fn generic_args(args: Option<Box<GenericArgs>>) -> anyhow::Result<TokenStream> {
    let mut struct_generics = vec![];

    if let Some(args) = args
        && let GenericArgs::AngleBracketed {
            args,
            constraints: _,
        } = *args
    {
        for arg in args {
            let mut tokens = vec![];

            match arg {
                GenericArg::Lifetime(lifetime) => {
                    tokens.push(parse_str::<Lifetime>(&lifetime)?.to_token_stream());
                }
                GenericArg::Type(type_) => {
                    tokens.push(type_tokens(&type_)?);
                }
                _ => (),
            };

            struct_generics.push(q!(#(#tokens)*));
        }
    };

    let struct_generics = if struct_generics.is_empty() {
        q!()
    } else {
        q!(<#(#struct_generics),*>)
    };

    Ok(struct_generics)
}
