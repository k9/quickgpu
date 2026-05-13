use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::{bind_group_code::quickgpu_crate_name, utils::err};

pub struct FieldEntry {
    pub helper_field: TokenStream,
    pub helper_return_field: TokenStream,
    pub helper_new_arg: TokenStream,
    pub helper_layout_descriptor_entry: TokenStream,
    pub resource_field: TokenStream,
    pub offset_field: TokenStream,
    pub declaration_field: TokenStream,
    pub declaration_return_field: TokenStream,
    pub binding_entry: BindingEntry,
}

pub enum BindingEntry {
    Buffer {
        field: TokenStream,
        make: TokenStream,
    },
    Texture {
        entry: TokenStream,
    },
    Sampler {
        entry: TokenStream,
    },
}

pub struct ResourceSpecific {
    pub binding_entry: BindingEntry,
    pub resource_field: TokenStream,
}

impl std::fmt::Debug for FieldEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FieldEntry")
            .field("binding", &self.helper_field.to_string())
            .finish()
    }
}

pub fn process_field(f: &mut syn::Field, binding: u32) -> Result<Option<FieldEntry>, TokenStream> {
    let quickgpu = quickgpu_crate_name();
    let field_ident = f.ident.clone().unwrap();
    let ty = &f.ty;

    if let syn::Type::Path(inner) = ty
        && let Some(last) = inner.path.segments.last()
        && last.ident.to_string().ends_with("Bind")
    {
        let mut inner_ty = None;
        if let syn::PathArguments::AngleBracketed(args) = &last.arguments
            && args.args.len() == 1
        {
            inner_ty = Some(&args.args[0]);
        }

        let helper_field = quote! {
            pub #field_ident: #quickgpu::binder::#ty
        };

        let helper_return_field = quote! {
            #field_ident
        };

        let helper_new_arg = quote! {
            #field_ident: #quickgpu::binder::#ty
        };

        let helper_layout_descriptor_entry = quote! {
            #field_ident.layout_entry(#binding)
        };

        let ResourceSpecific {
            binding_entry,
            resource_field,
        } = if last.ident.to_string().starts_with("Buffer") {
            let field = quote! { pub #field_ident: Option<for<'a> fn(binding: u32, buffer: &'a wgpu::Buffer) -> wgpu::BindGroupEntry<'a>> };
            let make = quote! {
                #quickgpu::Nested::unnest(
                    (entries.#field_ident.unwrap_or(default_entry))(#binding, &resources.#field_ident.buffer)
                )
            };

            let resource_field = quote! {
                pub #field_ident: &'a #quickgpu::binder::BoundBuffer<#inner_ty>
            };

            ResourceSpecific {
                resource_field,
                binding_entry: BindingEntry::Buffer { field, make },
            }
        } else if last.ident.to_string().starts_with("Texture") {
            let entry = quote! {
                wgpu::BindGroupEntry {
                    binding: #binding,
                    resource: wgpu::BindingResource::TextureView(&resources.#field_ident.texture_view)
                }
            };

            let resource_field = quote! {
                pub #field_ident: &'a #quickgpu::binder::BoundTextureView<#inner_ty>
            };

            ResourceSpecific {
                resource_field,
                binding_entry: BindingEntry::Texture { entry },
            }
        } else if last.ident.to_string().starts_with("Sampler") {
            let entry = quote! {
                wgpu::BindGroupEntry {
                    binding: #binding,
                    resource: wgpu::BindingResource::Sampler(&resources.#field_ident.sampler)
                }
            };

            let resource_field = quote! {
                pub #field_ident: &'a #quickgpu::binder::BoundSampler<#inner_ty>
            };

            ResourceSpecific {
                resource_field,
                binding_entry: BindingEntry::Sampler { entry },
            }
        } else {
            return Err(err(Span::call_site(), "Unsupported bind type"));
        };

        let offset_field = quote! {
            pub #field_ident: wgpu::BufferAddress
        };

        let declaration_field = quote! {
            pub #field_ident: String
        };

        let declaration_return_field = quote! {
            #field_ident: #quickgpu::binder::Declarable::wgsl_declaration(
                &self.#field_ident,
                group,
                #binding
            )
        };

        let entry = FieldEntry {
            helper_field,
            helper_return_field,
            helper_new_arg,
            helper_layout_descriptor_entry,
            resource_field,
            binding_entry,
            offset_field,
            declaration_field,
            declaration_return_field,
        };

        log::debug!("field entry {:?}", entry);

        Ok(Some(entry))
    } else {
        Err(err(
            Span::call_site(),
            &format!("All fields on struct must be Bind {:?}", ty),
        ))
    }
}
