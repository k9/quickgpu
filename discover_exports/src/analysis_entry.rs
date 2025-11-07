use syn::{
    Ident, ImplItemConst, Item, ItemEnum, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType,
    Visibility, token,
};

use crate::utils::id;

#[derive(Clone, Debug)]
pub struct AnalysisStruct {
    pub item: ItemStruct,
}

impl AnalysisStruct {
    pub fn new(mut item: ItemStruct) -> Self {
        item.ident = id("__dont_use__");
        Self { item }
    }
}

#[derive(Clone, Debug)]
pub struct AnalysisEnum {
    pub item: ItemEnum,
}

impl AnalysisEnum {
    pub fn new(mut item: ItemEnum) -> Self {
        item.ident = id("__dont_use__");
        Self { item }
    }
}

#[derive(Clone, Debug)]
pub struct AnalysisType {
    pub item: ItemType,
}

impl AnalysisType {
    pub fn new(mut item: ItemType) -> Self {
        item.ident = id("__dont_use__");
        Self { item }
    }
}

#[derive(Clone, Debug)]
pub struct AnalysisTrait {
    item: ItemTrait,
}

impl AnalysisTrait {
    pub fn new(mut item: ItemTrait) -> Self {
        item.ident = id("__dont_use__");
        Self { item }
    }
}

#[derive(Clone, Debug)]
pub struct AnalysisMod {
    pub item: ItemMod,
    pub root_of_crate: Option<Ident>,
}

impl AnalysisMod {
    pub fn new(mut item: ItemMod, root_of_crate: Option<Ident>) -> Self {
        item.ident = id("__dont_use__");

        Self {
            item,
            root_of_crate,
        }
    }

    pub fn content(&self) -> Vec<Item> {
        self.item.content.clone().map_or(vec![], |c| c.1)
    }
}

#[derive(Clone, Debug)]
pub struct AnalysisImpl {
    pub item: ItemImpl,
}

impl AnalysisImpl {
    pub fn new(item: ItemImpl) -> Self {
        Self { item }
    }
}

#[derive(Clone, Debug)]
pub struct AnalysisImplConst {
    pub item: ImplItemConst,
}

impl AnalysisImplConst {
    pub fn new(mut item: ImplItemConst) -> Self {
        item.ident = id("__dont_use__");

        Self { item }
    }
}

#[derive(Clone, Debug)]
pub enum AnalysisEntry {
    Struct(AnalysisStruct),
    Enum(AnalysisEnum),
    Type(AnalysisType),
    Trait(AnalysisTrait),
    Mod(AnalysisMod),
    Impl(AnalysisImpl),
    ImplConst(AnalysisImplConst),
    Variant,
}

impl AnalysisEntry {
    pub fn vis(&self) -> Visibility {
        match self {
            AnalysisEntry::Struct(entry) => entry.item.vis.clone(),
            AnalysisEntry::Enum(entry) => entry.item.vis.clone(),
            AnalysisEntry::Type(entry) => entry.item.vis.clone(),
            AnalysisEntry::Trait(entry) => entry.item.vis.clone(),
            AnalysisEntry::Mod(entry) => entry.item.vis.clone(),
            AnalysisEntry::ImplConst(entry) => entry.item.vis.clone(),
            AnalysisEntry::Impl(_) => Visibility::Public(token::Pub::default()),
            AnalysisEntry::Variant => Visibility::Public(token::Pub::default()),
        }
    }

    pub fn has_impls(&self) -> bool {
        match self {
            AnalysisEntry::Struct(_) => true,
            AnalysisEntry::Enum(_) => true,
            _ => false,
        }
    }
}
