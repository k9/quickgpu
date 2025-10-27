use std::collections::HashMap;

use crate::{AResult, utils::token_string};
use anyhow::{Context, bail};
use petgraph::{graph::NodeIndex, prelude::StableGraph};
use syn::{ItemEnum, ItemImpl, ItemMod, ItemStruct, ItemType};

pub struct AnalysisStruct {
    pub item: ItemStruct,
    pub impls: Vec<ItemImpl>,
}

pub struct AnalysisEnum {
    pub item: ItemEnum,
    pub impls: Vec<ItemImpl>,
}

#[derive(Default)]
pub struct Analysis {
    pub crates: HashMap<String, NodeIndex>,
    pub graph: StableGraph<AnalysisItem, AnalysisEdge>,
    pub structs: Vec<AnalysisStruct>,
    pub enums: Vec<AnalysisEnum>,
    pub types: Vec<ItemType>,
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
    Type(&'a ItemType),
    Impl(&'a ItemImpl),
    Mod(&'a ItemMod),
}

impl AnalysisItem {
    pub fn get_ref<'a>(&'a self, analysis: &'a Analysis) -> AResult<AnalysisRef<'a>> {
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
                if token_string(&struct_item.item.ident) == token_string(&ident) {
                    return Some((neighbor, AnalysisRef::Struct(struct_item)));
                }
            }
            Ok(AnalysisRef::Type(type_item)) => {
                if token_string(&type_item.ident) == token_string(&ident) {
                    return Some((neighbor, AnalysisRef::Type(type_item)));
                }
            }
            Ok(AnalysisRef::Mod(mod_item)) => {
                if token_string(&mod_item.ident) == token_string(&ident) {
                    return Some((neighbor, AnalysisRef::Mod(mod_item)));
                }
            }
            _ => (),
        }
    }

    None
}
