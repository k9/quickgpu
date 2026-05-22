use proc_macro2::TokenStream;
use quote::quote;
use syn::{AngleBracketedGenericArguments, Type, parse_quote};

use crate::{
    generate::{
        CreateWithDevice, Version,
        builder::add_state_param,
        docs::setter_docs,
        struct_entry::{BuilderField, BuilderStruct, FieldIdent, StructIdent},
    },
    utils::{OptionType, option_argument, option_type},
};

pub fn make_setters(builder_struct: &BuilderStruct, version: Version) -> TokenStream {
    let setter_fns = builder_struct
        .fields
        .iter()
        .map(|f| make_setter(builder_struct, f))
        .collect::<TokenStream>();

    let builder = builder_struct.ident(StructIdent::Builder);
    let build_fn = make_build(builder_struct, version);

    let args = builder_struct.generics.as_args();
    let generics_with_state = add_state_param(
        &builder_struct.fields,
        &builder_struct.generics,
        &parse_quote!(CS: State #args),
        false,
    );

    let state_params = generics_with_state.as_params();
    let state_args = generics_with_state.as_args();

    quote!(
        impl #state_params #builder #state_args {
            #setter_fns
        }
        #build_fn
    )
}

fn make_setter(builder_struct: &BuilderStruct, f: &BuilderField) -> TokenStream {
    let set = f.ident(FieldIdent::Set);

    let ty = &f.field.ty;
    let option_arg = if option_type(f.field) == OptionType::Option
        && let Some(arg) = option_argument(&mut ty.clone())
    {
        Some(arg.clone())
    } else {
        None
    };

    let mut set_args = builder_struct.generics.as_args_vec();
    set_args.push(parse_quote!(#set <CS>));
    let set_args = quote!(<#(#set_args),*>);

    let mut code = make_setter_fn(builder_struct, f, &option_arg, ty, &set_args, false);

    if option_arg.is_some() {
        let maybe_code = make_setter_fn(builder_struct, f, &option_arg, ty, &set_args, true);

        code = quote!(
            #code
            #maybe_code
        );
    }

    code
}

fn make_setter_fn(
    builder_struct: &BuilderStruct,
    f: &BuilderField,
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
        f.ident(FieldIdent::SetterMaybeFn)
    } else {
        f.ident(FieldIdent::SetterFn)
    };

    let builder = builder_struct.ident(StructIdent::Builder);
    let field = f.ident(FieldIdent::Original);
    let upper = f.ident(FieldIdent::UpperCamel);
    let is_empty = f.ident(FieldIdent::IsEmpty);

    let bounds = quote!(CS::#upper: #is_empty);
    let builder_fields = builder_struct
        .fields
        .iter()
        .map(|f_inner| {
            let field_inner = f_inner.ident(FieldIdent::Original);
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
                let value_inner = f_inner.ident(FieldIdent::Value);
                quote!(#field_inner: #value_inner(#field_value),)
            } else {
                quote!(#field_inner: self.#field_inner,)
            }
        })
        .collect::<TokenStream>();

    let docs = setter_docs(&builder_struct.path, f);
    quote!(
        #[doc=#docs]
        pub fn #fn_ident(self, #field: #ty) -> #builder #set_args
            where #bounds {
            #builder {
                #builder_fields
            }
        }
    )
}

fn make_build(builder_struct: &BuilderStruct, version: Version) -> TokenStream {
    let params = builder_struct.generics.as_params();
    let args = builder_struct.generics.as_args();
    let builder = builder_struct.ident(StructIdent::Builder);

    let mut state_args = builder_struct.generics.as_args_vec();
    for f in &builder_struct.fields {
        let upper = f.ident(FieldIdent::UpperCamel);
        let is_set = f.ident(FieldIdent::IsSet);
        let args = f.generics.as_args();

        state_args.push(parse_quote!(#upper: #is_set #args));
    }

    let inner_state_args: AngleBracketedGenericArguments = parse_quote!(<#(#state_args),*>);

    let generics_with_state = add_state_param(
        &builder_struct.fields,
        &builder_struct.generics,
        &parse_quote!(CS: State #inner_state_args),
        true,
    );

    let state_params = generics_with_state.as_params();

    let build_generics = add_state_param(
        &builder_struct.fields,
        &builder_struct.generics,
        &parse_quote!(CS: Complete #args),
        false,
    );

    let build_params = build_generics.as_params();
    let build_args = build_generics.as_args();

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

    let path = &builder_struct.path;

    let wgpu_source_ident = version.wgpu_source_ident();
    let create_with_device = match builder_struct.create_with_device {
        Some(CreateWithDevice {
            use_reference,
            output,
            name,
            ..
        }) => {
            let reference = if *use_reference { quote!(&) } else { quote!() };

            quote! {
                pub fn create_with(self, device: &#wgpu_source_ident::Device) #output {
                   device.#name(#reference self.build())
                }
            }
        }
        _ => quote!(),
    };

    quote!(
        pub trait Complete #params: State #inner_state_args {}
        impl #state_params Complete #args for CS {}

        impl #build_params #builder #build_args {
            pub fn build(self) -> #path #args {
                #path {
                    #struct_fields
                }
            }

            #create_with_device
        }
    )
}
