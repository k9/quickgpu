use std::collections::HashMap;

use crate::AResult;
use anyhow::{Context, bail};
use petgraph::{graph::NodeIndex, prelude::StableGraph};
use syn::{ItemEnum, ItemImpl, ItemMod, ItemStruct, ItemType};

#[derive(Debug, Clone)]
pub struct AnalysisStruct {
    pub item: ItemStruct,
    pub impls: Vec<ItemImpl>,
}

#[derive(Debug, Clone)]
pub struct AnalysisEnum {
    pub item: ItemEnum,
    pub impls: Vec<ItemImpl>,
}

#[derive(Debug, Clone)]
pub struct AnalysisTypeAlias {
    pub item: ItemType,
    pub impls: Vec<ItemImpl>,
}

#[derive(Default)]
pub struct Analysis {
    pub crates: HashMap<String, NodeIndex>,
    pub graph: StableGraph<AnalysisItem, AnalysisEdge>,
    pub structs: Vec<AnalysisStruct>,
    pub enums: Vec<AnalysisEnum>,
    pub types: Vec<AnalysisTypeAlias>,
    pub impls: Vec<ItemImpl>,
    pub modules: Vec<ItemMod>,
}

#[derive(Debug)]
pub enum AnalysisEdge {
    Normal,
    Rename(String),
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
pub enum AnalysisItem {
    Struct(usize),
    Enum(usize),
    Type(usize),
    Impl(usize),
    Mod(usize),
    None,
}

#[derive(Clone, Copy)]
pub enum AnalysisRef<'a> {
    Struct(&'a AnalysisStruct),
    Enum(&'a AnalysisEnum),
    Type(&'a AnalysisTypeAlias),
    Impl(&'a ItemImpl),
    Mod(&'a ItemMod),
}

pub enum AnalysisRefMut<'a> {
    Struct(&'a mut AnalysisStruct),
    Enum(&'a mut AnalysisEnum),
    Type(&'a mut AnalysisTypeAlias),
    Impl(&'a mut ItemImpl),
    Mod(&'a mut ItemMod),
}

impl AnalysisItem {
    pub fn get_ref<'a, 'b>(&'b self, analysis: &'a Analysis) -> AResult<AnalysisRef<'a>> {
        match self {
            AnalysisItem::Struct(id) => {
                if let Some(item) = analysis.structs.get(*id) {
                    return Ok(AnalysisRef::Struct(item));
                };
            }
            AnalysisItem::Enum(id) => {
                if let Some(item) = analysis.enums.get(*id) {
                    return Ok(AnalysisRef::Enum(item));
                };
            }
            AnalysisItem::Type(id) => {
                if let Some(item) = analysis.types.get(*id) {
                    return Ok(AnalysisRef::Type(item));
                };
            }
            AnalysisItem::Impl(id) => {
                if let Some(item) = analysis.impls.get(*id) {
                    return Ok(AnalysisRef::Impl(item));
                };
            }
            AnalysisItem::Mod(id) => {
                if let Some(item) = analysis.modules.get(*id) {
                    return Ok(AnalysisRef::Mod(item));
                };
            }
            AnalysisItem::None => (),
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
            AnalysisItem::Struct(id) => {
                if let Some(item) = analysis.structs.get_mut(*id) {
                    return Ok(AnalysisRefMut::Struct(item));
                };
            }
            AnalysisItem::Enum(id) => {
                if let Some(item) = analysis.enums.get_mut(*id) {
                    return Ok(AnalysisRefMut::Enum(item));
                };
            }
            AnalysisItem::Type(id) => {
                if let Some(item) = analysis.types.get_mut(*id) {
                    return Ok(AnalysisRefMut::Type(item));
                };
            }
            AnalysisItem::Impl(id) => {
                if let Some(item) = analysis.impls.get_mut(*id) {
                    return Ok(AnalysisRefMut::Impl(item));
                };
            }
            AnalysisItem::Mod(id) => {
                if let Some(item) = analysis.modules.get_mut(*id) {
                    return Ok(AnalysisRefMut::Mod(item));
                };
            }
            AnalysisItem::None => (),
        };

        bail!("Couldn't get AnalysisItem ref")
    }

    pub fn node_index_ref_mut<'a>(
        analysis: &'a mut Analysis,
        node_index: NodeIndex,
    ) -> AResult<AnalysisRefMut<'a>> {
        let item = analysis
            .graph
            .node_weight(node_index)
            .context("Couldn't get node")?;

        let item = item.clone();

        item.get_ref_mut(analysis)
    }
}

pub fn find_neighbor<'a>(
    analysis: &'a Analysis,
    item_tree_node: NodeIndex,
    ident: &syn::Ident,
) -> Option<(NodeIndex, AnalysisRef<'a>)> {
    let neighbors = analysis.graph.neighbors(item_tree_node);
    for neighbor in neighbors {
        match AnalysisItem::node_index_ref(analysis, neighbor) {
            Ok(AnalysisRef::Struct(struct_item)) => {
                if &struct_item.item.ident.to_string() == &ident.to_string() {
                    return Some((neighbor, AnalysisRef::Struct(struct_item)));
                }
            }
            Ok(AnalysisRef::Enum(enum_item)) => {
                if &enum_item.item.ident.to_string() == &ident.to_string() {
                    return Some((neighbor, AnalysisRef::Enum(enum_item)));
                }
            }
            Ok(AnalysisRef::Type(type_item)) => {
                if &type_item.item.ident.to_string() == &ident.to_string() {
                    return Some((neighbor, AnalysisRef::Type(type_item)));
                }
            }
            Ok(AnalysisRef::Mod(mod_item)) => {
                if &mod_item.ident.to_string() == &ident.to_string() {
                    return Some((neighbor, AnalysisRef::Mod(mod_item)));
                }
            }
            _ => (),
        }
    }

    None
}
