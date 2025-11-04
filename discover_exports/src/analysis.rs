use std::collections::HashMap;

use crate::AResult;
use anyhow::{Context, bail};
use petgraph::{graph::NodeIndex, prelude::StableGraph};
use syn::{
    Attribute, Fields, Generics, Ident, ImplRestriction, Item, ItemEnum, ItemImpl, ItemMod,
    ItemStruct, ItemTrait, ItemType, TraitItem, Type, TypeParamBound, Variant, Visibility,
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
    pub structs: Vec<AnalysisStruct>,
    pub enums: Vec<AnalysisEnum>,
    pub types: Vec<AnalysisTypeAlias>,
    pub traits: Vec<AnalysisTrait>,
    pub modules: Vec<AnalysisMod>,
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

#[derive(Clone, Copy, Debug)]
pub enum AnalysisEntry {
    Struct(usize),
    Enum(usize),
    Type(usize),
    Trait(usize),
    Mod(usize),
    Origin,
    None,
}

#[derive(Clone, Copy, Debug)]
pub enum AnalysisRef<'a> {
    Struct(&'a AnalysisStruct),
    Enum(&'a AnalysisEnum),
    Type(&'a AnalysisTypeAlias),
    Trait(&'a AnalysisTrait),
    Mod(&'a AnalysisMod),
    Origin,
}

pub enum AnalysisRefMut<'a> {
    Struct(&'a mut AnalysisStruct),
    Enum(&'a mut AnalysisEnum),
    Type(&'a mut AnalysisTypeAlias),
    Trait(&'a mut AnalysisTrait),
    Mod(&'a mut AnalysisMod),
    Origin,
}

impl AnalysisEntry {
    pub fn get_ref<'a, 'b>(&'b self, analysis: &'a Analysis) -> AResult<AnalysisRef<'a>> {
        match self {
            AnalysisEntry::Struct(id) => {
                if let Some(item) = analysis.structs.get(*id) {
                    return Ok(AnalysisRef::Struct(item));
                };
            }
            AnalysisEntry::Enum(id) => {
                if let Some(item) = analysis.enums.get(*id) {
                    return Ok(AnalysisRef::Enum(item));
                };
            }
            AnalysisEntry::Type(id) => {
                if let Some(item) = analysis.types.get(*id) {
                    return Ok(AnalysisRef::Type(item));
                };
            }
            AnalysisEntry::Trait(id) => {
                if let Some(item) = analysis.traits.get(*id) {
                    return Ok(AnalysisRef::Trait(item));
                };
            }
            AnalysisEntry::Mod(id) => {
                if let Some(item) = analysis.modules.get(*id) {
                    return Ok(AnalysisRef::Mod(item));
                };
            }
            AnalysisEntry::Origin => return Ok(AnalysisRef::Origin),
            AnalysisEntry::None => (),
        };

        bail!("Couldn't get AnalysisItem ref")
    }

    pub fn node_index_ref<'a>(
        analysis: &'a Analysis,
        node_index: NodeIndex,
    ) -> AResult<AnalysisRef<'a>> {
        analysis
            .graph
            .node_weight(node_index)
            .context("Couldn't get node")?
            .get_ref(analysis)
    }

    pub fn get_ref_mut<'a, 'b>(
        &'b self,
        analysis: &'a mut Analysis,
    ) -> AResult<AnalysisRefMut<'a>> {
        match self {
            AnalysisEntry::Struct(id) => {
                if let Some(item) = analysis.structs.get_mut(*id) {
                    return Ok(AnalysisRefMut::Struct(item));
                };
            }
            AnalysisEntry::Enum(id) => {
                if let Some(item) = analysis.enums.get_mut(*id) {
                    return Ok(AnalysisRefMut::Enum(item));
                };
            }
            AnalysisEntry::Type(id) => {
                if let Some(item) = analysis.types.get_mut(*id) {
                    return Ok(AnalysisRefMut::Type(item));
                };
            }
            AnalysisEntry::Trait(id) => {
                if let Some(item) = analysis.traits.get_mut(*id) {
                    return Ok(AnalysisRefMut::Trait(item));
                };
            }
            AnalysisEntry::Mod(id) => {
                if let Some(item) = analysis.modules.get_mut(*id) {
                    return Ok(AnalysisRefMut::Mod(item));
                };
            }
            AnalysisEntry::Origin => (),
            AnalysisEntry::None => (),
        };

        bail!("Couldn't get AnalysisItem ref")
    }

    pub fn node_index_ref_mut<'a>(
        analysis: &'a mut Analysis,
        node_index: NodeIndex,
    ) -> AResult<AnalysisRefMut<'a>> {
        let entry = analysis
            .graph
            .node_weight(node_index)
            .context("Couldn't get node")?;

        let entry = entry.clone();

        entry.get_ref_mut(analysis)
    }
}
