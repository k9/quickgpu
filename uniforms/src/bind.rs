pub mod buffer;
pub mod sampler;
pub mod texture;

pub use buffer::*;
pub use sampler::*;
pub use texture::*;

use bytemuck::NoUninit;
use quickgpu::bind_group_layout_entry;
use std::marker::PhantomData;
use wgpu::{BindGroupLayoutEntry, BindingType, ShaderStages};

pub trait Datalike: Copy + Clone + NoUninit {}
impl<T> Datalike for T where T: Copy + Clone + NoUninit {}

pub struct Bind<Data: Datalike, Resource: Resourcelike> {
    pub ty: BindingType,
    pub visibility: ShaderStages,
    pub wgsl_type: String,
    pub wgsl_name: String,
    phantom: PhantomData<(Data, Resource)>,
}

impl<Data: Datalike, Resource: Resourcelike> Bind<Data, Resource> {
    pub fn new(
        ty: BindingType,
        visibility: ShaderStages,
        wgsl_type: impl Into<String>,
        wgsl_name: impl Into<String>,
    ) -> Self {
        Self {
            ty,
            visibility,
            wgsl_type: wgsl_type.into(),
            wgsl_name: wgsl_name.into(),
            phantom: PhantomData,
        }
    }

    pub fn layout_entry(&self, binding: u32) -> BindGroupLayoutEntry {
        bind_group_layout_entry()
            .ty(self.ty)
            .visibility(self.visibility)
            .binding(binding)
            .build()
    }
}

pub trait Resourcelike {}
pub struct BufferResource;
impl Resourcelike for BufferResource {}
pub struct TextureResource;
impl Resourcelike for TextureResource {}
pub struct SamplerResource;
impl Resourcelike for SamplerResource {}

pub trait Declarable {
    fn wgsl_declaration(&self, group: u32, binding: u32) -> String;
}
