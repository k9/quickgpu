use crate::{
    generate::{builder::add_state_param, struct_entry::BuilderField},
    type_helpers::UniqueGenerics,
    utils::{FieldIdent, field_ident},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

pub fn make_state(fields: &[BuilderField], struct_generics: &UniqueGenerics) -> TokenStream {
    let params = struct_generics.as_params();
    let state_fields = fields
        .iter()
        .map(|f| {
            let upper = field_ident(&f.field, FieldIdent::UpperCamel);
            quote!(
                type #upper: Field;
            )
        })
        .collect::<TokenStream>();

    quote!(
        pub trait State #params {
            #state_fields
        }
    )
}

pub fn make_empty(fields: &[BuilderField], struct_generics: &UniqueGenerics) -> TokenStream {
    let params = struct_generics.as_params();
    let args = struct_generics.as_args();

    let empty_fields = fields
        .iter()
        .map(|f| {
            let upper = field_ident(&f.field, FieldIdent::UpperCamel);
            if f.default_value.is_some() {
                let optional = field_ident(&f.field, FieldIdent::Optional);
                quote!(
                    type #upper = #optional;
                )
            } else {
                let empty = field_ident(&f.field, FieldIdent::Empty);
                quote!(
                    type #upper = #empty;
                )
            }
        })
        .collect::<TokenStream>();

    quote!(
        pub struct Empty;
        impl #params State #args for Empty {
            #empty_fields
        }
    )
}

pub fn make_complete(fields: &[BuilderField], struct_generics: &UniqueGenerics) -> TokenStream {
    if fields.len() == 0 {
        return quote!();
    }

    let params = struct_generics.as_params();
    let args = struct_generics.as_args();
    let generics_with_state = add_state_param(
        fields,
        struct_generics,
        &parse_quote!(CurrentState: State #args),
    );

    let state_params = generics_with_state.as_params();

    let mut details_args = struct_generics.as_args_vec();

    for f in fields {
        let upper = field_ident(&f.field, FieldIdent::UpperCamel);
        let ty = &f.field.ty;

        details_args.push(parse_quote!(
            #upper: IsSet<#ty>
        ));
    }

    let mut impl_bounds: Vec<TokenStream> = vec![];
    for f in fields {
        let upper = field_ident(&f.field, FieldIdent::UpperCamel);
        let ty = &f.field.ty;

        impl_bounds.push(parse_quote!(
            CurrentState::#upper: IsSet<#ty>
        ));
    }

    quote!(
        pub trait Complete #params: State<#(#details_args),*> {}

        impl #state_params Complete #args for CurrentState
        where #(#impl_bounds),*
        {
        }
    )
}
