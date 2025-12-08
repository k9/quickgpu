use proc_macro2::TokenStream;
use quote::{ToTokens, quote as q};
use syn::{GenericArgument, GenericParam, Generics, Ident, Path, parse_quote, visit::Visit};

use crate::{
    generate::{
        builder_generics::{
            BuildImplGenerics, SetterImplGenerics, make_build_impl_generics,
            make_setter_impl_generics,
        },
        docs::{builder_docs, builder_fn_docs, setter_docs},
        state::make_typestate,
        struct_entry::{BuilderField, ident_from_path},
    },
    type_helpers::{GatherGenerics, UniqueGenerics},
    utils::{
        FieldIdent, OptionType, StructIdent, field_ident, option_argument, option_type,
        struct_ident,
    },
};

pub fn builder_code(
    path: &Path,
    fields: &[BuilderField],
    struct_generics: &Generics,
    generate_nested_impl: bool,
) -> GeneratedBuilder {
    let ident = ident_from_path(path).unwrap();
    let builder_ident = struct_ident(&ident, StructIdent::Builder);
    let fn_ident = struct_ident(&ident, StructIdent::Fn);

    let label = fields
        .iter()
        .find(|f| f.field.ident.as_ref().unwrap().to_string() == "label");

    let builder_mod_ident = struct_ident(&ident, StructIdent::BuilderMod);
    let struct_generics = UniqueGenerics::new(Some(struct_generics.clone()));
    let SetterImplGenerics {
        setter_impl_args,
        setter_impl_params,
        ..
    } = make_setter_impl_generics(fields, None, &struct_generics);

    let mut builder_fields = vec![];
    for f in fields.iter() {
        let ident = field_ident(f.field, FieldIdent::Original);
        let upper_camel = field_ident(f.field, FieldIdent::UpperCamel);

        builder_fields.push(q!(
            pub(crate) #ident: S::#upper_camel
        ));
    }

    let mut field_types_list = vec![];
    for f in fields.iter() {
        field_types_list.push(field_types(f, &struct_generics));
    }

    let mut setters = vec![];
    for f in fields.iter() {
        setter(
            &mut setters,
            path,
            fields,
            f,
            &builder_ident,
            &struct_generics,
        );
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
    let args_without_state = struct_generics.as_args();

    let mut generics_with_state = struct_generics.clone();
    generics_with_state.insert(&parse_quote!(S: State #args_without_state));
    let args_with_state = generics_with_state.as_args();
    let params_with_state = generics_with_state.as_params();

    let generics_empty = struct_generics.clone();
    let params_empty = generics_empty.as_params();
    let mut args_empty = generics_empty.as_args_vec();
    args_empty.push(parse_quote!(Empty));
    let args_empty = args_empty.iter().map(|a| q!(#a,)).collect::<TokenStream>();

    let typestate = make_typestate(fields, &struct_generics, &generics_with_state);

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
        let ty = if f.default_value.is_some() {
            field_ident(&f.field, FieldIdent::Optional)
        } else {
            field_ident(&f.field, FieldIdent::Empty)
        };

        if f.field.ident.as_ref().unwrap().to_string() == "label" {
            constructor_return_args.push(q!(LabelOptValue #constructor_generic_args));
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
        &args_with_state,
        generate_nested_impl,
    );

    let builder_fn_docs = builder_fn_docs(path, fields);
    let builder_docs = builder_docs(path, fields);

    GeneratedBuilder {
        use_statement: q!(
            pub use builders::#builder_mod_ident::#fn_ident;
        )
        .to_string(),
        code: q!(
            pub mod #builder_mod_ident {
                #[allow(unused_imports)]
                use super::common::*;

                #[doc = #builder_fn_docs]
                pub fn #fn_ident #params_empty (#(#builder_new_params),*) ->
                    #builder_ident<#args_empty> {
                    #builder_ident::new() #constructor_calls
                }

                #[doc = #builder_docs]
                pub struct #builder_ident #params_with_state {
                    #(#builder_fields),*
                }

                impl #params_empty #builder_ident<#args_empty> {
                    pub fn new() -> Self {
                        Self {
                            #(#builder_new_fields),*
                        }
                    }
                }

                #(#field_types_list)*

                #typestate

                impl #params_with_state #builder_ident #args_with_state {
                    #(#setters)*
                }

                impl <#(#build_impl_params),*> #builder_ident <#(#build_impl_args),*> {
                    pub fn build<#(#build_fn_params),*>(self) -> #path #args_with_state
                        where #(#build_where),* {
                        #path {
                            #(#build_fields),*
                        }
                    }
                }

                #nested
            }
        )
        .to_string(),
    }
}

