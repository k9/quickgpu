use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    GenericArgument, GenericParam, Generics, parse_quote,
    punctuated::Punctuated,
    visit::{self, Visit},
};

#[derive(Clone)]
pub struct UniqueGenerics {
    inner: Generics,
}

impl UniqueGenerics {
    /// Wrap syn's Generics and only insert unique ones
    pub(crate) fn new(generics: Option<Generics>) -> Self {
        let mut unique = Self {
            inner: Generics::default(),
        };

        let to_process = generics.unwrap_or_default();

        for param in to_process.params {
            unique.insert(&param);
        }

        unique
    }

    pub(crate) fn insert(&mut self, param: &GenericParam) {
        if self.get(param).is_none() {
            self.inner.params.push(param.clone());
        }
    }

    pub(crate) fn get(&self, param: &GenericParam) -> Option<&GenericParam> {
        self.inner.params.iter().find(|existing| {
            as_arg(existing).into_token_stream().to_string()
                == as_arg(param).into_token_stream().to_string()
        })
    }

    pub(crate) fn get_mut(&mut self, param: &GenericParam) -> Option<&mut GenericParam> {
        self.inner.params.iter_mut().find(|existing| {
            as_arg(existing).into_token_stream().to_string()
                == as_arg(param).into_token_stream().to_string()
        })
    }

    pub(crate) fn as_params(&self) -> TokenStream {
        self.inner.clone().into_token_stream()
    }

    pub(crate) fn as_args(&self) -> TokenStream {
        let inner = self.inner.clone();
        let mut args = vec![];

        for param in &inner.params {
            if let Some(arg) = as_arg(param) {
                args.push(arg);
            }
        }

        if !args.is_empty() {
            quote!(<#(#args),*>)
        } else {
            quote!()
        }
    }
}

fn as_arg(param: &GenericParam) -> Option<GenericArgument> {
    match param {
        GenericParam::Lifetime(param) => {
            let mut param = param.clone();
            param.bounds = Punctuated::new();
            Some(GenericArgument::Lifetime(parse_quote!(#param)))
        }
        GenericParam::Type(param) => {
            let ident = &param.ident;
            Some(GenericArgument::Type(parse_quote!(#ident)))
        }
        _ => None,
    }
}

pub struct GatherGenerics<'a> {
    pub params: &'a UniqueGenerics,
    pub used: UniqueGenerics,
}

impl<'a> GatherGenerics<'a> {
    /// Find out which out of a set of params are used in
    /// some code.
    ///
    /// For example: Given the generic params of a struct,
    /// which of them are used in the definition of one of its fields?
    pub(crate) fn new(params: &'a UniqueGenerics) -> Self {
        Self {
            params,
            used: UniqueGenerics::new(None),
        }
    }
}

impl<'a> Visit<'a> for GatherGenerics<'a> {
    fn visit_lifetime(&mut self, lifetime: &'a syn::Lifetime) {
        let param = parse_quote!(#lifetime);
        if let Some(param) = self.params.get(&param) {
            self.used.insert(param);
        }

        visit::visit_lifetime(self, lifetime);
    }

    fn visit_type_path(&mut self, path: &'a syn::TypePath) {
        if let Some(last) = path.path.segments.last() {
            let ident = &last.ident;
            let param = parse_quote!(#ident);
            if let Some(param) = self.params.get(&param) {
                self.used.insert(param);
            }
        }

        visit::visit_type_path(self, path);
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::{Field, ItemStruct, parse_quote, visit::Visit};

    use crate::type_helpers::{GatherGenerics, UniqueGenerics};

    #[test]
    pub(crate) fn basics() {
        let a_struct: ItemStruct = parse_quote!(
            pub struct A<'a, 'b, T: Special> {}
        );

        let a_field: Field = parse_quote!(x: (&'a T, &'a T2));

        let struct_generics = UniqueGenerics::new(Some(a_struct.generics));
        let mut gather = GatherGenerics::new(&struct_generics);
        gather.visit_field(&a_field);

        assert_eq!(
            gather.used.as_params().to_string(),
            quote! {<'a, T: Special>}.to_string()
        );

        assert_eq!(
            gather.used.as_args().to_string(),
            quote! {<'a, T>}.to_string()
        );
    }
}
