use syn::{
    Ident, ImplItemConst, ImplItemFn, Item, ItemEnum, ItemImpl, ItemMod, ItemStruct, ItemTrait,
    ItemType, Variant, Visibility,
    token::{self, Pub},
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
pub struct AnalysisVariant {
    pub vis: Visibility,
    pub item: Variant,
}

impl AnalysisVariant {
    pub fn new(mut item: Variant) -> Self {
        item.ident = id("__dont_use__");
        Self {
            item,
            vis: Visibility::Public(token::Pub::default()),
        }
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

    pub fn content(&self) -> Option<&Vec<Item>> {
        self.item.content.as_ref().map(|c| &c.1)
    }
}

#[derive(Clone, Debug)]
pub struct AnalysisImpl {
    pub item: ItemImpl,
    pub vis: Visibility,
}

impl AnalysisImpl {
    pub fn new(item: ItemImpl) -> Self {
        Self {
            item,
            vis: Visibility::Public(token::Pub::default()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AnalysisImplConst {
    pub item: ImplItemConst,
}

impl AnalysisImplConst {
    pub fn new(mut item: ImplItemConst, from_trait: bool) -> Self {
        item.ident = id("__dont_use__");
        if from_trait {
            item.vis = Visibility::Public(Pub::default());
        }

        Self { item }
    }
}

#[derive(Clone, Debug)]
pub struct AnalysisImplFn {
    pub item: ImplItemFn,
}

impl AnalysisImplFn {
    pub fn new(mut item: ImplItemFn, from_trait: bool) -> Self {
        if from_trait {
            item.vis = Visibility::Public(Pub::default());
        }

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
    ImplFn(AnalysisImplFn),
    Variant(AnalysisVariant),
}

impl AnalysisEntry {
    pub fn vis(&self) -> &Visibility {
        match self {
            AnalysisEntry::Struct(entry) => &entry.item.vis,
            AnalysisEntry::Enum(entry) => &entry.item.vis,
            AnalysisEntry::Type(entry) => &entry.item.vis,
            AnalysisEntry::Trait(entry) => &entry.item.vis,
            AnalysisEntry::Mod(entry) => &entry.item.vis,
            AnalysisEntry::ImplConst(entry) => &entry.item.vis,
            AnalysisEntry::ImplFn(entry) => &entry.item.vis,
            AnalysisEntry::Impl(entry) => &entry.vis,
            AnalysisEntry::Variant(entry) => &entry.vis,
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
