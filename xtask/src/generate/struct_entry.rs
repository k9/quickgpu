use std::collections::HashMap;

use anyhow::Context;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote as q};
use syn::{
    Expr, Field, FieldValue, Fields, Ident, ImplItem, ItemStruct, Member, Path, PathArguments,
    Stmt, Type, TypeParamBound, Visibility, parse_quote,
    punctuated::Punctuated,
    token::Comma,
    visit_mut::{self, VisitMut},
};

use discover_exports::{
    EntryIndex,
    analysis::Ctx,
    resolve::{resolve_assoc_consts, resolve_impls, resolve_struct, resolve_type_alias},
    utils::id,
};

use super::SKIP;

pub struct BuilderField<'a> {
    pub field: &'a mut Field,
    pub default_value: Option<TokenStream>,
}

pub(crate) fn filter_struct(
    ctx: &Ctx,
    index: EntryIndex,
    path: &Path,
) -> Option<(EntryIndex, Ident, ItemStruct)> {
    let segment = path.segments.last();
    let Some(segment) = segment else {
        return None;
    };

    let ident = segment.ident.clone();

    let Some((index, item)) = get_index_and_item(ctx, index) else {
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

    Some((index, ident, item))
}

pub(crate) fn output_struct(
    ctx: &Ctx,
    index: EntryIndex,
    path: Path,
    builders: &HashMap<String, (EntryIndex, Path)>,
) -> (String, String) {
    let (index, ident, mut item) = filter_struct(ctx, index, &path).unwrap();

    let comment = "".to_string();

    let Fields::Named(fields) = &mut item.fields else {
        panic!("Invalid struct");
    };

    let mut fields = fields
        .named
        .iter_mut()
        .map(|field| BuilderField {
            field,
            default_value: None,
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

    for f in fields.iter_mut() {
        BuilderResolve {
            builders,
            is_nested: false,
        }
        .visit_field_mut(f.field);

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

    (comment, code.to_string())
}

pub(crate) fn output_nested_impl(
    ctx: &Ctx,
    index: EntryIndex,
    path: Path,
    builders: &HashMap<String, (EntryIndex, Path)>,
) -> String {
    let builder_ident = path.segments.last().unwrap().clone().ident;
    let builder_ident = id(format!("{}Builder", builder_ident.to_string()));
    let builder_path = q!(crate::builders::#builder_ident);

    let (index, ident, item) = filter_struct(ctx, index, &path).unwrap();
    let generics = item.generics.clone();

    q!(
        impl #generics Nested<#path #generics> for #path #generics {
            fn unnest(self) -> #path #generics {
                self
            }
        }

        impl #generics Nested<#path #generics> for #builder_path #generics {
            fn unnest(self) -> #path #generics {
                self.build()
            }
        }
    )
    .to_string()
}

fn get_index_and_item(ctx: &Ctx, index: EntryIndex) -> Option<(EntryIndex, ItemStruct)> {
    let item;
    let mut index = index;
    if let Ok(as_struct) = resolve_struct(ctx, index) {
        item = as_struct;
    } else if let Ok((as_alias, struct_index)) = resolve_type_alias(ctx, index) {
        item = as_alias;
        index = struct_index;
    } else {
        return None;
    };

    Some((index, item))
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

pub struct BuilderResolve<'a> {
    pub is_nested: bool,
    pub builders: &'a HashMap<String, (EntryIndex, Path)>,
}

impl<'a> VisitMut for BuilderResolve<'a> {
    fn visit_type_mut(&mut self, ty: &mut syn::Type) {
        if let Type::Path(path) = ty {
            let idents = without_args(&path.path);
            if self.builders.get(&q!(#idents).to_string()).is_some() {
                *ty = parse_quote!(impl Nested<#ty>);
                self.is_nested = true;
                return;
            }
        }

        visit_mut::visit_type_mut(self, ty);
    }
}

pub fn without_args(p: &Path) -> Path {
    let mut p = p.clone();
    for segment in p.segments.iter_mut() {
        segment.arguments = PathArguments::None;
    }

    p
}
