use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::parenthesized;

use crate::utils::err;

pub struct FieldEntry {
    pub binding: String,
    pub bind_group_layout_entry: TokenStream,
    pub bind_group_entry: TokenStream,
    pub resource_field: TokenStream,
    pub slice_field: TokenStream,
    pub resource_write: TokenStream,
    pub resource_write_fn: TokenStream,
    pub resource_init: TokenStream,
}

impl std::fmt::Debug for FieldEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FieldEntry")
            .field("binding", &self.binding.to_string())
            .field(
                "bind_group_layout_entry",
                &self.bind_group_layout_entry.to_string(),
            )
            .field("bind_group_entry", &self.bind_group_entry.to_string())
            .field("resource_field", &self.resource_field.to_string())
            .field("slice_field", &self.slice_field.to_string())
            .field("resource_write", &self.resource_write.to_string())
            .field("resource_write_fn", &self.resource_write_fn.to_string())
            .field("resource_init", &self.resource_init.to_string())
            .finish()
    }
}

pub fn process_field(
    f: &mut syn::Field,
    next_binding: &mut u32,
) -> Result<Option<FieldEntry>, TokenStream> {
    let mut ty = None;
    let mut stages: Option<TokenStream> = None;
    let mut usage: Option<TokenStream> = None;
    let mut binding_type: Option<TokenStream> = None;
    let field_ident = f.ident.clone().unwrap();

    let attr_index = f
        .attrs
        .iter()
        .position(|attr| attr.path().is_ident("qbind"));

    let Some(attr_index) = attr_index else {
        return Err(err(
            Span::call_site(),
            "Must have qbind attribute on each bind group field",
        ));
    };

    let attr = f.attrs.remove(attr_index);

    let _ = attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("binding") {
            let content;
            parenthesized!(content in meta.input);

            let content: u32 = content.to_string().parse().unwrap();
            *next_binding = content;
        }

        if meta.path.is_ident("ty") {
            let content;
            parenthesized!(content in meta.input);
            ty = Some(format!("{}", content))
        }

        if meta.path.is_ident("stages") {
            let content;
            parenthesized!(content in meta.input);
            stages = Some(content.parse().unwrap())
        }

        if meta.path.is_ident("usage") {
            let content;
            parenthesized!(content in meta.input);
            usage = Some(content.parse().unwrap())
        }

        if meta.path.is_ident("binding_type") {
            let content;
            parenthesized!(content in meta.input);
            binding_type = Some(content.parse().unwrap());
        }

        Ok(())
    });

    let Some(ty) = ty else {
        return Err(err(
            Span::call_site(),
            &format!("Must specify 'ty' on field '{}'", field_ident),
        ));
    };

    let Some(stages) = stages else {
        return Err(err(
            Span::call_site(),
            &format!("Must specify 'stages' on field '{}'", field_ident),
        ));
    };

    let Some(usage) = usage else {
        return Err(err(
            Span::call_site(),
            &format!("Must specify 'usage' on field '{}'", field_ident),
        ));
    };

    let Some(binding_type) = binding_type else {
        return Err(err(
            Span::call_site(),
            &format!("Must specify 'binding_type' on field '{}'", field_ident),
        ));
    };

    let bind_group_layout_entry = quote! {
        bind_group_layout_entry()
            .binding(#next_binding)
            .visibility(#stages)
            .ty(#binding_type)
    };

    let bind_group_entry = quote! {
        bind_group_entry().binding(#next_binding).resource(
            BindingResource::Buffer(buffer_binding().buffer(&resources.#field_ident).offset(0).build()),
        )
    };

    let binding = format!("@group(0) @binding({next_binding}) var<uniform> {field_ident}: {ty};");

    let resource_field = quote! {
        #field_ident: wgpu::Buffer
    };

    let field_ty = &f.ty;

    let slice_field = quote! {
        #field_ident: &'a [#field_ty]
    };

    let resource_init = quote! {
        #field_ident: buffer_init_descriptor(None)
            .contents(bytemuck::cast_slice(std::slice::from_ref(&data.#field_ident)))
            .usage(#usage)
            .create_with(device)
    };

    let resource_write = quote! {
        queue.write_buffer(&self.#field_ident, 0, bytemuck::cast_slice(std::slice::from_ref(&data.#field_ident)))
    };

    let resource_write_fn_ident = format_ident!("write_{}", field_ident);
    let resource_write_fn = quote! {
        pub fn #resource_write_fn_ident(&self, queue: &Queue, data: &#field_ty) {
            queue.write_buffer(&self.#field_ident, 0, bytemuck::cast_slice(std::slice::from_ref(data)))
        }
    };

    *next_binding += 1;

    let entry = FieldEntry {
        binding,
        bind_group_layout_entry,
        bind_group_entry,
        resource_field,
        slice_field,
        resource_write,
        resource_write_fn,
        resource_init,
    };

    log::debug!("field entry {:?}", entry);

    Ok(Some(entry))
}
