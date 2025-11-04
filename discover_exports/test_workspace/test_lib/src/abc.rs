pub struct Abc {
    pub z: String,
}

impl Abc {
    const XYZ: i8 = 4;
}

mod z {
    pub struct ZZ {}
}

pub use super::BigTypeAlias;
pub use tlt::{CounterB, CounterC};
pub use z::ZZ;
pub extern crate test_lib_types as tlt;

#[allow(dead_code)]
pub(crate) struct Def {
    pub def: String,
}
