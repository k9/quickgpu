use std::collections::HashMap;

use petgraph::{graph::NodeIndex, prelude::StableGraph};
use syn::{
    Attribute, Fields, Generics, Ident, ImplItemConst, ImplRestriction, Item, ItemEnum, ItemImpl,
    ItemMod, ItemStruct, ItemTrait, ItemType, TraitItem, Type, TypeParamBound, Variant, Visibility,
};

#[derive(Clone, Debug, Default)]
pub struct CrateRoot {
    pub extern_prelude: HashMap<String, NodeIndex>,
}

#[derive(Clone, Debug)]
pub struct AnalysisMod {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub content: Vec<Item>,
    pub crate_root: Option<CrateRoot>,
}

impl AnalysisMod {
    pub fn new(
        ItemMod {
            attrs,
            vis,
            content,
            ..
        }: ItemMod,
        crate_root: Option<CrateRoot>,
    ) -> Self {
        Self {
            attrs,
            vis,
            content: content.map_or(vec![], |c| c.1),
            crate_root,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AnalysisStruct {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub generics: Generics,
    pub fields: Fields,
    pub impls: Vec<ItemImpl>,
}

impl AnalysisStruct {
    pub fn new(
        ItemStruct {
            attrs,
            vis,
            generics,
            fields,
            ..
        }: ItemStruct,
        impls: Vec<ItemImpl>,
    ) -> Self {
        Self {
            attrs,
            vis,
            generics,
            fields,
            impls,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AnalysisConst {
    pub vis: Visibility,
}

impl AnalysisConst {
    pub fn new(ImplItemConst { vis, .. }: ImplItemConst) -> Self {
        Self { vis }
    }
}

#[derive(Clone, Debug)]
pub struct AnalysisEnum {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub generics: Generics,
    pub variants: Vec<Variant>,
    pub impls: Vec<ItemImpl>,
}

impl AnalysisEnum {
    pub fn new(
        ItemEnum {
            attrs,
            vis,
            generics,
            variants,
            ..
        }: ItemEnum,
        impls: Vec<ItemImpl>,
    ) -> Self {
        Self {
            attrs,
            vis,
            generics,
            variants: variants.into_iter().collect(),
            impls,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AnalysisTypeAlias {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub generics: Generics,
    pub ty: Box<Type>,
}

impl AnalysisTypeAlias {
    pub fn new(
        ItemType {
            attrs,
            vis,
            generics,
            ty,
            ..
        }: ItemType,
    ) -> Self {
        Self {
            attrs,
            vis,
            generics,
            ty,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AnalysisTrait {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub restriction: Option<ImplRestriction>,
    pub generics: Generics,
    pub supertraits: Vec<TypeParamBound>,
    pub items: Vec<TraitItem>,
}

impl AnalysisTrait {
    pub fn new(
        ItemTrait {
            attrs,
            vis,
            restriction,
            generics,
            supertraits,
            items,
            ..
        }: ItemTrait,
    ) -> Self {
        Self {
            attrs,
            vis,
            restriction,
            generics,
            supertraits: supertraits.into_iter().collect(),
            items,
        }
    }
}

#[derive(Default)]
pub struct Analysis {
    pub graph: StableGraph<AnalysisEntry, AnalysisEdge>,
    pub root_index: NodeIndex,
}

#[derive(Debug, Clone)]
pub struct AnalysisEdge {
    pub from_hierarchy: bool,
    pub name: Option<Ident>,
}

impl AnalysisEdge {
    pub fn new(from_hierarchy: bool, rename: Option<Ident>) -> Self {
        Self {
            from_hierarchy,
            name: rename,
        }
    }
}

pub trait VecPushIndex<T> {
    fn push_index(&mut self, item: T) -> usize;
}

impl<T> VecPushIndex<T> for Vec<T> {
    fn push_index(&mut self, item: T) -> usize {
        self.push(item);
        self.len() - 1
    }
}

#[derive(Clone, Debug)]
pub enum AnalysisEntry {
    Struct(AnalysisStruct),
    Enum(AnalysisEnum),
    Type(AnalysisTypeAlias),
    Trait(AnalysisTrait),
    Mod(AnalysisMod),
    Variant,
    Const(AnalysisConst),
    Origin,
}
