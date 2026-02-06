use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, spanned::Spanned};

use crate::{field::process_field, inner::PUBLIC_NAMED, utils::err};

pub fn bind_group_code(input: DeriveInput) -> Result<TokenStream, TokenStream> {
    let ident = &input.ident;
    let input_span = input.span();
    let Data::Struct(data) = input.data else {
        return Err(err(input_span, PUBLIC_NAMED));
    };

    let Fields::Named(mut fields) = data.fields else {
        return Err(err(input_span, PUBLIC_NAMED));
    };

    let mut next_binding = 0u32;
    let field_entries: Result<Vec<_>, TokenStream> = fields
        .named
        .iter_mut()
        .map(|f| process_field(f, &mut next_binding))
        .collect();

    let field_entries = match field_entries {
        Ok(entries) => entries,
        Err(err) => {
            return Err(err);
        }
    };

    let field_entries = field_entries
        .into_iter()
        .filter_map(|f| f)
        .collect::<Vec<_>>();

    let bindings = field_entries
        .iter()
        .map(|entry| entry.binding.clone())
        .collect::<Vec<_>>()
        .join("\n");

    let bindings = format!(
        "

{bindings}

    "
    );

    let bind_group_layout_entries = field_entries.iter().map(|f| &f.bind_group_layout_entry);
    let bind_group_entries = field_entries.iter().map(|f| &f.bind_group_entry);
    let ident_resources = format_ident!("{}Resources", ident);
    let resource_fields = field_entries.iter().map(|f| &f.resource_field);
    let ident_slices = format_ident!("{}Slices", ident);
    let slice_fields = field_entries.iter().map(|f| &f.slice_field);
    let resource_inits = field_entries.iter().map(|f| &f.resource_init);
    let resource_writes = field_entries.iter().map(|f| &f.resource_write);
    let resource_write_fns = field_entries.iter().map(|f| &f.resource_write_fn);

    let code = quote! {
        mod mod_ident {
            use wgpu::*;
            use quickgpu::*;
            use quickgpu::builders::bind_group_layout_descriptor_builder::*;
            use super::#ident;

            pub struct #ident_resources {
                #(#resource_fields),*
            }

            impl #ident_resources {
                pub fn write(&self, queue: &Queue, data: #ident) {
                    #(#resource_writes);*
                }

                #(#resource_write_fns)*
            }

            pub struct #ident_slices<'a> {
                #(#slice_fields),*
            }

            impl #ident {
                pub const WGSL: &str = #bindings;

                pub fn layout_entries() -> Vec<BindGroupLayoutEntry> {
                    builders([
                        #(#bind_group_layout_entries),*
                    ])
                }

                pub fn bind_group<'a>(
                    device: &Device,
                    label: Option<&'a str>,
                    bgl: &wgpu::BindGroupLayout,
                    resources: &#ident_resources
                ) -> wgpu::BindGroup {
                    device.create_bind_group(
                        &bind_group_descriptor(label)
                            .layout(&bgl)
                            .entries(&builders([
                                #(#bind_group_entries),*
                            ]))
                            .build(),
                    )
                }

                pub fn resources(
                    device: &Device,
                    data: &#ident,
                ) -> #ident_resources {
                    #ident_resources {
                        #(#resource_inits),*
                    }
                }
            }
        }

        pub use mod_ident::{
            #ident_resources,
            #ident_slices
        };
    };

    Ok(code)
}
