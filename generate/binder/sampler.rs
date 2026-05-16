use wgpu::{BindGroupEntry, BindingResource, Sampler};

use crate::bind_group_entry;
use crate::binder::{Bind, Declarable, SamplerResource};

pub type SamplerBind = Bind<(), SamplerResource>;

impl SamplerBind {
    pub fn make_sampler(&self, sampler: Sampler) -> BoundSampler {
        BoundSampler { sampler }
    }
}

impl Declarable for SamplerBind {
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

pub struct BoundSampler {
    pub sampler: Sampler,
}

impl BoundSampler {
    pub fn bind_group_entry<'a>(&'a self, binding: u32) -> BindGroupEntry<'a> {
        bind_group_entry()
            .binding(binding)
            .resource(BindingResource::Sampler(&self.sampler))
            .build()
    }
}
