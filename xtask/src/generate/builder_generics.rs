use proc_macro2::TokenStream;
use quote::format_ident;
use quote::{ToTokens, quote as q};
use syn::GenericArgument;
use syn::GenericParam;
use syn::parse_quote;
use syn::visit::Visit;

use crate::{
    generate::struct_entry::BuilderField,
    type_helpers::{GatherGenerics, UniqueGenerics},
    utils::upper_camel_ident,
};

pub struct SetterImplGenerics {
    pub setter_impl_args: Vec<TokenStream>,
    pub setter_impl_params: Vec<TokenStream>,
    pub setter_where_params: Vec<TokenStream>,
}

pub fn make_setter_impl_generics(
    fields: &[BuilderField<'_>],
    selected: Option<&BuilderField<'_>>,
    struct_generics: &UniqueGenerics,
) -> SetterImplGenerics {
    let mut setter_impl_params = vec![];
    let mut setter_impl_args = vec![];
    let mut setter_where_params = vec![];

    for (i, f) in fields.iter().enumerate() {
        if let Some(selected) = selected
            && selected.field.ident.as_ref().unwrap().to_string()
                == f.field.ident.as_ref().unwrap().to_string()
        {
            let upper_camel = upper_camel_ident(&f.field);
            let ident = if f.default_value.is_some() {
                format_ident!("{}OptionalValue", upper_camel)
            } else {
                format_ident!("{}Value", upper_camel)
            };

            let ty = &f.field.ty;
            let mut gather = GatherGenerics::new(struct_generics);
            gather.visit_type(ty);

            setter_impl_params.push(q!(#ident));

            let impl_args = gather.used.as_args();
            setter_impl_args.push(q!(#ident #impl_args));

            let where_ident = format_ident!("T{}", i);
            let where_constraint = if f.default_value.is_some() {
                format_ident!("IsUnsetOptional")
            } else {
                format_ident!("IsUnset")
            };

            setter_where_params.push(q!(#where_ident: #where_constraint));
        } else {
            let constraint = if f.default_value.is_some() {
                format_ident!("IsOptional")
            } else {
                format_ident!("IsRequired")
            };

            let ident = format_ident!("T{}", i);
            setter_impl_args.push(q!(#ident));
            setter_impl_params.push(q!(#ident: #constraint));
        }
    }

    SetterImplGenerics {
        setter_impl_args,
        setter_impl_params,
        setter_where_params,
    }
}

pub struct BuildImplGenerics {
    pub build_where: Vec<GenericParam>,
    pub build_impl_params: UniqueGenerics,
    pub build_impl_args: Vec<GenericArgument>,
    pub build_fn_params: Vec<TokenStream>,
}

pub fn make_build_impl_generics(
    fields: &[BuilderField<'_>],
    struct_generics: &UniqueGenerics,
) -> BuildImplGenerics {
    let mut build_impl_params = UniqueGenerics::new(None);
    let mut build_where: Vec<GenericParam> = vec![];
    let mut build_impl_args: Vec<GenericArgument> = vec![];

    for f in fields {
        let ty = &f.field.ty;
        let upper_camel = upper_camel_ident(&f.field);

        if f.default_value.is_some() {
            let param = format_ident!("R{}", upper_camel);
            let constraint = q!(ResolveOptional<#ty>);

            build_where.push(parse_quote!(#param: #constraint));
            build_impl_params.insert(&parse_quote!(#param));
            build_impl_args.push(parse_quote!(#param));
        } else {
            let arg = format_ident!("{}Value", upper_camel);

            let mut gather = GatherGenerics::new(&struct_generics);
            let ty = &f.field.ty;
            gather.visit_type(ty);

            let generic_args = gather.used.as_args();
            build_impl_args.push(parse_quote!(#arg #generic_args));
        };
    }

    let mut gather_impl_args = GatherGenerics::new(&struct_generics);
    for arg in build_impl_args.iter() {
        gather_impl_args.visit_generic_argument(arg);
    }

    for param in gather_impl_args.used.as_params_vec() {
        build_impl_params.insert(&param);
    }

    let mut build_fn_params = vec![];
    let mut gather_where_params = GatherGenerics::new(&struct_generics);
    for arg in build_where.iter() {
        gather_where_params.visit_generic_param(arg);
    }

    for param in gather_where_params.used.as_params_vec() {
        // Only add param to fn if it's not added to impl
        if gather_impl_args.used.get(&param).is_none() {
            build_fn_params.push(param.into_token_stream());
        }
    }

    BuildImplGenerics {
        build_where,
        build_impl_args,
        build_impl_params,
        build_fn_params,
    }
}
