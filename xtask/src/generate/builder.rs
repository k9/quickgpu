use std::collections::BTreeSet;

use convert_case::{Case, Casing};
use discover_exports::utils::id;
use quote::{ToTokens, format_ident, quote as q};
use syn::{
    GenericParam, Generics, Ident, Path,
    visit::{self, Visit},
};

use crate::generate::struct_entry::BuilderField;

#[derive(Default)]
pub struct StructGenerics {
    pub lifetimes: Vec<String>,
    pub types: Vec<String>,
}

pub struct GatherGenerics<'a> {
    pub struct_generics: &'a StructGenerics,
    pub lifetimes: BTreeSet<&'a syn::Lifetime>,
    pub types: BTreeSet<&'a syn::Ident>,
}

impl<'a> GatherGenerics<'a> {
    pub fn new(struct_generics: &'a StructGenerics) -> Self {
        Self {
            struct_generics,
            lifetimes: BTreeSet::new(),
            types: BTreeSet::new(),
        }
    }
}

impl<'a> Visit<'a> for GatherGenerics<'a> {
    fn visit_lifetime(&mut self, lifetime: &'a syn::Lifetime) {
        if self
            .struct_generics
            .lifetimes
            .contains(&lifetime.to_string())
        {
            self.lifetimes.insert(lifetime);
        }

        visit::visit_lifetime(self, lifetime);
    }

    fn visit_type_path(&mut self, path: &'a syn::TypePath) {
        if let Some(last) = path.path.segments.last()
            && self
                .struct_generics
                .types
                .contains(&last.to_token_stream().to_string())
        {
            self.types.insert(&last.ident);
        }

        visit::visit_type_path(self, path);
    }
}

pub fn output_builder_code(
    _path: &Path,
    ident: Ident,
    fields: &[BuilderField],
    generics: &Generics,
    _generics_with_constraints: &Generics,
) -> String {
    let builder_ident = id(format!("{}Builder", ident.to_string()));

    let fn_ident = id(builder_ident.to_string().to_case(Case::Snake).as_str());
    let builder_mod_ident = fn_ident.clone();

    let mut struct_generics = StructGenerics::default();

    for param in &generics.params {
        match param {
            GenericParam::Lifetime(lifetime) => {
                struct_generics
                    .lifetimes
                    .push(lifetime.to_token_stream().to_string());
            }
            GenericParam::Type(ty) => {
                struct_generics.types.push(q!(#ty).to_string());
            }
            _ => (),
        };
    }

    let field_types = fields.iter().map(|f| {
        let ident = f
            .field
            .ident
            .as_ref()
            .unwrap()
            .to_string()
            .to_case(Case::Pascal);

        let set_ident = format_ident!("{}Set", ident);
        let unset_ident = format_ident!("{}Unset", ident);
        let required_ident = format_ident!("{}Required", ident);
        let is_unset_ident = format_ident!("{}IsUnset", ident);
        let ty = &f.field.ty;

        let mut generics = GatherGenerics::new(&struct_generics);
        generics.visit_type(ty);

        let lifetimes = generics.lifetimes.iter().map(|lifetime| q!(#lifetime));
        let types = generics.types.iter().map(|ty| q!(#ty));
        let generics = lifetimes.chain(types);
        let generics = q!(<#(#generics),*>);

        q!(
            pub struct #set_ident #generics(pub #ty);
            pub struct #unset_ident #generics(PhantomData<#ty>);
            pub trait #required_ident {}
            pub trait #is_unset_ident: #required_ident {}

            impl #generics #required_ident for #unset_ident #generics {}
            impl #generics #is_unset_ident for #unset_ident #generics {}
            impl #generics #required_ident for #set_ident #generics {}
            impl #generics #set_ident #generics {
                fn get(self) -> #ty {
                    self.0
                }
            }
        )
    });

    let builder_code = q!(
        pub mod #builder_mod_ident {
            use std::{marker::PhantomData, num::NonZeroU32, borrow::Cow, ops::Range};

            #(#field_types)*
        }
    )
    .to_string();

    builder_code
}
