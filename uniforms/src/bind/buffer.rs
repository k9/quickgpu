use std::marker::PhantomData;

use quickgpu::{Nested, bind_group_entry, buffer_init_descriptor};
use wgpu::{
    BindGroupEntry, Buffer, BufferAddress, BufferBinding, BufferUsages, Device, Label, Queue,
};

use crate::bind::{BufferResource, Datalike};

pub type BufferBind<Data> = super::Bind<Data, BufferResource>;

impl<Data: Datalike> BufferBind<Data> {
    pub fn make_buffer<'a>(
        &self,
        label: Label<'a>,
        data: Data,
        usage: BufferUsages,
        device: &Device,
    ) -> BoundBuffer<Data> {
        BoundBuffer {
            buffer: buffer_init_descriptor(label)
                .contents(bytemuck::cast_slice(std::slice::from_ref(&data)))
                .usage(usage)
                .create_with(device),
            phantom: PhantomData,
        }
    }

    pub fn write(
        &self,
        queue: &Queue,
        buffer: &BoundBuffer<Data>,
        offset: Option<BufferAddress>,
        data: &Data,
    ) {
        queue.write_buffer(
            &buffer.buffer,
            offset.unwrap_or(0),
            bytemuck::cast_slice(std::slice::from_ref(data)),
        )
    }
}

impl<Data: Datalike> super::Declarable for BufferBind<Data> {
    fn wgsl_declaration(&self, group: u32, binding: u32) -> String {
        let wgsl_type = &self.wgsl_type;
        let wgsl_name = &self.wgsl_name;
        format!(
            "

@group({group})
@binding({binding})
var<uniform> {wgsl_name}: {wgsl_type};

            "
        )
    }
}

pub struct BoundBuffer<Data: Datalike> {
    pub buffer: Buffer,
    phantom: PhantomData<Data>,
}

impl<Data: Datalike> BoundBuffer<Data> {
    pub fn bind_group_entry<'a, F: Nested<BufferBinding<'a>>>(
        &'a self,
        binding: u32,
        make: impl Fn(&'a Buffer) -> F,
    ) -> BindGroupEntry<'a> {
        bind_group_entry()
            .binding(binding)
            .resource(wgpu::BindingResource::Buffer(make(&self.buffer).unnest()))
            .build()
    }
}
