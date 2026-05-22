#![doc=include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/INTRO.md"))]

#[cfg(feature = "wgpu_27")]
pub mod wgpu_27;

#[cfg(feature = "wgpu_28")]
pub mod wgpu_28;
