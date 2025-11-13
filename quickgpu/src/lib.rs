use wgpu::GlBackendOptions;

use crate::builders::GlBackendOptionsBuilder;

pub mod builders;
pub mod nested;

pub trait Nested<T> {
    fn unnest(self) -> T;
}

impl<T, N: Nested<T>> Nested<Option<T>> for Option<N> {
    fn unnest(self) -> Option<T> {
        self.map(|t| t.unnest())
    }
}

pub enum NestedGlBackendOptions {
    Base(GlBackendOptions),
    Builder(GlBackendOptionsBuilder),
}

impl NestedGlBackendOptions {
    pub fn unnest(self) -> GlBackendOptions {
        match self {
            NestedGlBackendOptions::Base(base) => base,
            NestedGlBackendOptions::Builder(builder) => builder.build(),
        }
    }
}

impl Default for NestedGlBackendOptions {
    fn default() -> Self {
        NestedGlBackendOptions::Base(GlBackendOptions::default())
    }
}
