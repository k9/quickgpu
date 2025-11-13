use std::collections::HashMap;

use anyhow::Context;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote as q};
use syn::{
    Expr, Field, FieldValue, Fields, Ident, ImplItem, ItemStruct, Member, Path, Stmt,
    TypeParamBound, Visibility, punctuated::Punctuated, token::Comma, visit_mut::VisitMut,
};

use discover_exports::{
    EntryIndex,
    analysis::Ctx,
    resolve::{resolve_assoc_consts, resolve_impls, resolve_struct, resolve_type_alias},
};

use crate::{
    generate::{
        builder::output_builder_code,
        nested::{BuilderResolve, output_nested},
    },
    utils::is_option,
};

use super::SKIP;

pub struct BuilderField<'a> {
    pub field: &'a mut Field,
    pub default_value: Option<TokenStream>,
    pub nested_impl: bool,
}

pub struct Output {
    pub builder_comment: String,
    pub builder_code: String,
    pub nested_impl: String,
}

pub(crate) fn filter_struct(
    ctx: &Ctx,
    index: EntryIndex,
    path: &Path,
) -> Option<(EntryIndex, Ident, ItemStruct, bool)> {
    let segment = path.segments.last();
    let Some(segment) = segment else {
        return None;
    };

    let ident = segment.ident.clone();

    let Some((index, item, generate_nested_impl)) = get_index_and_item(ctx, index) else {
        return None;
    };

    if SKIP.contains(&ident.to_string().as_str()) {
        log::debug!("Skipping {} since it's in skip list", ident);
        return None;
    }

    let Fields::Named(fields) = &item.fields else {
        log::debug!("Skipping {} since it doesn't have named fields", ident);
        return None;
    };

    if fields
        .named
        .iter()
        .any(|f| !matches!(f.vis, Visibility::Public(_)))
    {
        log::debug!("Skipping {} since it has non-public fields", ident);
        return None;
    };

    Some((index, ident, item, generate_nested_impl))
}

pub(crate) fn output_struct(
    ctx: &Ctx,
    index: EntryIndex,
    path: Path,
    builders: &HashMap<String, (EntryIndex, Path)>,
) -> Output {
    let (index, ident, mut item, generate_nested_impl) = filter_struct(ctx, index, &path).unwrap();

    let builder_comment = "".to_string();

    let Fields::Named(fields) = &mut item.fields else {
        panic!("Invalid struct");
    };

    let mut fields = fields
        .named
        .iter_mut()
        .map(|field| BuilderField {
            field,
            default_value: None,
            nested_impl: false,
        })
        .collect::<Vec<_>>();

    let impls = resolve_impls(ctx, index).unwrap();
    let consts = resolve_assoc_consts(ctx, index).unwrap();
    let generics = item.generics.clone();
    let mut generics_with_constraints = item.generics.clone();

    if ["Operations", "CommandBufferDescriptor"].contains(&ident.to_string().as_str()) {
        for param in generics_with_constraints.type_params_mut() {
            param.bounds = Punctuated::new();
            let z: TypeParamBound = syn::parse_quote!(Default);
            param.bounds.push(z);
        }
    }

    for impl_item in &impls {
        apply_impl(&mut fields, &consts, impl_item);
    }

    for f in fields.iter_mut() {
        let mut resolver = BuilderResolve {
            builders,
            nested_impl: false,
        };

        resolver.visit_field_mut(f.field);
        f.nested_impl = resolver.nested_impl;

        let ident = &f.field.ident;
        let ty = &f.field.ty;

        log::debug!("    {}", q!(#ident: #ty));
    }

    let nested_fields = fields.iter().filter(|f| f.nested_impl).count();

    let builder_code =
        output_builder_code(&path, ident, fields, &generics, &generics_with_constraints);

    let nested_impl = if generate_nested_impl {
        output_nested(path, nested_fields, generics)
    } else {
        "".to_string()
    };

    Output {
        builder_comment,
        builder_code,
        nested_impl,
    }
}

pub fn apply_impl(
    fields: &mut Vec<BuilderField<'_>>,
    consts: &Vec<(Path, syn::ImplItemConst)>,
    impl_item: &syn::ItemImpl,
) {
    if let Some((_, trait_item, _)) = &impl_item.trait_
        && trait_item
            .segments
            .last()
            .is_some_and(|segment| segment.ident.to_string() == "Default")
    {
        if impl_item
            .attrs
            .iter()
            .any(|attr| q!(# [automatically_derived]).to_string() == q!(#attr).to_string())
        {
            for field in fields.iter_mut() {
                if !is_option(&field.field) {
                    field.default_value = Some(q!(default));
                }
            }
        } else if let ImplItem::Fn(func) = &impl_item.items[0]
            && let Some(Stmt::Expr(expr, _)) = func.block.stmts.last()
        {
            if let Expr::Path(expr_path) = expr {
                let const_value = consts
                    .iter()
                    .find(|(path, _)| {
                        let const_ident = path.segments.last().map(|s| s.ident.to_string());
                        let expr_ident =
                            expr_path.path.segments.last().map(|s| s.ident.to_string());

                        const_ident.is_some() && const_ident == expr_ident
                    })
                    .context("Couldn't find const")
                    .unwrap();

                let Expr::Struct(expr_struct) = &const_value.1.expr else {
                    panic!("Unsupported default");
                };

                for field in fields.iter_mut() {
                    set_field_default(field, &expr_struct.fields);
                }
            } else {
                if let Expr::Struct(expr) = expr {
                    for field in fields.iter_mut() {
                        set_field_default(field, &expr.fields);
                    }
                };
            };
        }
    }
}

fn get_index_and_item(ctx: &Ctx, index: EntryIndex) -> Option<(EntryIndex, ItemStruct, bool)> {
    let item;
    let mut index = index;
    let mut generate_nested_impl = false;
    if let Ok(as_struct) = resolve_struct(ctx, index) {
        item = as_struct;
        generate_nested_impl = true;
    } else if let Ok((as_alias, struct_index)) = resolve_type_alias(ctx, index) {
        item = as_alias;
        index = struct_index;
    } else {
        return None;
    };

    Some((index, item, generate_nested_impl))
}

fn set_field_default(field: &mut BuilderField<'_>, expr_fields: &Punctuated<FieldValue, Comma>) {
    if !is_option(&field.field) {
        let const_field = expr_fields
            .iter()
            .find(|const_field| {
                let Member::Named(const_ident) = &const_field.member else {
                    panic!("Unnamed field in default");
                };

                field.field.ident.as_ref().unwrap().to_string() == const_ident.to_string()
            })
            .unwrap();

        let default_value = const_field.expr.clone().into_token_stream();
        field.default_value = Some(q!(default = #default_value));
    }
}
