#![doc=include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/INTRO.md"))]

#[cfg(feature = "v27")]
pub mod v27;

#[cfg(feature = "v27")]
pub extern crate wgpu_27;

#[cfg(feature = "v28")]
pub mod v28;

#[cfg(feature = "v28")]
pub extern crate wgpu_28;
