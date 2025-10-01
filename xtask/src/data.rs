use rustdoc_types::{Crate, Id, Item};

pub struct Data {
    pub base: Crate,
    pub wgt: Crate,
}

pub trait ItemIter<'a>: Iterator<Item = (&'a Id, &'a Item)> {}

impl<'a, T> ItemIter<'a> for T where T: Iterator<Item = (&'a Id, &'a Item)> {}

impl<'a> Data {
    pub fn new(base: Crate, wgt: Crate) -> Self {
        Self { base, wgt }
    }

    pub fn iter_base(&'a self) -> impl ItemIter<'a> {
        self.base.index.iter()
    }

    pub fn iter_wgt(&'a self) -> impl ItemIter<'a> {
        self.wgt.index.iter()
    }

    #[allow(dead_code)]
    pub fn iter_both(&self) -> impl Iterator<Item = ((&Id, &Item), &Crate)> {
        self.base
            .index
            .iter()
            .map(|item| (item, &self.base))
            .chain(self.wgt.index.iter().map(|item| (item, &self.wgt)))
    }
}
