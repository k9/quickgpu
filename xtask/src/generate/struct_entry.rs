use std::collections::HashMap;

use anyhow::Context;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote as q};
use syn::{
    Expr, Field, FieldValue, Fields, Ident, ImplItem, ItemStruct, Member, Path, Stmt, Type,
    TypeParamBound, Visibility, punctuated::Punctuated, token::Comma, visit_mut::VisitMut,
};

use discover_exports::{
    EntryIndex,
    analysis::Ctx,
    resolve::{
        resolve_assoc_consts, resolve_impls, resolve_path, resolve_struct, resolve_type_alias,
    },
};

use crate::{
    generate::{
        builder::{GeneratedBuilder, builder_code},
        nested::BuilderResolve,
    },
    utils::{OptionType, option_type},
};

use super::SKIP;

pub struct BuilderField<'a> {
    pub field: &'a mut Field,
    pub default_value: Option<TokenStream>,
    pub nested_ty: bool,
}

pub struct Output {
    pub comment: String,
    pub use_statement: String,
    pub code: String,
}

pub(crate) fn filter_struct(
    ctx: &Ctx,
    index: EntryIndex,
    path: &Path,
) -> Option<(EntryIndex, ItemStruct, bool)> {
    let Some(ident) = ident_from_path(path) else {
        return None;
    };

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

    Some((index, item, generate_nested_impl))
}

pub fn ident_from_path(path: &Path) -> Option<Ident> {
    path.segments.last().map(|s| s.ident.clone())
}

pub(crate) fn output_struct(
    ctx: &Ctx,
    index: EntryIndex,
    path: Path,
    builders: &HashMap<String, (EntryIndex, Path)>,
) -> Output {
    let (index, mut item, generate_nested_impl) = filter_struct(ctx, index, &path).unwrap();

    let Fields::Named(fields) = &mut item.fields else {
        panic!("Invalid struct");
    };

    let mut fields = fields
        .named
        .iter_mut()
        .map(|field| BuilderField {
            field,
            default_value: None,
            nested_ty: false,
        })
        .collect::<Vec<_>>();

    let impls = resolve_impls(ctx, index).unwrap();
    let consts = resolve_assoc_consts(ctx, index).unwrap();
    let mut generics = item.generics.clone();

    let ident = ident_from_path(&path).unwrap();
    if ["Operations", "CommandBufferDescriptor"].contains(&ident.to_string().as_str()) {
        for param in generics.type_params_mut() {
            param.bounds = Punctuated::new();
            let z: TypeParamBound = syn::parse_quote!(Default);
            param.bounds.push(z);
        }
    }

    for impl_item in &impls {
        apply_impl(&mut fields, &consts, impl_item);
    }

    for field in fields.iter_mut() {
        let mut has_default_impl = false;

        if let Type::Path(path) = &field.field.ty
            && let Ok(field_ty_index) = resolve_path(ctx, ctx.crate_root, &path.path)
        {
            let impls = resolve_impls(ctx, field_ty_index).unwrap();
            if impls
                .iter()
                .any(|block| get_default_trait_item(block).is_some())
            {
                has_default_impl = true;
            }
        }

        let is_option = option_type(&field.field) != OptionType::None;

        if has_default_impl || is_option {
            field.default_value = Some(q!(Default::default()));
        }
    }

    for f in fields.iter_mut() {
        let mut resolver = BuilderResolve {
            builders,
            nested_ty: false,
        };

        resolver.visit_field_mut(f.field);
        f.nested_ty = resolver.nested_ty;

        let ident = &f.field.ident;
        let ty = &f.field.ty;

        log::debug!("    {}", q!(#ident: #ty));
    }

    let comment = "".to_string();
    let GeneratedBuilder {
        use_statement,
        code,
    } = builder_code(&path, &fields, &generics, generate_nested_impl);

    Output {
        comment,
        use_statement,
        code,
    }
}

pub fn apply_impl(
    fields: &mut Vec<BuilderField<'_>>,
    consts: &Vec<(Path, syn::ImplItemConst)>,
    impl_item: &syn::ItemImpl,
) {
    if get_default_trait_item(impl_item).is_some() {
        if impl_item
            .attrs
            .iter()
            .any(|attr| q!(# [automatically_derived]).to_string() == q!(#attr).to_string())
        {
            for field in fields.iter_mut() {
                field.default_value = Some(q!(Default::default()));
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
            }
        }
    }
}

fn get_default_trait_item(impl_item: &syn::ItemImpl) -> Option<&Path> {
    if let Some((_, trait_item, _)) = &impl_item.trait_
        && trait_item
            .segments
            .last()
            .is_some_and(|segment| segment.ident.to_string() == "Default")
    {
        Some(trait_item)
    } else {
        None
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
    field.default_value = Some(q!(#default_value));
}
