use proc_macro2::TokenStream;
use quote::{ToTokens, quote as q};
use rustdoc_types::{GenericArg, GenericArgs, GenericParamDef, GenericParamDefKind, Type};
use syn::{Expr, Ident, Lifetime, parse_str};

use crate::{
    AResult,
    analyze::core::StructParts,
    type_alias_helpers::TypeAliasMap,
    utils::{final_path, ident},
};

pub fn generic_params(struct_item: &StructParts, extra_types: &[Ident]) -> AResult<TokenStream> {
    let mut struct_generics = vec![];

    let generics = struct_item
        .type_alias_map
        .map_generics(&struct_item.generics);

    for GenericParamDef { name, kind } in &generics.params {
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

    for extra_type in extra_types {
        struct_generics.push(extra_type.to_token_stream());
    }

    let struct_generics = if struct_generics.is_empty() {
        q!()
    } else {
        q!(<#(#struct_generics),*>)
    };

    Ok(struct_generics)
}

pub fn generic_args(
    args: Option<Box<GenericArgs>>,
    type_alias_map: &TypeAliasMap,
) -> AResult<TokenStream> {
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
                    let lifetime = type_alias_map.map_lifetime(&lifetime);
                    tokens.push(parse_str::<Lifetime>(&lifetime)?.to_token_stream());
                }
                GenericArg::Type(type_) => {
                    tokens.push(type_tokens(&type_, type_alias_map)?);
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

pub fn type_tokens(field_type: &Type, type_alias_map: &TypeAliasMap) -> AResult<TokenStream> {
    match field_type {
        Type::ResolvedPath(path) => {
            let args = generic_args(path.args.clone(), type_alias_map)?;
            let path = final_path(&path.path)?;

            let path = parse_str::<Expr>(&path)?;
            Ok(q!(#path #args))
        }
        Type::Primitive(p) => Ok(ident(p).to_token_stream()),
        Type::Generic(g) => {
            let type_ = type_alias_map.map_generic(g);
            if let Type::Generic(generic) = type_ {
                Ok(ident(&generic).to_token_stream())
            } else {
                type_tokens(&type_, type_alias_map)
            }
        }
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

            tokens.push(type_tokens(type_, type_alias_map)?);

            Ok(q!(#(#tokens)*))
        }
        Type::Slice(ty) => {
            let inner = type_tokens(ty, type_alias_map)?;
            Ok(q!([#inner]))
        }
        Type::Tuple(tuple) => {
            let mut tokens = vec![];
            for item in tuple {
                tokens.push(type_tokens(item, type_alias_map)?);
            }

            Ok(q!(
                (#(#tokens),*)
            ))
        }
        Type::Array { type_, len } => {
            let tokens = type_tokens(type_, type_alias_map)?;
            let len: usize = len.parse()?;

            Ok(q!(
                [#tokens; #len]
            ))
        }
        ty => panic!("Failed to handle type {:?}", ty),
    }
}
