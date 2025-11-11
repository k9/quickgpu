use anyhow::{Context, bail};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote as q};
use syn::{
    Expr, Field, FieldValue, Fields, ImplItem, Member, Stmt, Type, TypeParamBound, Visibility,
    punctuated::Punctuated, token::Comma,
};

use discover_exports::{
    EntryIndex,
    analysis::Ctx,
    resolve::{
        PathType, full_path, resolve_assoc_consts, resolve_impls, resolve_struct,
        resolve_type_alias,
    },
    utils::id,
};

use super::{AResult, SKIP};

pub struct BuilderField<'a> {
    pub field: &'a Field,
    pub default_value: Option<TokenStream>,
}

pub(crate) fn output_struct(ctx: &Ctx, index: EntryIndex) -> AResult<Option<(String, String)>> {
    let comment = "".to_string();

    let Ok(path) = full_path(ctx, index, PathType::TopLevelPublicOnly) else {
        return Ok(None);
    };

    let segment = path.segments.last();
    let Some(segment) = segment else {
        return Ok(None);
    };
    let ident = segment.ident.clone();

    let item;
    let mut index = index;
    if let Ok(as_struct) = resolve_struct(ctx, index) {
        item = as_struct;
    } else if let Ok((as_alias, struct_index)) = resolve_type_alias(ctx, index) {
        item = as_alias;
        index = struct_index;
    } else {
        return Ok(None);
    };

    if SKIP.contains(&ident.to_string().as_str()) {
        log::debug!("Skipping {} since it's in skip list", ident);
        return Ok(None);
    }

    let Fields::Named(fields) = &item.fields else {
        log::debug!("Skipping {} since it doesn't have named fields", ident);
        return Ok(None);
    };

    if fields
        .named
        .iter()
        .any(|f| !matches!(f.vis, Visibility::Public(_)))
    {
        log::debug!("Skipping {} since it has non-public fields", ident);
        return Ok(None);
    };

    let mut fields = fields
        .named
        .iter()
        .map(|field| BuilderField {
            field,
            default_value: None,
        })
        .collect::<Vec<_>>();

    let impls = resolve_impls(ctx, index)?;
    let consts = resolve_assoc_consts(ctx, index)?;
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
                    if !is_option(field) {
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
                        .context("Couldn't find const")?;

                    let Expr::Struct(expr_struct) = &const_value.1.expr else {
                        bail!("Unsupported default");
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

    for f in &fields {
        let ident = &f.field.ident;
        let ty = &f.field.ty;
        log::debug!("    {}", q!(#ident: #ty));
    }

    let fn_ident = id(ident.to_string().to_case(Case::Snake).as_str());

    let fn_params = fields.iter().map(|f| {
        let ident = &f.field.ident;
        let ty = &f.field.ty;
        let builder_attr = match &f.default_value {
            Some(value) => q!(#[builder(#value)]),
            None => q!(),
        };

        q!(
            #builder_attr
            #ident: #ty
        )
    });

    let struct_values = fields.iter().map(|f| {
        let ident = &f.field.ident;
        q!(#ident)
    });

    let code = q! {
        #[bon::builder(
            //builder_type(doc __builder_type_docs__),
            state_mod(vis="pub(crate)"),
            finish_fn=build,
        )]
        pub fn #fn_ident #generics_with_constraints(
            #(#fn_params),*
        ) -> #path #generics {
            #path {
                #(#struct_values),*
            }
        }
    };

    Ok(Some((comment, code.to_string())))
}

fn set_field_default(field: &mut BuilderField<'_>, expr_fields: &Punctuated<FieldValue, Comma>) {
    if !is_option(field) {
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

fn is_option(field: &BuilderField<'_>) -> bool {
    if let Type::Path(path) = &field.field.ty
        && path.path.segments.last().map(|s| s.ident.to_string()) == Some("Option".to_string())
    {
        true
    } else {
        false
    }
}
