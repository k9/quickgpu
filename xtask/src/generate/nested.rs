use std::collections::HashMap;

use discover_exports::EntryIndex;
use quote::quote as q;
use syn::{
    Path, Type,
    visit_mut::{self, VisitMut},
};

use crate::utils::{option_argument, without_args};

pub struct BuilderResolve<'a> {
    pub nested_ty: bool,
    pub builders: &'a HashMap<String, (EntryIndex, Path)>,
}

impl<'a> VisitMut for BuilderResolve<'a> {
    fn visit_type_mut(&mut self, ty: &mut syn::Type) {
        if let Type::Path(path) = ty {
            let idents = without_args(&path.path);
            if self.builders.get(&q!(#idents).to_string()).is_some() {
                self.nested_ty = true;
            }

            if path.path.segments.last().map(|s| s.ident.to_string()) == Some("Option".to_string())
            {
                if let Some(arg) = option_argument(ty) {
                    visit_mut::visit_generic_argument_mut(self, arg);
                }
            }
        }
    }
}