pub struct GeneratedBuilder {
    pub use_statement: String,
    pub code: String,
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
    path: &Path,
    fields: &[BuilderField<'_>],
    f: &BuilderField<'_>,
    builder_ident: &Ident,
    struct_generics: &UniqueGenerics,
) {
    let mut ty = f.field.ty.clone();
    let mut gather = GatherGenerics::new(struct_generics);
    gather.visit_type(&ty);

    let setter_generic_params = gather.used.as_params();

    let SetterImplGenerics {
        setter_impl_args,
        setter_where_params,
        ..
    } = make_setter_impl_generics(&fields, Some(f), struct_generics);

    let ident = field_ident(f.field, FieldIdent::Original);

    if option_type(&f.field) == OptionType::Option
        && let Some(arg) = option_argument(&mut ty.clone())
    {
        // Create a "maybe" setter which takes option directly
        let setter_maybe_fn_ident = field_ident(f.field, FieldIdent::SetterMaybeFn);
        let setter_fields = make_setter_fields(&fields, Some(f), false);
        let nested_ty = if f.nested_ty {
            q!(impl Nested<#ty>)
        } else {
            q!(#ty)
        };

        let upper = field_ident(f.field, FieldIdent::UpperCamel);
        let set = field_ident(f.field, FieldIdent::Set);
        let set_generics = struct_generics.clone();
        let mut set_args = set_generics.as_args_vec();
        set_args.push(parse_quote!(#set<S>));
        let set_args = set_args.iter().map(|a| q!(#a,)).collect::<TokenStream>();

        let setter_docs = setter_docs(path, f);
        setters.push(q!(
            #[doc = #setter_docs]
            pub fn #setter_maybe_fn_ident (self, #ident: #nested_ty) ->
                #builder_ident<#set_args>
                where S::#upper: IsEmpty
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

    let setter_fn_ident = field_ident(f.field, FieldIdent::SetterFn);
    let setter_docs = setter_docs(path, f);
    setters.push(q!(
        #[doc = #setter_docs]
        pub fn #setter_fn_ident (self, #ident: #nested_ty) ->
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
            let value = field_ident(f.field, FieldIdent::Value);

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

    let empty_types = if let Some(default_value) = &f.default_value {
        let optional = field_ident(f.field, FieldIdent::Optional);

        q!(
            pub struct #optional;
            impl IsOptional for #optional {}
            impl Field for #optional {}
            impl #impl_params IsSet<#ty> for #optional {
                fn get(self) -> #ty {
                    #default_value
                }
            }
        )
    } else {
        let empty = field_ident(f.field, FieldIdent::Empty);

        q!(
            pub struct #empty;
            impl Field for #empty {}
            impl IsEmpty for #empty {}
        )
    };

    let value = field_ident(&f.field, FieldIdent::Value);

    q!(
        #empty_types

        pub struct #value #impl_params (pub #ty);
        impl #impl_params Field for #value #impl_args {}
        impl #impl_params IsSet<#ty> for #value #impl_args {
            fn get(self) -> #ty {
                self.0
            }
        }
    )
}
