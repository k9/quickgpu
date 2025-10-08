use rustdoc_types::{Crate, Id, Item};

pub struct Data {
    pub base: Crate,
    pub wgt: Crate,
}

#[derive(Debug)]
pub struct DataItem<'a> {
    pub id: &'a Id,
    pub item: &'a Item,
    pub krate: &'a Crate,
}

impl<'a> Data {
    pub fn new(base: Crate, wgt: Crate) -> Self {
        Self { base, wgt }
    }

    pub fn iter_base(&'a self) -> impl Iterator<Item = DataItem<'a>> {
        self.base.index.iter().map(|item| DataItem {
            id: item.0,
            item: item.1,
            krate: &self.base,
        })
    }

    pub fn iter_wgt(&'a self) -> impl Iterator<Item = DataItem<'a>> {
        self.wgt.index.iter().map(|item| DataItem {
            id: item.0,
            item: item.1,
            krate: &self.wgt,
        })
    }

    pub fn iter_both(&'a self) -> impl Iterator<Item = DataItem<'a>> {
        self.iter_base().chain(self.iter_wgt())
    }
}
