use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericParam, Ident, Path, parse_quote};

use crate::{
    generate::{
        base::{make_fn, make_new_impl, make_struct},
        setter::make_setters,
        state::{make_complete, make_empty, make_state},
        struct_entry::{BuilderField, ident_from_path},
        types::{make_field_types, make_set_types},
    },
    type_helpers::UniqueGenerics,
    utils::{FieldIdent, StructIdent, field_ident, struct_ident},
};

pub struct GeneratedBuilder {
    pub use_statement: String,
    pub code: String,
}

pub fn builder_code(
    path: &Path,
    fields: &[BuilderField],
    struct_generics: &UniqueGenerics,
    generate_nested_impl: bool,
) -> GeneratedBuilder {
    // Hack to fix lifetime bound issue
    let struct_generics = &mut struct_generics.clone();
    if let Some(b_lifetime) = struct_generics.get_mut(&parse_quote!('b)) {
        *b_lifetime = parse_quote!('b: 'a);
    }

    let label = fields
        .iter()
        .find(|f| f.field.ident.as_ref().unwrap().to_string() == "label");

    let field_types = fields.iter().map(make_field_types).collect::<TokenStream>();
    let ident = ident_from_path(path).unwrap();
    let state = make_state(fields, struct_generics);
    let empty = make_empty(fields, struct_generics);
    let complete = make_complete(fields, struct_generics);
    let set_types = make_set_types(fields, struct_generics);

    let module = struct_ident(&ident, StructIdent::BuilderMod);
    let fn_ident = struct_ident(&ident, StructIdent::Fn);

    let builder_struct = make_struct(&path, &ident, fields, &struct_generics);
    let new_impl = make_new_impl(&ident, fields, &struct_generics);
    let builder_fn = make_fn(&path, &ident, fields, &struct_generics, label);
    let setters = make_setters(path, &ident, fields, &struct_generics);

    let nested_impl = if generate_nested_impl {
        make_nested(path, &ident, fields, &struct_generics)
    } else {
        quote!()
    };

    let code = quote!(
        pub mod #module {
            pub trait Field {}
            pub trait IsEmpty {}
            pub trait IsSet<T> {
                fn get(self) -> T;
            }
            pub trait IsOptional {}

            #[allow(unused_imports)]
            use crate::builders::common::*;

            #builder_struct
            #new_impl
            #builder_fn

            #field_types
            #state
            #empty
            #complete
            #set_types
            #setters

            #nested_impl
        }
    )
    .to_string();

    let use_statement = quote!(
        pub use builders::#module::#fn_ident;
    )
    .to_string();

    GeneratedBuilder {
        use_statement,
        code,
    }
}

fn make_nested(
    path: &Path,
    ident: &Ident,
    fields: &[BuilderField],
    struct_generics: &UniqueGenerics,
) -> TokenStream {
    let builder = struct_ident(ident, StructIdent::Builder);
    let params = struct_generics.as_params();
    let args = struct_generics.as_args();

    let mut complete = struct_generics.clone();
    if fields.len() > 0 {
        complete.insert(&parse_quote!(CurrentState: Complete #args));
    }

    let state_params = complete.as_params();
    let state_args = complete.as_args();

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
) -> UniqueGenerics {
    let mut generics_with_state = struct_generics.clone();
    if fields.len() > 0 {
        generics_with_state.insert(param);
    }

    generics_with_state
}

pub fn make_build_impl(
    path: &Path,
    ident: &Ident,
    fields: &[BuilderField],
    struct_generics: &UniqueGenerics,
) -> TokenStream {
    let params = struct_generics.as_params();
    let args = struct_generics.as_args();
    let mut state_args = struct_generics.as_args_vec();
    if fields.len() > 0 {
        state_args.push(parse_quote!(Complete));
    }

    let state_args = quote!(<#(#state_args),*>);

    let struct_fields = fields
        .iter()
        .map(|f| {
            let field = field_ident(&f.field, FieldIdent::Original);

            quote!(
                #field: IsSet::get(self.#field),
            )
        })
        .collect::<TokenStream>();

    let builder = struct_ident(ident, StructIdent::Builder);

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
