use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

use crate::generate::{
    builder::add_state_param,
    docs::{builder_docs, builder_fn_docs},
    struct_entry::{BuilderField, BuilderStruct, FieldIdent, StructIdent},
};

pub fn make_struct(builder_struct: &BuilderStruct) -> TokenStream {
    let args = builder_struct.generics.as_args();
    let generics_with_state = add_state_param(
        &builder_struct.fields,
        &builder_struct.generics,
        &parse_quote!(CS: State #args),
        false,
    );

    let state_params = generics_with_state.as_params();

    let struct_fields = builder_struct
        .fields
        .iter()
        .map(|f| {
            let field = f.ident(FieldIdent::Original);
            let upper = f.ident(FieldIdent::UpperCamel);

            quote!(
                #field: CS::#upper,
            )
        })
        .collect::<TokenStream>();

    let builder = builder_struct.ident(StructIdent::Builder);
    let docs = builder_docs(builder_struct);
    quote!(
        #[doc=#docs]
        pub struct #builder #state_params {
            #struct_fields
        }
    )
}

pub fn make_new_impl(builder_struct: &BuilderStruct) -> TokenStream {
    let params = builder_struct.generics.as_params();
    let generics_with_state = add_state_param(
        &builder_struct.fields,
        &builder_struct.generics,
        &parse_quote!(Empty),
        false,
    );

    let state_args = generics_with_state.as_args();

    let new_fields = builder_struct
        .fields
        .iter()
        .map(|f| {
            let field = f.ident(FieldIdent::Original);

            let empty = f.ident(FieldIdent::Empty);
            quote!(#field: #empty,)
        })
        .collect::<TokenStream>();

    let builder = builder_struct.ident(StructIdent::Builder);

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

pub fn make_fn(builder_struct: &BuilderStruct, label: Option<&BuilderField>) -> TokenStream {
    let params = builder_struct.generics.as_params();

    let mut fn_params = quote!();
    let mut label_call = quote!();
    let mut return_state = quote!(Empty);

    if let Some(label) = label {
        let ty = &label.field.ty;
        fn_params = quote!(label: #ty);
        label_call = quote!(.label(label));
        return_state = quote!(SetLabel<Empty>);
    }

    let mut state_args = builder_struct.generics.as_args_vec();
    if builder_struct.fields.len() > 0 {
        state_args.push(parse_quote!(#return_state));
    }

    let state_args = quote!(<#(#state_args),*>);

    let fn_ident = builder_struct.ident(StructIdent::Fn);
    let builder = builder_struct.ident(StructIdent::Builder);
    let docs = builder_fn_docs(builder_struct);

    quote!(
        #[doc=#docs]
        pub fn #fn_ident #params(#fn_params) -> #builder #state_args {
            #builder::new() #label_call
        }
    )
}
