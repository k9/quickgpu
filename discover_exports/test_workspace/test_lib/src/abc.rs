pub struct Abc {
    pub z: String,
}

impl Abc {
    const XYZ: crate::util::A = crate::util::A {};
}

mod z {
    pub struct ZZ {}
}

pub use super::BigTypeAlias;
pub use tlt::{CounterB, CounterC};
pub use z::ZZ;

#[allow(dead_code)]
pub(crate) struct Def {
    pub def: String,
}
