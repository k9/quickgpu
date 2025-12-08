use proc_macro2::TokenStream;
use quote::quote;
use syn::visit::Visit;

use crate::{
    generate::struct_entry::BuilderField,
    type_helpers::{GatherGenerics, UniqueGenerics},
    utils::{FieldIdent, field_ident},
};

pub fn make_typestate(
    fields: &[BuilderField<'_>],
    struct_generics: &UniqueGenerics,
    generics_with_state: &UniqueGenerics,
) -> TokenStream {
    let struct_params = struct_generics.as_params();
    let struct_args = struct_generics.as_args();
    let params_with_state = generics_with_state.as_params();
    let args_with_state = generics_with_state.as_args();

    let state_types = fields
        .iter()
        .map(|f| {
            let upper_camel = field_ident(&f.field, FieldIdent::UpperCamel);
            quote!(type #upper_camel: Field;)
        })
        .collect::<TokenStream>();

    let empty_types = fields
        .iter()
        .map(|f| {
            let upper_camel = field_ident(&f.field, FieldIdent::UpperCamel);
            if f.default_value.is_some() {
                let optional = field_ident(&f.field, FieldIdent::Optional);
                quote!(type #upper_camel = #optional;)
            } else {
                let unset = field_ident(&f.field, FieldIdent::Empty);
                quote!(type #upper_camel = #unset;)
            }
        })
        .collect::<TokenStream>();

    let complete_types = fields
        .iter()
        .map(|f| {
            let ty = &f.field.ty;
            let mut gather = GatherGenerics::new(generics_with_state);
            gather.visit_type(ty);

            let impl_params = gather.used.as_params();

            let upper_camel = field_ident(&f.field, FieldIdent::UpperCamel);
            let value = field_ident(&f.field, FieldIdent::Value);
            quote!(type #upper_camel = #value #impl_params;)
        })
        .collect::<TokenStream>();

    let setter_types = fields
        .iter()
        .map(|f| {
            let inner_types = fields
                .iter()
                .map(|f_inner| {
                    let ty = &f.field.ty;
                    let mut gather = GatherGenerics::new(generics_with_state);
                    gather.visit_type(ty);

                    let impl_args = gather.used.as_args();

                    let upper_camel = field_ident(&f_inner.field, FieldIdent::UpperCamel);
                    let value = field_ident(&f_inner.field, FieldIdent::Value);

                    if f.field == f_inner.field {
                        quote!(type #upper_camel = #value #impl_args;)
                    } else {
                        quote!(type #upper_camel = S::#upper_camel;)
                    }
                })
                .collect::<TokenStream>();

            let set = field_ident(&f.field, FieldIdent::Set);

            quote!(
                pub struct #set<S>(S);
                impl #params_with_state State #struct_args for #set<S> {
                    #inner_types
                }
            )
        })
        .collect::<TokenStream>();

    quote!(
        pub trait State #struct_params {
            #state_types
        }

        pub struct Empty;
        impl #struct_params State #struct_args for Empty {
            #empty_types
        }

        pub struct Complete;
        impl #struct_params State #struct_args for Complete {
            #complete_types
        }

        #setter_types
    )
}
