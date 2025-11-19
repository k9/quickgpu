use convert_case::{Case, Casing};
use discover_exports::utils::id;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote as q};
use syn::{GenericArgument, GenericParam, Generics, Ident, Path, parse_quote, visit::Visit};

use crate::{
    generate::struct_entry::{BuilderField, ident_from_path},
    type_helpers::{GatherGenerics, UniqueGenerics},
};

pub fn output_builder_code(
    path: &Path,
    fields: &[BuilderField],
    struct_generics: &Generics,
) -> String {
    let ident = ident_from_path(path).unwrap();
    let builder_ident = format_ident!("{}Builder", ident);
    let fn_ident = format_ident!("{}", builder_ident.to_string().to_case(Case::Snake));

    let builder_mod_ident = format_ident!("builder_{}", ident.to_string().to_case(Case::Snake));
    let struct_generics = UniqueGenerics::new(Some(struct_generics.clone()));
    let SetterImplGenerics {
        setter_impl_args,
        setter_impl_params,
        setter_where_params,
    } = make_setter_impl_generics(fields, None, &struct_generics);

    let mut builder_struct_generics = UniqueGenerics::new(None);
    let mut builder_fields = vec![];
    for (i, f) in fields.iter().enumerate() {
        let field_ident = &f.field.ident;
        let param = format_ident!("T{}", i);
        builder_struct_generics.insert(&parse_quote!(#param));

        builder_fields.push(q!(
            #field_ident: #param
        ));
    }

    let builder_struct_params = builder_struct_generics.as_params();

    let mut field_types_list = vec![];
    for f in fields {
        field_types_list.push(field_types(f, &struct_generics));
    }

    let mut setters = vec![];
    for f in fields {
        let setter_ident = &f.field.ident;

        let ty = &f.field.ty;
        let mut gather = GatherGenerics::new(&struct_generics);
        gather.visit_type(ty);

        let setter_generic_params = gather.used.as_params();

        let SetterImplGenerics {
            setter_impl_args,
            setter_impl_params,
            setter_where_params,
        } = make_setter_impl_generics(fields, Some(f), &struct_generics);
        let setter_fields = make_setter_fields(fields, Some(f));

        setters.push(q!(
            pub fn #setter_ident #setter_generic_params (self, #setter_ident: #ty) ->
                #builder_ident< #(#setter_impl_args),*>
                where #(#setter_where_params),*
                  {
                    #builder_ident {
                        #(#setter_fields),*
                    }
                }
        ));
    }

    let BuildImplGenerics {
        build_where,
        build_impl_args,
        build_impl_params,
        build_fn_params,
    } = make_build_impl_generics(fields, &struct_generics);

    let mut build_fields = vec![];
    for f in fields {
        let ident = &f.field.ident;
        if f.default_value.is_some() {
            build_fields.push(q!(#ident: self.#ident.resolve()));
        } else {
            build_fields.push(q!(#ident: self.#ident.0));
        }
    }

    let build_impl_params = build_impl_params.as_params_vec();
    let struct_generic_args = struct_generics.as_args();

    let mut builder_new_params = vec![];
    let mut builder_new_fields = vec![];

    for f in fields {
        let ident = &f.field.ident;
        let upper_camel = upper_camel_ident(f);
        let ty = if f.default_value.is_some() {
            format_ident!("Unset{}Optional", upper_camel)
        } else {
            format_ident!("Unset{}", upper_camel)
        };

        builder_new_params.push(q!(#ty));
        builder_new_fields.push(q!(#ident: #ty));
    }

    q!(
        pub mod #builder_mod_ident {
            use super::common::*;

            pub fn #fn_ident() -> #builder_ident<#(#builder_new_params),*> {
                #builder_ident::new()
            }

            pub struct #builder_ident #builder_struct_params {
                #(#builder_fields),*
            }

            impl #builder_ident<#(#builder_new_params),*> {
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
        }
    )
    .to_string()
}

pub struct SetterImplGenerics {
    pub setter_impl_args: Vec<TokenStream>,
    pub setter_impl_params: Vec<TokenStream>,
    pub setter_where_params: Vec<TokenStream>,
}

fn make_setter_impl_generics(
    fields: &[BuilderField<'_>],
    selected: Option<&BuilderField<'_>>,
    struct_generics: &UniqueGenerics,
) -> SetterImplGenerics {
    let mut setter_impl_params = vec![];
    let mut setter_impl_args = vec![];
    let mut setter_where_params = vec![];

    for (i, f) in fields.iter().enumerate() {
        if let Some(selected) = selected
            && selected.field.ident.as_ref().unwrap().to_string()
                == f.field.ident.as_ref().unwrap().to_string()
        {
            let upper_camel = upper_camel_ident(f);
            let ident = if f.default_value.is_some() {
                format_ident!("{}OptionalValue", upper_camel)
            } else {
                format_ident!("{}Value", upper_camel)
            };

            let ty = &f.field.ty;
            let mut gather = GatherGenerics::new(struct_generics);
            gather.visit_type(ty);

            setter_impl_params.push(q!(#ident));

            let impl_args = gather.used.as_args();
            setter_impl_args.push(q!(#ident #impl_args));

            let where_ident = format_ident!("T{}", i);
            let where_constraint = if f.default_value.is_some() {
                format_ident!("IsUnsetOptional")
            } else {
                format_ident!("IsUnset")
            };

            setter_where_params.push(q!(#where_ident: #where_constraint));
        } else {
            let constraint = if f.default_value.is_some() {
                format_ident!("IsOptional")
            } else {
                format_ident!("IsRequired")
            };

            let ident = format_ident!("T{}", i);
            setter_impl_args.push(q!(#ident));
            setter_impl_params.push(q!(#ident: #constraint));
        }
    }

    SetterImplGenerics {
        setter_impl_args,
        setter_impl_params,
        setter_where_params,
    }
}

pub struct BuildImplGenerics {
    pub build_where: Vec<GenericParam>,
    pub build_impl_params: UniqueGenerics,
    pub build_impl_args: Vec<GenericArgument>,
    pub build_fn_params: Vec<TokenStream>,
}

fn make_build_impl_generics(
    fields: &[BuilderField<'_>],
    struct_generics: &UniqueGenerics,
) -> BuildImplGenerics {
    let mut build_impl_params = UniqueGenerics::new(None);
    let mut build_where: Vec<GenericParam> = vec![];
    let mut build_impl_args: Vec<GenericArgument> = vec![];

    for f in fields {
        let ty = &f.field.ty;
        let upper_camel = upper_camel_ident(f);

        if f.default_value.is_some() {
            let param = format_ident!("R{}", upper_camel);
            let constraint = q!(ResolveOptional<#ty>);

            build_where.push(parse_quote!(#param: #constraint));
            build_impl_params.insert(&parse_quote!(#param));
            build_impl_args.push(parse_quote!(#param));
        } else {
            let arg = format_ident!("{}Value", upper_camel);

            let mut gather = GatherGenerics::new(&struct_generics);
            let ty = &f.field.ty;
            gather.visit_type(ty);

            let generic_args = gather.used.as_args();
            build_impl_args.push(parse_quote!(#arg #generic_args));
        };
    }

    let mut gather_impl_args = GatherGenerics::new(&struct_generics);
    for arg in build_impl_args.iter() {
        gather_impl_args.visit_generic_argument(arg);
    }

    for param in gather_impl_args.used.as_params_vec() {
        build_impl_params.insert(&param);
    }

    let mut build_fn_params = vec![];
    let mut gather_where_params = GatherGenerics::new(&struct_generics);
    for arg in build_where.iter() {
        gather_where_params.visit_generic_param(arg);
    }

    for param in gather_where_params.used.as_params_vec() {
        // Only add param to fn if it's not added to impl
        if gather_impl_args.used.get(&param).is_none() {
            build_fn_params.push(param.into_token_stream());
        }
    }

    BuildImplGenerics {
        build_where,
        build_impl_args,
        build_impl_params,
        build_fn_params,
    }
}

fn make_setter_fields(
    fields: &[BuilderField<'_>],
    selected: Option<&BuilderField<'_>>,
) -> Vec<TokenStream> {
    let mut setter_fields = vec![];

    for f in fields {
        if let Some(selected) = selected
            && selected.field.ident.as_ref().unwrap().to_string()
                == f.field.ident.as_ref().unwrap().to_string()
        {
            let ident = &f.field.ident;
            let upper_camel = upper_camel_ident(f);
            let value = if f.default_value.is_some() {
                format_ident!("{}OptionalValue", upper_camel)
            } else {
                format_ident!("{}Value", upper_camel)
            };

            setter_fields.push(q!(#ident: #value(#ident)));
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
        let upper_camel = upper_camel_ident(f);
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
        let upper_camel = upper_camel_ident(f);
        let unset = format_ident!("Unset{}", upper_camel);
        let value = format_ident!("{}Value", upper_camel);

        q!(
            pub struct #unset;
            impl IsRequired for #unset {}
            impl IsUnset for #unset {}
            pub struct #value #impl_params (pub #ty);
            impl #impl_params IsRequired for #value #impl_params {}
        )
    }
}

fn upper_camel_ident(f: &BuilderField<'_>) -> Ident {
    id(f.field
        .ident
        .as_ref()
        .unwrap()
        .to_string()
        .to_case(Case::UpperCamel))
}
