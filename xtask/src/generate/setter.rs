use proc_macro2::TokenStream;
use quote::quote;
use syn::{AngleBracketedGenericArguments, Ident, Path, Type, parse_quote};

use crate::{
    generate::{builder::add_state_param, docs::setter_docs, struct_entry::BuilderField},
    type_helpers::UniqueGenerics,
    utils::{
        FieldIdent, OptionType, StructIdent, field_ident, option_argument, option_type,
        struct_ident,
    },
};

pub fn make_setters(
    path: &Path,
    ident: &Ident,
    fields: &[BuilderField],
    struct_generics: &UniqueGenerics,
) -> TokenStream {
    let setter_fns = fields
        .iter()
        .map(|f| make_setter(path, ident, f, fields, struct_generics))
        .collect::<TokenStream>();

    let build_fn = make_build(path, ident, fields, struct_generics);

    quote!(
        #setter_fns
        #build_fn
    )
}

fn make_setter(
    path: &Path,
    ident: &Ident,
    f: &BuilderField,
    fields: &[BuilderField],
    struct_generics: &UniqueGenerics,
) -> TokenStream {
    let set = field_ident(&f.field, FieldIdent::Set);

    let ty = &f.field.ty;
    let option_arg = if option_type(&f.field) == OptionType::Option
        && let Some(arg) = option_argument(&mut ty.clone())
    {
        Some(arg.clone())
    } else {
        None
    };

    let mut set_args = struct_generics.as_args_vec();
    set_args.push(parse_quote!(#set <CS>));
    let set_args = quote!(<#(#set_args),*>);

    let mut code = make_setter_fn(
        path,
        ident,
        f,
        fields,
        struct_generics,
        &option_arg,
        &ty,
        &set_args,
        false,
    );

    if option_arg.is_some() {
        let maybe_code = make_setter_fn(
            path,
            ident,
            f,
            fields,
            struct_generics,
            &option_arg,
            &ty,
            &set_args,
            true,
        );

        code = quote!(
            #code
            #maybe_code
        );
    }

    code
}

fn make_setter_fn(
    path: &Path,
    ident: &Ident,
    f: &BuilderField,
    fields: &[BuilderField],
    struct_generics: &UniqueGenerics,
    option_arg: &Option<syn::GenericArgument>,
    ty: &Type,
    set_args: &TokenStream,
    is_maybe: bool,
) -> TokenStream {
    let ty = if let Some(arg) = option_arg
        && !is_maybe
    {
        quote!(#arg)
    } else {
        quote!(#ty)
    };

    let ty = if f.nested_ty {
        quote!(impl Nested<#ty>)
    } else {
        quote!(#ty)
    };

    let fn_ident = if is_maybe {
        field_ident(&f.field, FieldIdent::SetterMaybeFn)
    } else {
        field_ident(&f.field, FieldIdent::SetterFn)
    };

    let builder = struct_ident(ident, StructIdent::Builder);
    let field = field_ident(&f.field, FieldIdent::Original);
    let upper = field_ident(&f.field, FieldIdent::UpperCamel);

    let mut args = struct_generics.as_args_vec();

    if f.default_value.is_some() {
        let optional = field_ident(&f.field, FieldIdent::Optional);
        args.push(parse_quote!(#upper = #optional));
    } else {
        let empty = field_ident(&f.field, FieldIdent::Empty);
        args.push(parse_quote!(#upper = #empty));
    };

    let args: AngleBracketedGenericArguments = parse_quote!(<#(#args),*>);

    let generics_with_state =
        add_state_param(fields, struct_generics, &parse_quote!(CS: State #args));

    let state_params = generics_with_state.as_params();
    let state_args = generics_with_state.as_args();

    let builder_fields = fields
        .iter()
        .map(|f_inner| {
            let field_inner = field_ident(&f_inner.field, FieldIdent::Original);
            let field_value = if f.nested_ty {
                quote!(Nested::unnest(#field_inner))
            } else {
                quote!(#field_inner)
            };

            let field_value = if !is_maybe && option_arg.is_some() {
                quote!(Some(#field_value))
            } else {
                field_value
            };

            if f.field == f_inner.field {
                let value_inner = field_ident(&f_inner.field, FieldIdent::Value);
                quote!(#field_inner: #value_inner(#field_value),)
            } else {
                quote!(#field_inner: self.#field_inner,)
            }
        })
        .collect::<TokenStream>();

    let docs = setter_docs(path, f);
    quote!(
        impl #state_params #builder #state_args {
            #[doc=#docs]
            pub fn #fn_ident(self, #field: #ty) -> #builder #set_args {
                #builder {
                    #builder_fields
                }
            }
        }
    )
}

fn make_build(
    path: &Path,
    ident: &Ident,
    fields: &[BuilderField],
    struct_generics: &UniqueGenerics,
) -> TokenStream {
    let args = struct_generics.as_args();
    let builder = struct_ident(ident, StructIdent::Builder);
    let generics_with_state = add_state_param(
        fields,
        struct_generics,
        &parse_quote!(CS: State #args + Complete #args),
    );

    let state_params = generics_with_state.as_params();
    let state_args = generics_with_state.as_args();

    let struct_fields = fields
        .iter()
        .map(|f| {
            let field = field_ident(&f.field, FieldIdent::Original);

            quote!(
                #field: IsSet::get(self.#field),
            )
        })
        .collect::<TokenStream>();

    quote!(
        impl #state_params #builder #state_args {
            pub fn build(self) -> #path #args {
                #path {
                    #struct_fields
                }
            }
        }
    )
}
