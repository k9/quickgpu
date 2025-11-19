use convert_case::{Case, Casing};
use discover_exports::utils::id;
use quote::{ToTokens, format_ident, quote as q};
use syn::{GenericParam, Generics, Ident, Path, Type, parse_quote, visit::Visit};

use crate::{generate::struct_entry::BuilderField, type_helpers::GatherGenerics};

pub fn output_builder_code(
    path: &Path,
    ident: Ident,
    fields: &[BuilderField],
    struct_generics: &Generics,
    _generics_with_constraints: &Generics,
) -> String {
    let builder_ident = id(format!("{}Builder", ident.to_string()));

    let fn_ident = id(builder_ident.to_string().to_case(Case::Snake).as_str());
    let builder_mod_ident = fn_ident.clone();

    let mut field_types = vec![];
    let mut struct_generic_params = Generics::default();
    let mut struct_field_inits = vec![];
    let mut struct_new_generic_args = vec![];
    let mut struct_new_generic_params = vec![];
    let mut struct_new_inits = vec![];
    let mut struct_build_generic_args = vec![];
    let mut struct_build_fields = vec![];
    let mut field_impl_generics = vec![];
    let mut field_setters = vec![];
    let mut setter_impl_params = vec![];
    let mut setter_generic_params = Generics::default();
    for lifetime in struct_generics.lifetimes() {
        setter_generic_params
            .params
            .push(GenericParam::Lifetime(lifetime.clone()));
    }

    for (i, f) in fields.iter().enumerate() {
        let (ident, upper_ident) = field_idents(f);

        let set_ident = format_ident!("{}Set", upper_ident);
        let ty = &f.field.ty;

        let mut generics = GatherGenerics::new(&struct_generics);
        generics.visit_type(ty);

        let generics = generics.used.get_tokens();

        let type_ident = format_ident!("T{}", i);

        let unset_ident;
        if let Some(default_value) = &f.default_value {
            unset_ident = format_ident!("{}Unset", upper_ident);

            struct_generic_params
                .params
                .push(parse_quote!(#type_ident: Optional));

            if let Type::Path(path) = ty {
                if let Some(segment) = path.path.segments.last() {
                    if let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments {
                        for arg in &arguments.args {
                            if let Some(last) = path.path.segments.last()
                                && !setter_generic_params.params.iter().any(|existing| {
                                    existing.into_token_stream().to_string()
                                        == last.into_token_stream().to_string()
                                })
                            {
                                setter_generic_params.params.push(parse_quote!(#arg));
                            }
                        }
                    }
                }
            }

            if let Type::Path(path) = ty {
                if let Some(segment) = path.path.segments.last() {
                    if let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments {
                        for arg in &arguments.args {
                            if let Some(last) = path.path.segments.last()
                                && !struct_generic_params.params.iter().any(|existing| {
                                    existing.into_token_stream().to_string()
                                        == last.into_token_stream().to_string()
                                })
                            {
                                struct_generic_params.params.push(parse_quote!(#arg));
                            }
                        }
                    }
                }
            }

            setter_generic_params
                .params
                .push(parse_quote!(#type_ident: Optional));

            field_types.push(q!(
                pub struct #unset_ident;
                impl IsUnset for #unset_ident {}
                impl #generics Optional for #unset_ident {
                    type Output = #ty;
                    fn get(self) -> Self::Output {
                        #default_value
                    }
                }

                pub struct #set_ident #generics(pub #ty);
                impl #generics Optional for #set_ident #generics {
                    type Output = #ty;
                    fn get(self) -> Self::Output {
                        self.0
                    }
                }
            ));
        } else {
            unset_ident = format_ident!("Unset");

            struct_generic_params
                .params
                .push(parse_quote!(#type_ident: Required));

            setter_generic_params
                .params
                .push(parse_quote!(#type_ident: Required));

            field_types.push(q!(
                pub struct #set_ident #generics(pub #ty);
                impl #generics Required for #set_ident #generics {}
                impl #generics #set_ident #generics  {
                    fn get(self) -> #ty {
                        self.0
                    }
                }
            ));
        }

        struct_field_inits.push(q!(#ident: #type_ident));
        struct_new_generic_params.push(q!(#type_ident: #unset_ident));
        struct_new_generic_args.push(q!(#unset_ident));
        struct_build_generic_args.push(q!(#set_ident #generics));
        struct_build_fields.push(q!(#ident: self.#ident.get()));
        struct_new_inits.push(q!(#ident: #unset_ident));
        field_impl_generics.push(q!(#type_ident));
        setter_impl_params.push(q!(#type_ident));

        let mut setter_generics = vec![];
        let mut setter_field_assignments = vec![];
        for (i, f_inner) in fields.iter().enumerate() {
            let (ident, _upper_ident) = field_idents(f_inner);

            if f.field == f_inner.field {
                setter_generics.push(q!(#set_ident #generics));
                setter_field_assignments.push(q!(#ident: #set_ident(#ident)));
            } else {
                let type_ident = format_ident!("T{}", i);
                setter_generics.push(q!(#type_ident));
                setter_field_assignments.push(q!(#ident: self.#ident));
            }
        }

        field_setters.push(q!(
            pub fn #ident(self, #ident: #ty) -> #builder_ident <#(#setter_generics),*>
            where
                #type_ident: IsUnset,
            {
                #builder_ident {
                    #(#setter_field_assignments),*
                }
            }
        ));
    }

    let builder_code = q!(
        pub mod #builder_mod_ident {
            use super::common::*;

            #(#field_types)*

            pub fn #fn_ident() -> #builder_ident<#(#struct_new_generic_args),*> {
                #builder_ident::new()
            }

            pub struct #builder_ident #struct_generic_params {
                #(#struct_field_inits),*
            }

            impl #builder_ident<#(#struct_new_generic_args),*> {
                pub fn new() -> Self {
                    #builder_ident {
                        #(#struct_new_inits),*
                    }
                }
            }

            impl #setter_generic_params #builder_ident<#(#setter_impl_params),*> {
                #(#field_setters)*
            }

            impl #struct_generics #builder_ident<#(#struct_build_generic_args),*> {
                pub fn build(self) -> #path #struct_generics {
                    #path {
                        #(#struct_build_fields),*
                    }
                }
            }
        }
    )
    .to_string();

    builder_code
}

fn field_idents(f: &BuilderField<'_>) -> (Ident, Ident) {
    let ident = id(f.field.ident.as_ref().unwrap().to_string());
    let upper_ident = id(f
        .field
        .ident
        .as_ref()
        .unwrap()
        .to_string()
        .to_case(Case::Pascal));
    (ident, upper_ident)
}
