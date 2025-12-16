use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path, parse_quote};

use crate::{
    generate::{
        builder::add_state_param,
        docs::{builder_docs, builder_fn_docs},
        struct_entry::BuilderField,
    },
    type_helpers::UniqueGenerics,
    utils::{FieldIdent, StructIdent, field_ident, struct_ident},
};

pub fn make_struct(
    path: &Path,
    ident: &Ident,
    fields: &[BuilderField],
    struct_generics: &UniqueGenerics,
) -> TokenStream {
    let args = struct_generics.as_args();
    let generics_with_state =
        add_state_param(fields, struct_generics, &parse_quote!(CS: State #args));
    let state_params = generics_with_state.as_params();

    let struct_fields = fields
        .iter()
        .map(|f| {
            let field = field_ident(&f.field, FieldIdent::Original);
            let upper = field_ident(&f.field, FieldIdent::UpperCamel);

            quote!(
                #field: CS::#upper,
            )
        })
        .collect::<TokenStream>();

    let builder = struct_ident(ident, StructIdent::Builder);
    let docs = builder_docs(path, fields);
    quote!(
        #[doc=#docs]
        pub struct #builder #state_params {
            #struct_fields
        }
    )
}

pub fn make_new_impl(
    ident: &Ident,
    fields: &[BuilderField],
    struct_generics: &UniqueGenerics,
) -> TokenStream {
    let params = struct_generics.as_params();
    let generics_with_state = add_state_param(fields, struct_generics, &parse_quote!(Empty));
    let state_args = generics_with_state.as_args();

    let new_fields = fields
        .iter()
        .map(|f| {
            let field = field_ident(&f.field, FieldIdent::Original);

            if f.default_value.is_some() {
                let optional = field_ident(&f.field, FieldIdent::Optional);
                quote!(#field: #optional,)
            } else {
                let empty = field_ident(&f.field, FieldIdent::Empty);
                quote!(#field: #empty,)
            }
        })
        .collect::<TokenStream>();

    let builder = struct_ident(ident, StructIdent::Builder);

    quote!(
        impl #params #builder #state_args {
            pub fn new() -> #builder #state_args {
                #builder {
                    #new_fields
                }
            }
        }
    )
}

pub fn make_fn(
    path: &Path,
    ident: &Ident,
    fields: &[BuilderField],
    struct_generics: &UniqueGenerics,
    label: Option<&BuilderField>,
) -> TokenStream {
    let params = struct_generics.as_params();

    let mut fn_params = quote!();
    let mut label_call = quote!();
    let mut return_state = quote!(Empty);

    if let Some(label) = label {
        let ty = &label.field.ty;
        fn_params = quote!(label: #ty);
        label_call = quote!(.label(label));
        return_state = quote!(SetLabel<Empty>);
    }

    let mut state_args = struct_generics.as_args_vec();
    if fields.len() > 0 {
        state_args.push(parse_quote!(#return_state));
    }

    let state_args = quote!(<#(#state_args),*>);

    let fn_ident = struct_ident(ident, StructIdent::Fn);
    let builder = struct_ident(ident, StructIdent::Builder);
    let docs = builder_fn_docs(path, fields);

    quote!(
        #[doc=#docs]
        pub fn #fn_ident #params(#fn_params) -> #builder #state_args {
            #builder::new() #label_call
        }
    )
}
