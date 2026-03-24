use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, spanned::Spanned};

use crate::{
    field::{BindingEntry, process_field},
    inner::PUBLIC_NAMED,
    utils::err,
};

pub fn bind_group_code(input: DeriveInput) -> Result<TokenStream, TokenStream> {
    let ident = &input.ident;
    let input_span = input.span();
    let Data::Struct(data) = input.data else {
        return Err(err(input_span, PUBLIC_NAMED));
    };

    let Fields::Named(mut fields) = data.fields else {
        return Err(err(input_span, PUBLIC_NAMED));
    };

    let field_entries: Result<Vec<_>, TokenStream> = fields
        .named
        .iter_mut()
        .enumerate()
        .map(|(i, f)| process_field(f, i as u32))
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

    let mod_ident = format_ident!("{ident}_mod");

    let helper_fields = field_entries.iter().map(|f| &f.helper_field);
    let helper_return_fields = field_entries.iter().map(|f| &f.helper_return_field);
    let helper_new_args = field_entries.iter().map(|f| &f.helper_new_arg);
    let helper_layout_descriptor_entries = field_entries
        .iter()
        .map(|f| &f.helper_layout_descriptor_entry);

    let buffer_fields = field_entries.iter().map(|f| &f.resource_field);

    let binding_entry_fields = field_entries
        .iter()
        .filter_map(|f| match &f.binding_entry {
            BindingEntry::Buffer { field, .. } => Some(field),
            BindingEntry::Sampler { .. } => None,
            BindingEntry::Texture { .. } => None,
        })
        .collect::<Vec<_>>();

    let binding_entry_constraints = field_entries
        .iter()
        .filter_map(|f| match &f.binding_entry {
            BindingEntry::Buffer { constraint, .. } => Some(constraint),
            BindingEntry::Sampler { .. } => None,
            BindingEntry::Texture { .. } => None,
        })
        .collect::<Vec<_>>();

    let binding_entry_params = field_entries
        .iter()
        .filter_map(|f| match &f.binding_entry {
            BindingEntry::Buffer { param, .. } => Some(param),
            BindingEntry::Sampler { .. } => None,
            BindingEntry::Texture { .. } => None,
        })
        .collect::<Vec<_>>();

    let binding_entry_makes = field_entries
        .iter()
        .filter_map(|f| match &f.binding_entry {
            BindingEntry::Buffer { make, .. } => Some(make),
            BindingEntry::Texture { entry, .. } => Some(entry),
            BindingEntry::Sampler { entry, .. } => Some(entry),
        })
        .collect::<Vec<_>>();

    let offset_fields = field_entries.iter().map(|f| &f.offset_field);
    let declaration_fields = field_entries.iter().map(|f| &f.declaration_field);
    let declaration_return_fields = field_entries.iter().map(|f| &f.declaration_return_field);

    let code = quote! {
        mod #mod_ident {
            use quickgpu::{Nested, bind_group_descriptor, bind_group_layout_descriptor, builders};
            use wgpu::*;

            use crate::bind::*;

            pub struct BgHelper {
                pub layout: BindGroupLayout,
                #(#helper_fields),*
            }

            pub struct BgBuffers<'a> {
                #(#buffer_fields),*
            }

            pub struct BgBindingEntries<#(#binding_entry_params),*> {
                #(#binding_entry_fields),*
            }

            #[derive(Copy, Clone)]
            pub struct BgOffsets {
                #(#offset_fields),*
            }

            pub struct BgDeclarations {
                #(#declaration_fields),*
            }

            impl BgHelper {
                pub fn new<'a>(
                    label: Label<'a>,
                    device: &Device,
                    #(#helper_new_args),*
                ) -> Self {
                    let layout = device.create_bind_group_layout(
                        &bind_group_layout_descriptor(label)
                            .entries(&builders([
                                #(#helper_layout_descriptor_entries),*
                            ]))
                            .build(),
                    );

                    Self {
                        layout,
                        #(#helper_return_fields),*
                    }
                }

                pub fn group<'a, #(#binding_entry_constraints),*>(
                    &self,
                    label: Label<'a>,
                    buffers: BgBuffers<'a>,
                    entries: BgBindingEntries<#(#binding_entry_params),*>,
                    offsets: Option<BgOffsets>,
                    device: &Device
                ) -> BindGroup {
                    device.create_bind_group(
                        &bind_group_descriptor(label)
                            .entries(&[
                                #(#binding_entry_makes),*
                            ])
                            .layout(&self.layout)
                            .build(),
                    )
                }

                pub fn declarations(&self, group: u32) -> BgDeclarations {
                    BgDeclarations {
                        #(#declaration_return_fields),*
                    }
                }
            }
        }

        use #mod_ident::*;
    };

    Ok(code)
}
