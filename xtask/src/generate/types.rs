use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

use crate::{
    generate::struct_entry::BuilderField,
    type_helpers::UniqueGenerics,
    utils::{FieldIdent, field_ident},
};

pub fn make_set_types(fields: &[BuilderField], struct_generics: &UniqueGenerics) -> TokenStream {
    let args = struct_generics.as_args();

    let mut generics_with_state = struct_generics.clone();
    generics_with_state.insert(&parse_quote!(CurrentState: State #args));

    fields
        .iter()
        .map(|f| {
            let inner_types = fields
                .iter()
                .map(|f_inner| {
                    let upper = field_ident(&f_inner.field, FieldIdent::UpperCamel);
                    let value = field_ident(&f_inner.field, FieldIdent::Value);
                    let field_args = f_inner.generics.as_args();

                    if f_inner.field == f.field {
                        quote!(type #upper = #value #field_args;)
                    } else {
                        quote!(type #upper = CurrentState::#upper;)
                    }
                })
                .collect::<TokenStream>();

            let set = field_ident(&f.field, FieldIdent::Set);

            let params_with_state = generics_with_state.as_params();

            quote!(
                pub struct #set<CurrentState>(CurrentState);
                impl #params_with_state State #args for #set<CurrentState> {
                    #inner_types
                }
            )
        })
        .collect::<TokenStream>()
}

pub fn make_field_types(f: &BuilderField) -> TokenStream {
    let empty = field_ident(&f.field, FieldIdent::Empty);
    let optional = field_ident(&f.field, FieldIdent::Optional);
    let value = field_ident(&f.field, FieldIdent::Value);
    let ty = &f.field.ty;
    let params = f.generics.as_params();
    let args = f.generics.as_args();

    let value_types = quote!(
        pub struct #value #params(pub #ty);
        impl #params Field for #value #args {}
        impl #params IsSet<#ty> for #value #args {
            fn get(self) -> #ty {
                self.0
            }
        }
    );

    if let Some(default_value) = &f.default_value {
        quote!(
            pub struct #optional;
            impl Field for #optional  {}
            impl IsOptional for #optional  {}
            impl #params IsSet<#ty> for #optional  {
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
            impl IsEmpty for #empty {}

            #value_types
        )
    }
}
