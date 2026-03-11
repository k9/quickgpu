use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};

use crate::utils::err;

pub struct FieldEntry {
    pub helper_field: TokenStream,
    pub helper_return_field: TokenStream,
    pub helper_new_arg: TokenStream,
    pub helper_layout_descriptor_entry: TokenStream,
    pub helper_descriptor_entry: TokenStream,
    pub resource_field: TokenStream,
    pub offset_field: TokenStream,
    pub declaration_field: TokenStream,
    pub declaration_return_field: TokenStream,
}

impl std::fmt::Debug for FieldEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FieldEntry")
            .field("binding", &self.helper_field.to_string())
            .finish()
    }
}

pub fn process_field(f: &mut syn::Field, binding: u32) -> Result<Option<FieldEntry>, TokenStream> {
    let field_ident = f.ident.clone().unwrap();
    let ty = &f.ty;

    if let syn::Type::Path(inner) = ty
        && let Some(last) = inner.path.segments.last()
        && last.ident.to_string().ends_with("Bind")
        && let syn::PathArguments::AngleBracketed(args) = &last.arguments
        && args.args.len() == 1
    {
        let inner_ty = &args.args[0];

        let helper_field = quote! {
            pub #field_ident: #ty
        };
        let helper_return_field = quote! {
            #field_ident
        };

        let helper_new_arg = quote! {
            #field_ident: #ty
        };

        let helper_layout_descriptor_entry = quote! {
            #field_ident.layout_entry(#binding)
        };

        let (bound_ident, helper_descriptor_entry) = if last.ident.to_string().starts_with("Buffer")
        {
            (
                format_ident!("BoundBuffer"),
                quote! {
                    buffers.#field_ident.bind_group_entry(
                        #binding,
                        offsets.map_or(0, |offsets| offsets.#field_ident)
                    )
                },
            )
        } else if last.ident.to_string().starts_with("Texture") {
            (
                format_ident!("BoundTextureView"),
                quote! {
                    buffers.#field_ident.bind_group_entry(
                        #binding,
                    )
                },
            )
        } else {
            return Err(err(Span::call_site(), "Unsupported bind type"));
        };

        let buffer_field = quote! {
            pub #field_ident: &'a #bound_ident<#inner_ty>
        };

        let offset_field = quote! {
            pub #field_ident: BufferAddress
        };

        let declaration_field = quote! {
            pub #field_ident: String
        };

        let declaration_return_field = quote! {
            #field_ident: self.#field_ident.wgsl_declaration(group, #binding)
        };

        let entry = FieldEntry {
            helper_field,
            helper_return_field,
            helper_new_arg,
            helper_layout_descriptor_entry,
            helper_descriptor_entry,
            resource_field: buffer_field,
            offset_field,
            declaration_field,
            declaration_return_field,
        };

        log::debug!("field entry {:?}", entry);

        Ok(Some(entry))
    } else {
        Err(err(Span::call_site(), "All fields on struct must be Bind"))
    }
}
