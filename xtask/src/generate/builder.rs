use convert_case::{Case, Casing};
use discover_exports::utils::id;
use quote::quote as q;
use syn::{GenericParam, Generics, Ident, Path, parse_quote};

use crate::generate::struct_entry::BuilderField;

pub fn output_builder_code(
    path: &Path,
    ident: Ident,
    fields: &[BuilderField],
    generics: &Generics,
    generics_with_constraints: &Generics,
) -> String {
    let builder_ident = path.segments.last().unwrap().clone().ident;
    let builder_ident = id(format!("{}Builder", builder_ident.to_string())
        .replace("Origin2d", "Origin2D")
        .replace("Origin3d", "Origin3D")
        .replace("Extent3d", "Extent3D"));

    let fn_ident = id(ident.to_string().to_case(Case::Snake).as_str());

    let field_generics = fields
        .iter()
        .map(|f| {
            let name = &f.field.ident.as_ref().unwrap().to_string();
            let name = format!("{}Field", name.to_case(Case::Pascal));

            id(name)
        })
        .collect::<Vec<_>>();

    let builder_generics = q!(<#(#field_generics),*>);

    let struct_fields = fields.iter().enumerate().map(|(i, f)| {
        let ident = &f.field.ident;
        let value = field_generics[i].clone();

        q!(#ident: #value)
    });

    let unset_generics = fields.iter().map(|f| {
        let ty = &f.field.ty;

        q!(Unset<#ty>)
    });

    let fn_fields = fields.iter().map(|f| {
        let ident = &f.field.ident;

        q!(#ident: Unset(PhantomData))
    });

    let finish_generics = generics.clone();
    let finish_generics_with_constraints = generics.clone();
    let finish_values = vec![q!()];

    let setters = fields.iter().map(|f| {
        let setter_ident = f.field.ident.clone();
        let setter_ty = &f.field.ty;
        let setter_param = q!(#setter_ident: #setter_ty);
        
        let setter_generics_before = fields.iter().enumerate().map(|(i, f_inner)| {
            let field_generic = &field_generics[i];
            let value = if f.field == f_inner.field {
                q!(Unset<#field_generic>)
            } else {
                q!(#field_generic)
            };

            q!(#value)
        });
        
        let setter_generics_before = q!(<#(#setter_generics_before),*>);

        let setter_generics_after = fields.iter().enumerate().map(|(i, f_inner)| {
            let field_generic = &field_generics[i];
            let value = if f.field == f_inner.field {
                q!(Set<#field_generic>)
            } else {
                q!(#field_generic)
            };

            q!(#value)
        });
        
        let setter_generics_after = q!(<#(#setter_generics_after),*>);
        
        let setter_values = fields.iter().map(|f| {
            let ident = &f.field.ident;
            let value = q!(Unset(PhantomData));

            q!(#ident: #value)
        });


        let mut start_generics_with_constraints = generics.clone();
        for generic in &field_generics {
            let param: GenericParam = parse_quote!(#generic);
            start_generics_with_constraints.params.push(param);
        }
    
        q!(
            impl #start_generics_with_constraints #builder_ident #setter_generics_before {
                pub fn #setter_ident (self, #setter_param) -> #builder_ident #setter_generics_after {
                    #builder_ident {
                        #(#setter_values),*
                    }
                }
            }
        )
    });

    let builder_code = q!(
        #[derive(Debug)]
        pub struct #builder_ident #builder_generics {
            #(#struct_fields),*
        }

        pub fn #fn_ident #generics_with_constraints () -> #builder_ident <#(#unset_generics),*> {
            #builder_ident {
               #(#fn_fields),*
            }
        }

        #(#setters)*

        impl #finish_generics_with_constraints #builder_ident #finish_generics
        {
            pub fn build(self) -> #path {
                #path {
                    #(#finish_values),*
                }
            }
        }
    )
    .to_string();

    builder_code
}
