use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{GenericParam, Path, parse_quote};

use crate::{
    generate::{
        CreateWithDevice, Version, docs,
        struct_entry::{BuilderField, BuilderStruct, FieldIdent, StructIdent},
    },
    type_helpers::UniqueGenerics,
    utils::{final_path, option_argument, snake, without_args},
};

pub struct GeneratedBuilder {
    pub name: String,
    pub builder_use: String,
    pub builder_mod: String,
    pub code: String,
}

pub(crate) fn builder_code(builder_struct: &BuilderStruct, version: Version) -> GeneratedBuilder {
    let module = builder_struct.ident(StructIdent::BuilderMod);
    let builder_fn_ident = builder_struct.ident(StructIdent::Fn);

    let nested_impl = if builder_struct.generate_nested_impl {
        make_nested(builder_struct)
    } else {
        quote!()
    };

    let path = &builder_struct.path;
    let args = builder_struct.generics.as_args();
    let params = builder_struct.generics.as_params();
    let wgpu_source_ident = version.wgpu_source_ident();

    let mut label_field = None;
    let mut param_fields = vec![];
    builder_struct.fields.iter().for_each(|f| {
        let f_ident = f.ident(FieldIdent::SetterFn);
        let f_with = if f.nested_ty {
            let f_type = f.field.ty.clone();
            let outer_path: Path = parse_quote!(#f_type);
            let path: Path = if outer_path.segments.last().map(|s| s.ident.to_string())
                == Some("Option".to_string())
                && let Some(argument) = option_argument(&mut f_type.clone())
            {
                parse_quote!(#argument)
            } else {
                parse_quote!(#f_type)
            };

            let last = path.segments.last().unwrap();
            let args = last.arguments.clone();

            let path = without_args(&path);
            let name = final_path(&path.to_token_stream().to_string()).unwrap();
            let builder = format_ident!("{}_builder", snake(name.clone()));
            let name = format_ident!("Nested{}", name);

            quote!(
                with = |b: impl super::#builder::#name #args| b.unnest(),
            )
        } else {
            quote!()
        };

        let f_type = f.field.ty.clone();

        let f_default = if let Some(f_default) = &f.default_value {
            quote!(default = #f_default,)
        } else {
            quote!()
        };

        let builder_args = quote!(#[builder(#f_with #f_default)]);
        let builder_args = if builder_args.to_string() == quote!(#[builder()]).to_string() {
            quote!()
        } else {
            builder_args
        };

        if f.field.ident.as_ref().unwrap() == "label" {
            label_field = Some(quote!(
                #[builder(start_fn)]
                #f_ident: #f_type
            ));
        } else {
            param_fields.push(quote!(
                #builder_args
                #f_ident: #f_type
            ));
        }
    });

    if let Some(label_field) = label_field {
        param_fields = [vec![label_field], param_fields].concat();
    }

    let return_fields = builder_struct.fields.iter().map(|f| {
        let f_ident = f.ident(FieldIdent::SetterFn);
        quote!(#f_ident)
    });

    let create_with_device = match builder_struct.create_with_device {
        Some(CreateWithDevice {
            use_reference,
            output,
            name,
            ..
        }) => {
            let reference = if *use_reference { quote!(&) } else { quote!() };
            let builder_struct_ident = builder_struct.ident(StructIdent::Builder);

            let mut generics = builder_struct.generics.clone();
            generics.insert(&parse_quote!(S: state::IsComplete));
            let params = generics.as_params();
            let args = generics.as_args();

            quote! {
                impl #params #builder_struct_ident #args {
                    pub fn create_with(self, device: &#wgpu_source_ident::Device) #output {
                       device.#name(#reference self.build())
                    }
                }
            }
        }
        _ => quote!(),
    };

    let builder_docs = docs::builder_docs(builder_struct);
    let builder_fn_docs = docs::builder_fn_docs(builder_struct);

    let code = quote!(
        #[allow(unused_imports)]
        use std::{borrow::Cow, num::NonZeroU32, ops::Range};

        #[allow(unused_imports)]
        use #wgpu_source_ident::util::DeviceExt;

        #[bon::builder(
            state_mod(vis = "pub(crate)", name = "state"),
            finish_fn = "build",
            builder_type(
              doc {
                #[doc = #builder_fn_docs]
              }
            ),
        )]
        #[doc = #builder_docs]
        pub fn #builder_fn_ident #params (
            #(#param_fields),*
        ) -> #path #args {
            #path {
                #(#return_fields),*
            }
        }

        #create_with_device

        #nested_impl
    )
    .to_string();

    let builder_use = quote!(
        #[doc(inline)]
        pub use builders::#builder_fn_ident;
    )
    .to_string();

    let builder_mod = quote!(
        pub mod #module;
        #[doc(hidden)]
        pub use #module::*;
    )
    .to_string();

    GeneratedBuilder {
        name: module.to_string(),
        builder_use,
        builder_mod,
        code,
    }
}

fn make_nested(builder_struct: &BuilderStruct) -> TokenStream {
    let builder = builder_struct.ident(StructIdent::Builder);
    let nested = builder_struct.ident(StructIdent::Nested);
    let params = builder_struct.generics.as_params();
    let args = builder_struct.generics.as_args();

    let generics_with_state = add_state_param(
        &builder_struct.fields,
        &builder_struct.generics,
        &parse_quote!(S:state::IsComplete),
        false,
    );

    let state_params = generics_with_state.as_params();
    let state_args = generics_with_state.as_args();
    let path = &builder_struct.path;

    quote!(
        pub trait #nested #params {
            fn unnest(self) -> #path #args;
        }

        impl #params #nested #args for #path #args {
            fn unnest(self) -> #path #args {
                self
            }
        }

        impl #state_params #nested #args for #builder #state_args {
            fn unnest(self) -> #path #args {
                self.build()
            }
        }
    )
}

pub(crate) fn add_state_param(
    fields: &[BuilderField<'_>],
    struct_generics: &UniqueGenerics,
    param: &GenericParam,
    add_with_zero_fields: bool,
) -> UniqueGenerics {
    let mut generics_with_state = struct_generics.clone();
    if add_with_zero_fields || !fields.is_empty() {
        generics_with_state.insert(param);
    }

    generics_with_state
}
