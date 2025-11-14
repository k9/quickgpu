pub mod builders;
pub mod nested;

use std::marker::PhantomData;

pub struct Unset<T>(PhantomData<T>);
pub struct Optional<T>(PhantomData<T>);
pub struct Set<T>(T);

pub trait GetValue<T> {
    fn get_value(self) -> Option<T>;
}

impl<T> GetValue<T> for Optional<T> {
    fn get_value(self) -> Option<T> {
        None
    }
}

impl<T> GetValue<T> for Set<T> {
    fn get_value(self) -> Option<T> {
        Some(self.0)
    }
}

pub trait Nested<T> {
    fn unnest(self) -> T;
}

impl Nested<u32> for u32 {
    fn unnest(self) -> u32 {
        self
    }
}

#[derive(Debug)]
pub struct Tcbib<B, L> {
    pub buffer: B,
    pub layout: L,
}

pub fn tcbib_builder<B>() -> Tcbib<Unset<B>, Unset<wgpu::TexelCopyBufferLayout>> {
    Tcbib {
        buffer: Unset(PhantomData),
        layout: Unset(PhantomData),
    }
}

impl<BufferType, LayoutType> Tcbib<Unset<BufferType>, LayoutType> {
    pub fn buffer(self, buffer: BufferType) -> Tcbib<Set<BufferType>, LayoutType> {
        Tcbib {
            buffer: Set(buffer),
            layout: self.layout,
        }
    }
}

impl<BufferType, LayoutType> Tcbib<BufferType, Unset<LayoutType>> {
    pub fn layout(self, layout: LayoutType) -> Tcbib<BufferType, Set<LayoutType>> {
        Tcbib {
            buffer: self.buffer,
            layout: Set(layout),
        }
    }
}

impl<BufferType, LayoutType: Nested<wgpu::TexelCopyBufferLayout>>
    Tcbib<Set<BufferType>, Set<LayoutType>>
{
    pub fn build(self) -> wgpu::TexelCopyBufferInfoBase<BufferType> {
        wgpu::TexelCopyBufferInfoBase {
            buffer: self.buffer.0,
            layout: self.layout.0.unnest(),
        }
    }
}
