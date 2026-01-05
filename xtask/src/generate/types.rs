use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

use crate::generate::struct_entry::{BuilderField, BuilderStruct, FieldIdent};

pub fn make_set_types(builder_struct: &BuilderStruct) -> TokenStream {
    let args = builder_struct.generics.as_args();

    let mut generics_with_state = builder_struct.generics.clone();
    generics_with_state.insert(&parse_quote!(CS: State #args));

    builder_struct
        .fields
        .iter()
        .map(|f| {
            let inner_types = builder_struct
                .fields
                .iter()
                .map(|f_inner| {
                    let upper = f_inner.ident(FieldIdent::UpperCamel);
                    let value = f_inner.ident(FieldIdent::Value);
                    let field_args = f_inner.generics.as_args();

                    if f_inner.field == f.field {
                        quote!(type #upper = #value #field_args;)
                    } else {
                        quote!(type #upper = CS::#upper;)
                    }
                })
                .collect::<TokenStream>();

            let set = f.ident(FieldIdent::Set);
            let params_with_state = generics_with_state.as_params();

            quote!(
                pub struct #set<CS>(CS);
                impl #params_with_state State #args for #set<CS> {
                    #inner_types
                }
            )
        })
        .collect::<TokenStream>()
}

pub fn make_field_types(f: &BuilderField) -> TokenStream {
    let empty = f.ident(FieldIdent::Empty);
    let is_empty = f.ident(FieldIdent::IsEmpty);
    let value = f.ident(FieldIdent::Value);
    let is_set = f.ident(FieldIdent::IsSet);

    let ty = &f.field.ty;
    let params = f.generics.as_params();
    let args = f.generics.as_args();

    let value_types = quote!(
        pub struct #value #params(pub #ty);
        impl #params Field for #value #args {}
        impl #params #is_set #args for #value #args {
            fn get(self) -> #ty {
                self.0
            }
        }
    );

    if let Some(default_value) = &f.default_value {
        quote!(
            pub struct #empty;
            impl Field for #empty {}

            pub trait #is_empty {}
            impl #is_empty for #empty {}

            pub trait #is_set #params {
                fn get(self) -> #ty;
            }

            impl #params #is_set #args for #empty {
                fn get(self) -> #ty {
                    #default_value
                }
            }

            #value_types
        )
    } else {
        quote!(
            pub struct #empty;
            impl Field for #empty {}

            pub trait #is_empty {}
            impl #is_empty for #empty {}

            pub trait #is_set #params {
                fn get(self) -> #ty;
            }

            #value_types
        )
    }
}
