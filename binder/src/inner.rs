use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, parenthesized, spanned::Spanned};

use crate::utils::err;

const PUBLIC_NAMED: &str = "BindGroup derive input must be struct with all public named fields.";

pub(crate) fn entry_point(input: DeriveInput) -> TokenStream {
    #[cfg(test)]
    env_logger::init();

    let ident = &input.ident;
    let input_span = input.span();

    let Data::Struct(data) = input.data else {
        return err(input_span, PUBLIC_NAMED);
    };

    let Fields::Named(mut fields) = data.fields else {
        return err(input_span, PUBLIC_NAMED);
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
            return err;
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

    let ident_buffers = format_ident!("{}Buffers", ident);
    let buffer_fields = field_entries.iter().map(|f| &f.buffer_field);

    let ident_slices = format_ident!("{}Slices", ident);
    let slice_fields = field_entries.iter().map(|f| &f.slice_field);

    let buffer_inits = field_entries.iter().map(|f| &f.buffer_init);
    let buffer_writes = field_entries.iter().map(|f| &f.buffer_write);
    let buffer_write_fns = field_entries.iter().map(|f| &f.buffer_write_fn);

    let ident_group = format_ident!("{}Group", ident);

    let code = quote! {
        pub struct #ident_buffers {
            #(#buffer_fields),*
        }

        pub struct #ident_slices<'a> {
            #(#slice_fields),*
        }

        pub struct #ident_group {
            pub bind_group: BindGroup,
            pub buffers: #ident_buffers,
        }

        impl #ident_group {
            pub fn write(&self, queue: &Queue, slices: #ident_slices) {
                #(#buffer_writes);*
            }

            #(#buffer_write_fns)*
        }

        impl #ident {
            pub const WGSL: &str = #bindings;

            pub fn layout(device: &Device) -> wgpu::BindGroupLayout {
                device.create_bind_group_layout(
                    &bind_group_layout_descriptor(Some("bgl0"))
                        .entries(&builders([
                            #(#bind_group_layout_entries),*
                        ]))
                        .build(),
                )
            }

            pub fn bind_group(
                device: &Device,
                bgl: &wgpu::BindGroupLayout,
                slices: &#ident_slices,
            ) -> #ident_group {
                let buffers = #ident_buffers {
                    #(#buffer_inits),*
                };

                let bind_group = device.create_bind_group(
                    &bind_group_descriptor(Some("bg"))
                        .layout(&bgl)
                        .entries(&builders([
                            #(#bind_group_entries),*
                        ]))
                        .build(),
                );

                #ident_group {
                    bind_group,
                    buffers,
                }
            }
        }
    };

    log::debug!("code {}", code);

    code
}

#[derive(derive_more::Debug)]
pub struct FieldEntry {
    #[debug("binding {binding}")]
    pub binding: String,
    #[debug("bind_group_layout_entry {bind_group_layout_entry}")]
    pub bind_group_layout_entry: TokenStream,
    #[debug("bind_group_entry {bind_group_entry}")]
    pub bind_group_entry: TokenStream,
    #[debug("buffer_field {buffer_field}")]
    pub buffer_field: TokenStream,
    #[debug("slice_field {slice_field}")]
    pub slice_field: TokenStream,
    #[debug("buffer_write {buffer_write}")]
    pub buffer_write: TokenStream,
    #[debug("buffer_write_fn {buffer_write_fn}")]
    pub buffer_write_fn: TokenStream,
    #[debug("buffer_init {buffer_init}")]
    pub buffer_init: TokenStream,
}

fn process_field(
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
            BindingResource::Buffer(buffer_binding().buffer(&buffers.#field_ident).offset(0).build()),
        )
    };

    let binding = format!("@group(0) @binding({next_binding}) var<uniform> {field_ident}: {ty};");

    let buffer_field = quote! {
        #field_ident: wgpu::Buffer
    };

    let field_ty = &f.ty;
    let slice_field = quote! {
        #field_ident: &'a [#field_ty]
    };

    let buffer_init = quote! {
        #field_ident: buffer_init_descriptor(None)
            .contents(bytemuck::cast_slice(slices.#field_ident))
            .usage(BufferUsages::UNIFORM | BufferUsages::COPY_DST)
            .create_with(device)
    };

    let buffer_write = quote! {
        queue.write_buffer(&self.buffers.#field_ident, 0, bytemuck::cast_slice(slices.#field_ident))
    };

    let buffer_write_fn_ident = format_ident!("write_{}", field_ident);
    let buffer_write_fn = quote! {
        pub fn #buffer_write_fn_ident(&self, queue: &Queue, data: &[#field_ty]) {
            queue.write_buffer(&self.buffers.#field_ident, 0, bytemuck::cast_slice(data))
        }
    };

    *next_binding += 1;

    let entry = FieldEntry {
        binding,
        bind_group_layout_entry,
        bind_group_entry,
        buffer_field,
        slice_field,
        buffer_write,
        buffer_write_fn,
        buffer_init,
    };

    log::debug!("field entry {:?}", entry);

    Ok(Some(entry))
}

#[cfg(test)]
mod tests {
    use crate::inner::entry_point;
    use crate::utils::prettyprint;
    use syn::parse_quote;

    #[test]
    fn simple() {
        println!(
            "\n{}\n",
            prettyprint(entry_point(parse_quote! {
                #[derive(QBind)]
                struct A {
                    #[qbind(ty(vec4<f32>))]
                    pub x: Vector4<f32>,
                    #[qbind(ty(u32))]
                    pub yyy: u32
                }
            }))
            .replace(
                "\\n", "
"
            )
        );
    }
}
