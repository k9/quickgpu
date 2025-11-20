use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote as q};
use syn::{GenericArgument, GenericParam, Generics, Ident, Path, parse_quote, visit::Visit};

use crate::{
    generate::{
        builder_generics::{
            BuildImplGenerics, SetterImplGenerics, make_build_impl_generics,
            make_setter_impl_generics,
        },
        struct_entry::{BuilderField, ident_from_path},
    },
    type_helpers::{GatherGenerics, UniqueGenerics},
    utils::{OptionType, option_argument, option_type, upper_camel_ident},
};

pub fn output_builder_code(
    path: &Path,
    fields: &[BuilderField],
    struct_generics: &Generics,
    generate_nested_impl: bool,
) -> String {
    let ident = ident_from_path(path).unwrap();
    let builder_ident = format_ident!("{}Builder", ident);
    let fn_ident = format_ident!("{}", ident.to_string().to_case(Case::Snake));

    let label = fields
        .iter()
        .find(|f| f.field.ident.as_ref().unwrap().to_string() == "label");

    let builder_mod_ident = format_ident!("builder_{}", ident.to_string().to_case(Case::Snake));
    let struct_generics = UniqueGenerics::new(Some(struct_generics.clone()));
    let SetterImplGenerics {
        setter_impl_args,
        setter_impl_params,
        ..
    } = make_setter_impl_generics(fields, None, &struct_generics);

    let mut builder_struct_generics = UniqueGenerics::new(None);
    let mut builder_fields = vec![];
    for (i, f) in fields.iter().enumerate() {
        let field_ident = &f.field.ident;
        let param = format_ident!("T{}", i);
        builder_struct_generics.insert(&parse_quote!(#param));

        builder_fields.push(q!(
            pub(crate) #field_ident: #param
        ));
    }

    let builder_struct_params = builder_struct_generics.as_params();

    let mut field_types_list = vec![];
    for f in fields.iter() {
        field_types_list.push(field_types(f, &struct_generics));
    }

    let mut setters = vec![];
    for f in fields.iter() {
        setter(&mut setters, fields, f, &builder_ident, &struct_generics);
    }

    let BuildImplGenerics {
        build_where,
        build_impl_args,
        build_impl_params,
        build_fn_params,
    } = make_build_impl_generics(&fields, &struct_generics);

    let mut build_fields = vec![];
    for f in fields.iter() {
        let ident = &f.field.ident;
        if f.default_value.is_some() {
            build_fields.push(q!(#ident: self.#ident.resolve()));
        } else {
            build_fields.push(q!(#ident: self.#ident.resolve()));
        }
    }

    let build_impl_params = build_impl_params.as_params_vec();
    let struct_generic_args = struct_generics.as_args();

    let mut builder_new_generic_params = vec![];
    let mut builder_new_fields = vec![];
    let mut builder_new_params = vec![];

    let mut constructor_calls = q!();
    let constructor_generic_params;
    let constructor_generic_args;
    let mut constructor_return_args = vec![];

    if let Some(label) = label {
        let ty = &label.field.ty;
        builder_new_params.push(q!(label: #ty));
        constructor_calls = q!(.label(label));

        let mut gather = GatherGenerics::new(&struct_generics);
        gather.visit_type(&label.field.ty);
        constructor_generic_params = gather.used.as_params();
        constructor_generic_args = gather.used.as_args();
    } else {
        constructor_generic_params = q!();
        constructor_generic_args = q!();
    };

    for f in fields.iter() {
        let ident = &f.field.ident;
        let upper_camel = upper_camel_ident(&f.field);
        let ty = if f.default_value.is_some() {
            format_ident!("Unset{}Optional", upper_camel)
        } else {
            format_ident!("Unset{}", upper_camel)
        };

        if f.field.ident.as_ref().unwrap().to_string() == "label" {
            constructor_return_args.push(q!(LabelOptionalValue #constructor_generic_args));
        } else {
            constructor_return_args.push(q!(#ty));
        }

        builder_new_generic_params.push(q!(#ty));
        builder_new_fields.push(q!(#ident: #ty));
    }

    let nested = make_nested(
        path,
        &builder_ident,
        &build_fn_params,
        &build_impl_params,
        &build_impl_args,
        &build_where,
        &struct_generic_args,
        generate_nested_impl,
    );

    q!(
        pub use #builder_mod_ident::#fn_ident;

        pub mod #builder_mod_ident {
            #[allow(unused_imports)]
            use super::common::*;

            pub fn #fn_ident #constructor_generic_params (#(#builder_new_params),*) ->
                #builder_ident<#(#constructor_return_args),*> {
                #builder_ident::new() #constructor_calls
            }

            pub struct #builder_ident #builder_struct_params {
                #(#builder_fields),*
            }

            impl #builder_ident<#(#builder_new_generic_params),*> {
                pub fn new() -> Self {
                    Self {
                        #(#builder_new_fields),*
                    }
                }
            }

            #(#field_types_list)*

            impl <#(#setter_impl_params),*> #builder_ident <#(#setter_impl_args),*> {
                #(#setters)*
            }

            impl <#(#build_impl_params),*> #builder_ident <#(#build_impl_args),*> {
                pub fn build<#(#build_fn_params),*>(self) -> #path #struct_generic_args
                    where #(#build_where),* {
                    #path {
                        #(#build_fields),*
                    }
                }
            }

            #nested
        }
    )
    .to_string()
}

pub fn make_nested(
    path: &Path,
    builder_ident: &Ident,
    build_fn_params: &[TokenStream],
    build_impl_params: &[GenericParam],
    build_impl_args: &[GenericArgument],
    build_where: &[GenericParam],
    struct_generic_args: &TokenStream,
    generate_nested_impl: bool,
) -> TokenStream {
    let nested_impl_params = build_fn_params
        .iter()
        .cloned()
        .chain(build_impl_params.iter().map(|p| p.to_token_stream()));

    if generate_nested_impl {
        q!(
            impl<#(#nested_impl_params),*> Nested<#path #struct_generic_args> for
                #builder_ident <#(#build_impl_args),*>
                where #(#build_where),* {
                fn unnest(self) -> #path #struct_generic_args {
                    self.build()
                }
            }

            impl #struct_generic_args Nested<#path #struct_generic_args> for #path #struct_generic_args {
                fn unnest(self) -> #path #struct_generic_args {
                    self
                }
            }
        )
    } else {
        q!()
    }
}

fn setter(
    setters: &mut Vec<TokenStream>,
    fields: &[BuilderField<'_>],
    f: &BuilderField<'_>,
    builder_ident: &Ident,
    struct_generics: &UniqueGenerics,
) {
    let mut ty = f.field.ty.clone();
    let mut gather = GatherGenerics::new(struct_generics);
    gather.visit_type(&ty);

    let setter_generic_params = gather.used.as_params();

    let setter_ident = &f.field.ident;

    let SetterImplGenerics {
        setter_impl_args,
        setter_where_params,
        ..
    } = make_setter_impl_generics(&fields, Some(f), struct_generics);

    if option_type(&f.field) == OptionType::Option
        && let Some(arg) = option_argument(&mut ty.clone())
    {
        // Create a "maybe" setter which takes option directly
        let setter_fn_ident = format_ident!("maybe_{}", setter_ident.as_ref().unwrap());
        let setter_fields = make_setter_fields(&fields, Some(f), false);
        let nested_ty = if f.nested_ty {
            q!(impl Nested<#ty>)
        } else {
            q!(#ty)
        };

        setters.push(q!(
            pub fn #setter_fn_ident #setter_generic_params (self, #setter_ident: #nested_ty) ->
                #builder_ident< #(#setter_impl_args),*>
                where #(#setter_where_params),*
                  {
                    #builder_ident {
                        #(#setter_fields),*
                    }
                }
        ));

        // Unwrap option type for default setter
        ty = parse_quote!(#arg);
    };

    let setter_fields = make_setter_fields(&fields, Some(f), true);
    let nested_ty = if f.nested_ty {
        q!(impl Nested<#ty>)
    } else {
        q!(#ty)
    };

    setters.push(q!(
        pub fn #setter_ident #setter_generic_params (self, #setter_ident: #nested_ty) ->
            #builder_ident< #(#setter_impl_args),*>
            where #(#setter_where_params),*
              {
                #builder_ident {
                    #(#setter_fields),*
                }
            }
    ));
}

fn make_setter_fields(
    fields: &[BuilderField<'_>],
    selected: Option<&BuilderField<'_>>,
    unnest_option: bool,
) -> Vec<TokenStream> {
    let mut setter_fields = vec![];

    for f in fields {
        if let Some(selected) = selected
            && selected.field.ident.as_ref().unwrap().to_string()
                == f.field.ident.as_ref().unwrap().to_string()
        {
            let ident = &f.field.ident;
            let upper_camel = upper_camel_ident(&f.field);
            let value = if f.default_value.is_some() {
                format_ident!("{}OptionalValue", upper_camel)
            } else {
                format_ident!("{}Value", upper_camel)
            };

            let expr = if selected.nested_ty {
                q!(#ident .unnest())
            } else {
                q!(#ident)
            };

            if unnest_option && option_type(&f.field) == OptionType::Option {
                setter_fields.push(q!(#ident: #value(Some(#expr))));
            } else {
                setter_fields.push(q!(#ident: #value(#expr)));
            }
        } else {
            let ident = &f.field.ident;
            setter_fields.push(q!(#ident: self.#ident));
        }
    }

    setter_fields
}

fn field_types(f: &BuilderField<'_>, struct_generics: &UniqueGenerics) -> proc_macro2::TokenStream {
    let ty = &f.field.ty;
    let mut gather = GatherGenerics::new(struct_generics);
    gather.visit_type(ty);

    let impl_args = gather.used.as_args();
    let impl_params = gather.used.as_params();

    if let Some(default_value) = &f.default_value {
        let upper_camel = upper_camel_ident(&f.field);
        let unset_optional = format_ident!("Unset{}Optional", upper_camel);
        let optional_value = format_ident!("{}OptionalValue", upper_camel);

        q!(
            pub struct #unset_optional;
            impl IsOptional for #unset_optional {}
            impl IsUnsetOptional for #unset_optional {}
            impl #impl_params ResolveOptional<#ty> for #unset_optional {
                fn resolve(self) -> #ty {
                    #default_value
                }
            }

            pub struct #optional_value #impl_params (pub #ty);
            impl #impl_params IsOptional for #optional_value #impl_args {}
            impl #impl_params ResolveOptional<#ty> for #optional_value #impl_args {
                fn resolve(self) -> #ty {
                    self.0
                }
            }
        )
    } else {
        let upper_camel = upper_camel_ident(&f.field);
        let unset = format_ident!("Unset{}", upper_camel);
        let value = format_ident!("{}Value", upper_camel);

        q!(
            pub struct #unset;
            impl IsRequired for #unset {}
            impl IsUnset for #unset {}
            pub struct #value #impl_params (pub #ty);
            impl #impl_params IsRequired for #value #impl_args {}
            impl #impl_params Resolve<#ty> for #value #impl_args {
                fn resolve(self) -> #ty {
                    self.0
                }
            }
        )
    }
}
