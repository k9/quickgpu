use proc_macro2::TokenStream;
use quote::{ToTokens, quote as q};
use rustdoc_types::Type;
use syn::Ident;

use crate::{
    analyze::{core::FieldParts, field_default::FieldDefault},
    output::types::type_tokens,
    type_alias_helpers::TypeAliasMap,
    utils::ident,
};

pub struct GeneratedField {
    pub fn_param: TokenStream,
    pub return_param: TokenStream,
}

pub struct FieldDetails {
    pub name: Ident,
    pub path: String,
    pub ty: TokenStream,
    pub start_fn: Option<TokenStream>,
    pub into: Option<TokenStream>,
    pub default: Option<TokenStream>,
}

pub fn field_fn_param(
    FieldDetails {
        name,
        ty,
        start_fn,
        into,
        default,
        path,
        ..
    }: FieldDetails,
) -> anyhow::Result<TokenStream> {
    let attrs = [start_fn, into, default]
        .into_iter()
        .filter_map(|item| item)
        .collect::<Vec<_>>();

    let attrs = if attrs.is_empty() {
        q!()
    } else {
        q!(
            #[builder(#(#attrs),*)]
        )
    };

    let doc = format!("Sets [`{path}::{name}`]");
    Ok(q!(
        #[rustfmt::skip]
        #[doc=#doc]
        #attrs
        #name: #ty
    ))
}

pub fn field_details(
    field: &FieldParts,
    path: &str,
    type_alias_map: &TypeAliasMap,
) -> anyhow::Result<FieldDetails> {
    let name = ident(&field.name);
    let ty = &field.ty;
    let ty = type_tokens(ty, type_alias_map)?;
    let mut start_fn = None;
    let mut into = None;
    let mut default = None;

    match &field.default_value {
        FieldDefault::None { msg: _ } => (),
        FieldDefault::Default => {
            if ty.to_string().starts_with("LoadOp <") {
                println!("{:?}", ty);
            }

            if field.name == "label" {
                start_fn = q!(start_fn).into();
            } else {
                default = q!(default).into();
            }
        }
        FieldDefault::Value { value } => {
            if !q!(#value)
                .to_string()
                .starts_with(&q!(LoadOp::).to_string())
            {
                if field.name == "label" {
                    start_fn = q!(start_fn).into();
                } else {
                    let value = match &field.ty {
                        Type::Primitive(p) if p == "u16" => {
                            syn::parse_str(&format!("{}u16", q!(#value)))?
                        }
                        Type::Primitive(p) if p == "u32" => {
                            syn::parse_str(&format!("{}u32", q!(#value)))?
                        }
                        Type::Primitive(p) if p == "u64" => {
                            syn::parse_str(&format!("{}u64", q!(#value)))?
                        }
                        _ => value.to_token_stream(),
                    };

                    default = q!(default=#value).into();
                }
            }
        }
        FieldDefault::Generic => {
            if ty.to_string().starts_with("Label <") {
                start_fn = q!(start_fn).into();
            }
        }
    };

    if let Type::BorrowedRef { type_, .. } = &field.ty
        && matches!(**type_, Type::Slice(_))
    {
        // Into conversion doesn't work on &[...]
    } else {
        into = q!(into).into();
    };

    Ok(FieldDetails {
        name,
        path: path.to_string(),
        ty,
        start_fn,
        into,
        default,
    })
}

pub fn field_return_param(field: &FieldParts) -> anyhow::Result<TokenStream> {
    let field_name = ident(&field.name);
    Ok(q!(#field_name))
}
