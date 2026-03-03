use quickgpu::bind_group_descriptor;
use wgpu::{BindGroup, BindGroupEntry, BindGroupLayout, Device, Label};

use crate::bind::{BindBuffer, Datalike};

pub struct GroupBuilder<'a> {
    label: Label<'a>,
    layout: &'a BindGroupLayout,
    entries: Vec<BindGroupEntry<'a>>,
}

impl<'a> GroupBuilder<'a> {
    pub fn entry<Data: Datalike>(&'a mut self, bind: &'a BindBuffer<Data>) -> &'a mut Self {
        self.entries
            .push(bind.bind_group_entry(self.entries.len() as u32));

        self
    }

    pub fn make(&'a self, device: &'a Device) -> BindGroup {
        device.create_bind_group(
            &bind_group_descriptor(self.label)
                .entries(&self.entries)
                .layout(self.layout)
                .build(),
        )
    }
}

pub fn group_builder<'a>(label: Label<'a>, layout: &'a BindGroupLayout) -> GroupBuilder<'a> {
    GroupBuilder {
        label,
        layout,
        entries: vec![],
    }
}
