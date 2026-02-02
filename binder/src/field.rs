use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::parenthesized;

use crate::utils::err;

#[derive(derive_more::Debug)]
pub struct FieldEntry {
    #[debug("binding {binding}")]
    pub binding: String,
    #[debug("bind_group_layout_entry {bind_group_layout_entry}")]
    pub bind_group_layout_entry: TokenStream,
    #[debug("bind_group_entry {bind_group_entry}")]
    pub bind_group_entry: TokenStream,
    #[debug("resource_field {resource_field}")]
    pub resource_field: TokenStream,
    #[debug("data_field {data_field}")]
    pub data_field: TokenStream,
    #[debug("slice_field {slice_field}")]
    pub slice_field: TokenStream,
    #[debug("slice_of_data_field {slice_of_data_field}")]
    pub slice_of_data_field: TokenStream,
    #[debug("resource_write {resource_write}")]
    pub resource_write: TokenStream,
    #[debug("buffer_write_fn {resource_write_fn}")]
    pub resource_write_fn: TokenStream,
    #[debug("resource_init {resource_init}")]
    pub resource_init: TokenStream,
}

pub fn process_field(
    f: &mut syn::Field,
    next_binding: &mut u32,
) -> Result<Option<FieldEntry>, TokenStream> {
    let mut binding = "".to_string();
    let mut ty = None;
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

        binding = format!("@binding({})", *next_binding);

        if meta.path.is_ident("ty") {
            let content;
            parenthesized!(content in meta.input);
            ty = Some(format!("{}", content))
        }

        Ok(())
    });

    let Some(ty) = ty else {
        return Err(err(
            Span::call_site(),
            &format!("Must specify 'ty' on field '{}'", field_ident),
        ));
    };

    let bind_group_layout_entry = quote! {
        bind_group_layout_entry()
            .binding(#next_binding)
            .visibility(ShaderStages::FRAGMENT)
            .ty(wgpu::BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            })
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

    let data_field = quote! {
        #field_ident: Vec<#field_ty>
    };

    let slice_of_data_field = quote! {
        #field_ident: &self.#field_ident
    };

    let slice_field = quote! {
        #field_ident: &'a [#field_ty]
    };

    let resource_init = quote! {
        #field_ident: buffer_init_descriptor(None)
            .contents(bytemuck::cast_slice(slices.#field_ident))
            .usage(BufferUsages::UNIFORM | BufferUsages::COPY_DST)
            .create_with(device)
    };

    let resource_write = quote! {
        queue.write_buffer(&self.#field_ident, 0, bytemuck::cast_slice(slices.#field_ident))
    };

    let resource_write_fn_ident = format_ident!("write_{}", field_ident);
    let resource_write_fn = quote! {
        pub fn #resource_write_fn_ident(&self, queue: &Queue, data: &[#field_ty]) {
            queue.write_buffer(&self.#field_ident, 0, bytemuck::cast_slice(data))
        }
    };

    *next_binding += 1;

    let entry = FieldEntry {
        binding,
        bind_group_layout_entry,
        bind_group_entry,
        resource_field,
        data_field,
        slice_field,
        slice_of_data_field,
        resource_write,
        resource_write_fn,
        resource_init,
    };

    log::debug!("field entry {:?}", entry);

    Ok(Some(entry))
}
