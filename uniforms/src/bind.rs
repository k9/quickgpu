use std::marker::PhantomData;

use bytemuck::NoUninit;
use quickgpu::{bind_group_entry, bind_group_layout_entry, buffer_binding, buffer_init_descriptor};
use wgpu::{
    BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferUsages, Device, Queue,
    ShaderStages,
};

pub trait Datalike: Copy + Clone + NoUninit {}
impl<T> Datalike for T where T: Copy + Clone + NoUninit {}

pub struct Bind<Data: Datalike> {
    pub ty: BindingType,
    pub visibility: ShaderStages,
    pub usage: BufferUsages,
    pub wgsl_type: String,
    pub wgsl_name: String,
    phantom: PhantomData<Data>,
}

impl<Data: Datalike> Bind<Data> {
    pub fn new(
        ty: BindingType,
        visibility: ShaderStages,
        usage: BufferUsages,
        wgsl_type: String,
        wgsl_name: String,
    ) -> Self {
        Bind {
            ty,
            visibility,
            usage,
            wgsl_type,
            wgsl_name,
            phantom: PhantomData,
        }
    }

    pub fn make_buffer(&self, data: Data, device: &Device) -> BindBuffer<Data> {
        BindBuffer {
            buffer: buffer_init_descriptor(None)
                .contents(bytemuck::cast_slice(std::slice::from_ref(&data)))
                .usage(self.usage)
                .create_with(device),
            phantom: PhantomData,
        }
    }

    pub fn layout_entry(&self, binding: u32) -> BindGroupLayoutEntry {
        bind_group_layout_entry()
            .binding(binding)
            .visibility(self.visibility)
            .ty(self.ty)
            .build()
    }

    pub fn wgsl_declaration(&self, group: u32, binding: u32) -> String {
        let wgsl_type = &self.wgsl_type;
        let wgsl_name = &self.wgsl_name;
        format!(
            "

            @group({group}) @binding({binding})
            var<uniform> {wgsl_name}: {wgsl_type};

            "
        )
    }

    pub fn write(&self, queue: &Queue, buffer: &BindBuffer<Data>, data: &Data) {
        queue.write_buffer(
            &buffer.buffer,
            0,
            bytemuck::cast_slice(std::slice::from_ref(data)),
        )
    }
}

pub struct BindBuffer<Data: Datalike> {
    pub buffer: Buffer,
    phantom: PhantomData<Data>,
}

impl<Data: Datalike> BindBuffer<Data> {
    pub fn bind_group_entry<'a>(&'a self, binding: u32) -> wgpu::BindGroupEntry<'a> {
        bind_group_entry()
            .binding(binding)
            .resource(BindingResource::Buffer(
                buffer_binding().buffer(&self.buffer).offset(0).build(),
            ))
            .build()
    }
}
