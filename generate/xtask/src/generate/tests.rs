use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::PathSep;
use syn::visit_mut::{self, VisitMut};
use syn::{AngleBracketedGenericArguments, GenericArgument, Type};

use crate::generate::struct_entry::{BuilderStruct, FieldIdent, StructIdent, ident_from_path};

struct TyVisitor;

impl VisitMut for TyVisitor {
    fn visit_type_mut(&mut self, ty: &mut syn::Type) {
        if let syn::Type::Reference(r) = ty {
            r.lifetime = None;
        };

        visit_mut::visit_type_mut(self, ty);
    }

    fn visit_angle_bracketed_generic_arguments_mut(
        &mut self,
        args: &mut AngleBracketedGenericArguments,
    ) {
        let old_args = args.args.clone();
        args.args = Punctuated::new();
        for arg in old_args.iter() {
            if let GenericArgument::Type(_) = arg {
                args.args.push(arg.clone())
            }
        }

        visit_mut::visit_angle_bracketed_generic_arguments_mut(self, args);
    }
}

pub fn builder_tests(builder_struct: &BuilderStruct, has_label: bool) -> TokenStream {
    let build_fn = builder_struct.ident(StructIdent::Fn);
    let path = &builder_struct.path;

    let assertions = if builder_struct.has_default {
        let label = if has_label {
            quote!(Option::<&str>::None)
        } else {
            quote!()
        };

        let ident = ident_from_path(path);
        let generics = if ident == Some(format_ident!("RequestAdapterOptionsBase")) {
            quote!(::<u32>)
        } else if ident == Some(format_ident!("CommandBufferDescriptor")) {
            quote!(::<Option<&str>>)
        } else if ident == Some(format_ident!("Operations")) {
            quote!(::<u32>)
        } else {
            quote!()
        };

        quote!(
            assert_eq!(
                format!("{:#?}", super:: #build_fn #generics(#label).build()),
                format!("{:#?}", #path #generics::default()),
            );
        )
    } else {
        let field_defaults = builder_struct.fields.iter().filter_map(|f| {
            if f.default_value.is_some()
                && let Type::Path(mut path) = f.field.ty.clone()
            {
                let empty = f.ident(FieldIdent::Empty);
                let is_set = f.ident(FieldIdent::IsSet);
                if let Some(last) = path.path.segments.last_mut()
                    && let syn::PathArguments::AngleBracketed(args) = &mut last.arguments
                {
                    TyVisitor.visit_angle_bracketed_generic_arguments_mut(args);
                    args.colon2_token = Some(PathSep::default());
                };

                Some(quote!(
                    assert_eq!(
                        format!("{:#?}", super::#is_set::get(super::#empty)),
                        format!("{:#?}", #path::default()),
                    );
                ))
            } else {
                None
            }
        });

        quote!(#(#field_defaults)*)
    };

    quote!(
        #[cfg(test)]
        mod tests {
            #[allow(unused_imports)]

            #[allow(unused_imports)]
            use std::num::NonZeroU32;

            #[test]
            pub fn test_default() {
                #assertions
            }
        }
    )
}
