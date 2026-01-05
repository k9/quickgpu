use crate::{
    generate::struct_entry::{BuilderField, BuilderStruct, FieldIdent},
    type_helpers::UniqueGenerics,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

pub fn make_state(builder_struct: &BuilderStruct) -> TokenStream {
    let params = builder_struct.generics.as_params();
    let state_fields = builder_struct
        .fields
        .iter()
        .map(|f| {
            let upper = f.ident(FieldIdent::UpperCamel);
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

pub fn make_empty(builder_struct: &BuilderStruct) -> TokenStream {
    let params = builder_struct.generics.as_params();
    let args = builder_struct.generics.as_args();

    let empty_fields = builder_struct
        .fields
        .iter()
        .map(|f| {
            let upper = f.ident(FieldIdent::UpperCamel);
            let empty = f.ident(FieldIdent::Empty);
            quote!(
                type #upper = #empty;
            )
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
    let params = struct_generics.as_params();
    let args = struct_generics.as_args();

    let complete_fields = fields
        .iter()
        .map(|f_inner| {
            let upper = f_inner.ident(FieldIdent::UpperCamel);
            let value = f_inner.ident(FieldIdent::Value);
            let field_args = f_inner.generics.as_args();
            quote!(type #upper = #value #field_args;)
        })
        .collect::<TokenStream>();

    let mut generics_with_state = struct_generics.clone();
    generics_with_state.insert(&parse_quote!(CS: State #args));

    quote!(
        pub struct Complete;
        impl #params State #args for Complete {
            #complete_fields
        }
    )
}
