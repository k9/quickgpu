use std::marker::PhantomData;

use wgpu::{
    BindGroupEntry, BindingResource, Extent3d, Queue, TexelCopyTextureInfo, Texture, TextureView,
    TextureViewDescriptor,
};

use crate::binder::{Bind, Datalike, Declarable, TextureResource};
use crate::{bind_group_entry, texel_copy_buffer_layout, Nested};

pub type TextureBind<Data> = Bind<Data, TextureResource>;

impl<Data: Datalike> TextureBind<Data> {
    pub fn make_view(
        &self,
        texture: &Texture,
        view_descriptor: &TextureViewDescriptor,
    ) -> BoundTextureView<Data> {
        BoundTextureView {
            texture_view: texture.create_view(view_descriptor),
            phantom: PhantomData,
        }
    }

    pub fn write<'a>(
        &self,
        queue: &Queue,
        texture: &Texture,
        info: impl Nested<TexelCopyTextureInfo<'a>>,
        data: &Data,
        size: Option<Extent3d>,
    ) {
        queue.write_texture(
            info.unnest(),
            bytemuck::cast_slice(std::slice::from_ref(data)),
            texel_copy_buffer_layout()
                .bytes_per_row(texture.size().width)
                .build(),
            size.unwrap_or(texture.size()),
        );
    }
}

impl<Data: Datalike> Declarable for TextureBind<Data> {
    fn wgsl_declaration(&self, group: u32, binding: u32) -> String {
        let wgsl_type = &self.wgsl_type;
        let wgsl_name = &self.wgsl_name;
        format!(
            "

@group({group})
@binding({binding})
var {wgsl_name}: {wgsl_type};

            "
        )
    }
}

pub struct BoundTextureView<Data: Datalike> {
    pub texture_view: TextureView,
    phantom: PhantomData<Data>,
}

impl<Data: Datalike> BoundTextureView<Data> {
    pub fn bind_group_entry<'a>(&'a self, binding: u32) -> BindGroupEntry<'a> {
        bind_group_entry()
            .binding(binding)
            .resource(BindingResource::TextureView(&self.texture_view))
            .build()
    }
}
