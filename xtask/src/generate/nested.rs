use std::collections::HashMap;

use convert_case::Casing;
use discover_exports::{EntryIndex, utils::id};
use quote::quote as q;
use syn::{
    GenericParam, Generics, Path, Type, parse_quote,
    visit_mut::{self, VisitMut},
};

use crate::utils::without_args;

pub struct BuilderResolve<'a> {
    pub nested_impl: bool,
    pub builders: &'a HashMap<String, (EntryIndex, Path)>,
}

impl<'a> VisitMut for BuilderResolve<'a> {
    fn visit_type_mut(&mut self, ty: &mut syn::Type) {
        if let Type::Path(path) = ty {
            let idents = without_args(&path.path);
            if self.builders.get(&q!(#idents).to_string()).is_some() {
                *ty = parse_quote!(impl Nested<#ty>);
                self.nested_impl = true;
                return;
            }
        }

        visit_mut::visit_type_mut(self, ty);
    }
}

pub fn output_nested(path: Path, nested_fields: usize, generics: Generics) -> String {
    let builder_ident = path.segments.last().unwrap().clone().ident;
    let builder_ident = id(format!("{}Builder", builder_ident.to_string())
        .replace("Origin2d", "Origin2D")
        .replace("Origin3d", "Origin3D")
        .replace("Extent3d", "Extent3D"));

    let builder_path = q!(crate::builders::#builder_ident);

    let mut builder_generics = generics.clone();
    for i in 0..nested_fields {
        let ident = id(format!("T{}", i));
        let param: GenericParam = parse_quote!(#ident);
        builder_generics.params.push(param);
    }

    let param: GenericParam = parse_quote!(BuilderState);
    builder_generics.params.push(param);

    let builder_mod = builder_ident
        .to_string()
        .to_case(convert_case::Case::Snake)
        .replace("_2_d", "2_d")
        .replace("_3_d", "3_d")
        .replace("dx_12", "dx12");

    let builder_mod = id(builder_mod);

    let nested_impl = q!(
        impl #generics Nested<#path #generics> for #path #generics {
            fn unnest(self) -> #path #generics {
                self
            }
        }

        impl #builder_generics Nested<#path #generics> for #builder_path #builder_generics
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
