#![doc=include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/INTRO.md"))]

// wgpu 29 needs this to be referenced
extern crate alloc;

#[cfg(feature = "v27")]
pub mod v27;

#[cfg(feature = "v27")]
pub extern crate wgpu_27;

#[cfg(feature = "v28")]
pub mod v28;

#[cfg(feature = "v28")]
pub extern crate wgpu_28;

#[cfg(feature = "v29")]
pub mod v29;

#[cfg(feature = "v29")]
pub extern crate wgpu_29;
