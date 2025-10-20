pub extern crate test_lib_types as tlt;
pub type BigTypeAlias = tlt::BigType;

impl Default for BigTypeAlias {}

pub mod abc;

mod util;
pub use util::*;
