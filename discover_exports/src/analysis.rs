use std::collections::HashMap;

use crate::AResult;
use anyhow::{Context, bail};
use petgraph::{graph::NodeIndex, prelude::StableGraph};
use syn::{ItemEnum, ItemImpl, ItemMod, ItemStruct, ItemType};

#[derive(Clone)]
pub struct AnalysisStruct {
    pub item: ItemStruct,
    pub impls: Vec<ItemImpl>,
    pub path: Vec<String>,
}

#[derive(Clone)]
pub struct AnalysisEnum {
    pub item: ItemEnum,
    pub impls: Vec<ItemImpl>,
    pub path: Vec<String>,
}

#[derive(Clone)]
pub struct AnalysisTypeAlias {
    pub item: ItemType,
    pub impls: Vec<ItemImpl>,
    pub path: Vec<String>,
}

pub trait HasPath {
    fn get_path(&self) -> &[String];
}

impl HasPath for AnalysisStruct {
    fn get_path(&self) -> &[String] {
        &self.path
    }
}

impl HasPath for AnalysisEnum {
    fn get_path(&self) -> &[String] {
        &self.path
    }
}

impl HasPath for AnalysisTypeAlias {
    fn get_path(&self) -> &[String] {
        &self.path
    }
}

#[derive(Default)]
pub struct Analysis {
    pub crates: HashMap<String, NodeIndex>,
    pub graph: StableGraph<AnalysisEntry, AnalysisEdge>,
    pub structs: Vec<AnalysisStruct>,
    pub enums: Vec<AnalysisEnum>,
    pub types: Vec<AnalysisTypeAlias>,
    pub impls: Vec<ItemImpl>,
    pub modules: Vec<ItemMod>,
}

#[derive(Debug)]
pub struct AnalysisEdge {
    pub from_use_statement: bool,
    pub rename: Option<String>,
}

impl AnalysisEdge {
    pub fn new(from_use_statement: bool, rename: Option<String>) -> Self {
        Self {
            from_use_statement,
            rename,
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
            AnalysisEntry::Impl(id) => {
                if let Some(item) = analysis.impls.get(*id) {
                    return Ok(AnalysisRef::Impl(item));
                };
            }
            AnalysisEntry::Mod(id) => {
                if let Some(item) = analysis.modules.get(*id) {
                    return Ok(AnalysisRef::Mod(item));
                };
            }
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
            AnalysisEntry::Impl(id) => {
                if let Some(item) = analysis.impls.get_mut(*id) {
                    return Ok(AnalysisRefMut::Impl(item));
                };
            }
            AnalysisEntry::Mod(id) => {
                if let Some(item) = analysis.modules.get_mut(*id) {
                    return Ok(AnalysisRefMut::Mod(item));
                };
            }
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

pub fn find_neighbor<'a>(
    analysis: &'a Analysis,
    entry_tree_node: NodeIndex,
    ident: &syn::Ident,
) -> Option<NodeIndex> {
    let neighbors = analysis.graph.neighbors(entry_tree_node);
    for neighbor in neighbors {
        match AnalysisEntry::node_index_ref(analysis, neighbor) {
            Ok(AnalysisRef::Struct(struct_entry)) => {
                if &struct_entry.item.ident.to_string() == &ident.to_string() {
                    return Some(neighbor);
                }
            }
            Ok(AnalysisRef::Enum(enum_entry)) => {
                if &enum_entry.item.ident.to_string() == &ident.to_string() {
                    return Some(neighbor);
                }
            }
            Ok(AnalysisRef::Type(type_entry)) => {
                if &type_entry.item.ident.to_string() == &ident.to_string() {
                    return Some(neighbor);
                }
            }
            Ok(AnalysisRef::Mod(mod_entry)) => {
                if &mod_entry.ident.to_string() == &ident.to_string() {
                    return Some(neighbor);
                }
            }
            _ => (),
        }
    }

    None
}
