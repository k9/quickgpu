use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericParam, parse_quote};

use crate::{
    generate::{
        base::{make_fn, make_new_impl, make_struct},
        setter::make_setters,
        state::{make_empty, make_state},
        struct_entry::{BuilderField, BuilderStruct, FieldIdent, StructIdent},
        tests::builder_tests,
        types::{make_field_types, make_set_types},
    },
    type_helpers::UniqueGenerics,
};

pub struct GeneratedBuilder {
    pub name: String,
    pub use_statement: String,
    pub builder_mod: String,
    pub code: String,
}

pub fn builder_code(builder_struct: &BuilderStruct) -> GeneratedBuilder {
    let label = builder_struct
        .fields
        .iter()
        .find(|f| f.field.ident.as_ref().unwrap().to_string() == "label");

    let field_types = builder_struct
        .fields
        .iter()
        .map(make_field_types)
        .collect::<TokenStream>();

    let state = make_state(&builder_struct);
    let empty = make_empty(&builder_struct);
    let set_types = make_set_types(&builder_struct);

    let module = builder_struct.ident(StructIdent::BuilderMod);
    let fn_ident = builder_struct.ident(StructIdent::Fn);

    let struct_definition = make_struct(&builder_struct);
    let new_impl = make_new_impl(builder_struct);
    let builder_fn = make_fn(builder_struct, label);
    let setters = make_setters(builder_struct);

    let nested_impl = if builder_struct.generate_nested_impl {
        make_nested(builder_struct)
    } else {
        quote!()
    };

    let tests = builder_tests(&builder_struct, label.is_some());

    let code = quote!(
        pub use super::super::Nested;
        pub use std::{borrow::Cow, num::NonZeroU32, ops::Range};

        pub trait Field {}
        pub trait IsOptional {}

        #struct_definition
        #new_impl
        #builder_fn

        #field_types
        #state
        #empty
        #set_types
        #setters

        #nested_impl

        #tests
    )
    .to_string();

    let use_statement = quote!(
        #[doc(inline)]
        pub use builders::#module::#fn_ident;
    )
    .to_string();

    let builder_mod = quote!(
        pub mod #module;
    )
    .to_string();

    GeneratedBuilder {
        name: module.to_string(),
        use_statement,
        builder_mod,
        code,
    }
}

fn make_nested(builder_struct: &BuilderStruct) -> TokenStream {
    let builder = builder_struct.ident(StructIdent::Builder);
    let params = builder_struct.generics.as_params();
    let args = builder_struct.generics.as_args();

    let generics_with_state = add_state_param(
        &builder_struct.fields,
        &builder_struct.generics,
        &parse_quote!(CS:Complete #args),
        false,
    );

    let state_params = generics_with_state.as_params();
    let state_args = generics_with_state.as_args();
    let path = &builder_struct.path;

    quote!(
        impl #params Nested<#path #args> for #path #args {
            fn unnest(self) -> #path #args {
                self
            }
        }

        impl #state_params Nested<#path #args> for #builder #state_args {
            fn unnest(self) -> #path #args {
                self.build()
            }
        }
    )
}

pub fn add_state_param(
    fields: &[BuilderField<'_>],
    struct_generics: &UniqueGenerics,
    param: &GenericParam,
    add_with_zero_fields: bool,
) -> UniqueGenerics {
    let mut generics_with_state = struct_generics.clone();
    if add_with_zero_fields || fields.len() > 0 {
        generics_with_state.insert(param);
    }

    generics_with_state
}

pub fn make_build_impl(builder_struct: &BuilderStruct) -> TokenStream {
    let params = builder_struct.generics.as_params();
    let args = builder_struct.generics.as_args();
    let mut state_args = builder_struct.generics.as_args_vec();
    if builder_struct.fields.len() > 0 {
        state_args.push(parse_quote!(Complete));
    }

    let state_args = quote!(<#(#state_args),*>);

    let struct_fields = builder_struct
        .fields
        .iter()
        .map(|f| {
            let field = f.ident(FieldIdent::Original);
            let is_set = f.ident(FieldIdent::IsSet);

            quote!(
                #field: #is_set::get(self.#field),
            )
        })
        .collect::<TokenStream>();

    let builder = builder_struct.ident(StructIdent::Builder);
    let path = &builder_struct.path;

    quote!(
        impl #params #builder #state_args {
            pub fn build(self) -> #path #args {
                #path {
                    #struct_fields
                }
            }
        }
    )
}
