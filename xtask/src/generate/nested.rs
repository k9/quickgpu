use std::collections::HashMap;

use convert_case::Casing;
use discover_exports::{EntryIndex, utils::id};
use proc_macro2::TokenStream;
use quote::quote as q;
use syn::{
    GenericParam, Generics, Path, Type, parse_quote,
    visit_mut::{self, VisitMut},
};

use crate::{generate::struct_entry::BuilderField, utils::without_args};

pub struct BuilderResolve<'a> {
    pub nested_impl: Option<TokenStream>,
    pub builders: &'a HashMap<String, (EntryIndex, Path)>,
}

impl<'a> VisitMut for BuilderResolve<'a> {
    fn visit_type_mut(&mut self, ty: &mut syn::Type) {
        if let Type::Path(path) = ty {
            let idents = without_args(&path.path);
            if self.builders.get(&q!(#idents).to_string()).is_some() {
                let nested_impl = q!(Nested<#ty>);
                *ty = parse_quote!(impl #nested_impl);
                self.nested_impl = Some(nested_impl);
                return;
            }
        } else if let Type::Slice(_) = ty {
            return;
        };

        visit_mut::visit_type_mut(self, ty);
    }
}

pub fn output_nested(
    path: Path,
    fields: &[BuilderField],
    generics: &Generics,
    generics_with_constraints: &Generics,
) -> String {
    let builder_ident = path.segments.last().unwrap().clone().ident;
    let builder_ident = id(format!("{}Builder", builder_ident.to_string())
        .replace("Origin2d", "Origin2D")
        .replace("Origin3d", "Origin3D")
        .replace("Extent3d", "Extent3D"));

    let builder_path = q!(crate::builders::#builder_ident);

    let mut builder_generics = generics.clone();
    add_builder_generics(fields, &mut builder_generics, false);

    let mut builder_generics_with_constraints = generics_with_constraints.clone();
    add_builder_generics(fields, &mut builder_generics_with_constraints, true);

    let builder_mod = builder_ident
        .to_string()
        .to_case(convert_case::Case::Snake)
        .replace("_2_d", "2_d")
        .replace("_3_d", "3_d")
        .replace("dx_12", "dx12");

    let builder_mod = id(builder_mod);

    let nested_impl = q!(
        impl #generics_with_constraints Nested<#path #generics> for #path #generics {
            fn unnest(self) -> #path #generics {
                self
            }
        }

        impl #builder_generics_with_constraints Nested<#path #generics> for #builder_path #builder_generics
        where BuilderState: crate::builders::#builder_mod::IsComplete,
        {
            fn unnest(self) -> #path #generics {
                self.build()
            }
        }
    )
    .to_string();

    nested_impl
}

fn add_builder_generics(
    fields: &[BuilderField],
    builder_generics: &mut Generics,
    add_constraints: bool,
) {
    for (i, field) in fields.iter().enumerate() {
        if let Some(nested_impl) = &field.nested_impl {
            let ident = id(format!("NestedField{}", i));
            let param: GenericParam = if add_constraints {
                parse_quote!(#ident: #nested_impl)
            } else {
                parse_quote!(#ident)
            };

            builder_generics.params.push(param);
        }
    }

    let param: GenericParam = parse_quote!(BuilderState);
    builder_generics.params.push(param);
}
