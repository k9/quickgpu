use anyhow::bail;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote as q};
use syn::{Expr, Field, Fields, ImplItem, Member, Stmt, Type, Visibility, spanned::Spanned};

use discover_exports::{
    EntryIndex,
    crate_graph::{full_path, get_struct_const, node_ident},
    utils::id,
};

use super::{AResult, SKIP};

pub struct BuilderField<'a> {
    pub field: &'a Field,
    pub default_value: Option<TokenStream>,
}

pub(crate) fn output_struct(
    analysis: &CrateAnalysis,
    index: EntryIndex,
) -> AResult<Option<(String, String)>> {
    let comment = "".to_string();
    let Some(entry) = get_struct(analysis, index)? else {
        return Ok(None);
    };

    let ident = node_ident(analysis, index)?;

    if SKIP.contains(&ident.to_string().as_str()) {
        log::debug!("Skipping {} since it's in skip list", ident);

        return Ok(None);
    }

    let Fields::Named(fields) = &entry.fields else {
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

    for impl_item in &entry.impls {
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
                    let const_value =
                        get_struct_const(entry, &expr_path.path.segments.last().unwrap().ident)
                            .unwrap();

                    let Expr::Struct(expr_struct) = &const_value.expr else {
                        bail!("Unsupported default");
                    };

                    for field in fields.iter_mut() {
                        if !is_option(field) {
                            let const_field = expr_struct
                                .fields
                                .iter()
                                .find(|const_field| {
                                    let Member::Named(const_ident) = &const_field.member else {
                                        panic!("Unnamed field in default");
                                    };

                                    field.field.ident.as_ref().unwrap().to_string()
                                        == const_ident.to_string()
                                })
                                .unwrap();

                            let default_value = const_field.expr.clone().into_token_stream();
                            field.default_value = Some(q!(default = #default_value));
                        }
                    }
                } else {
                    println!("{:?} _CUSTOM_", expr.span().source_text());
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

    let generics = &entry.generics;
    let path = full_path(analysis, index)?;

    let code = q! {
        #[bon::builder(
            //builder_type(doc __builder_type_docs__),
            state_mod(vis="pub(crate)"),
            finish_fn=build,
        )]
        pub fn #fn_ident #generics(
            #(#fn_params),*
        ) -> #(#path)::* #generics {
            #(#path)::* {
                #(#struct_values),*
            }
        }
    };

    Ok(Some((comment, code.to_string())))
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
