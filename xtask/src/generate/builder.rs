use convert_case::{Case, Casing};
use discover_exports::utils::id;
use quote::quote as q;
use syn::{Generics, Ident, Path};

use crate::generate::struct_entry::BuilderField;

pub fn output_builder_code(
    path: &Path,
    ident: Ident,
    fields: &[BuilderField],
    generics: &Generics,
    generics_with_constraints: &Generics,
) -> String {
    let fn_ident = id(ident.to_string().to_case(Case::Snake).as_str());

    let fn_params = fields.iter().map(|f| {
        let ident = &f.field.ident;
        let ty = &f.field.ty;
        let builder_attr = match &f.default_value {
            Some(value) => q!(#[builder(#value)]),
            None => q!(),
        };

        q!(
            #builder_attr
            #ident: #ty
        )
    });

    let struct_values = fields.iter().map(|f| {
        let ident = &f.field.ident;
        let value = if f.nested_impl.is_some() {
            q!(#ident .unnest())
        } else {
            q!(#ident)
        };

        q!(#ident: #value)
    });

    let builder_code = q!(
        #[bon::builder(
            //builder_type(doc __builder_type_docs__),
            state_mod(vis="pub(crate)"),
            finish_fn=build,
        )]
        pub fn #fn_ident #generics_with_constraints(
            #(#fn_params),*
        ) -> #path #generics {
            #path {
                #(#struct_values),*
            }
        }
    )
    .to_string();

    builder_code
}
