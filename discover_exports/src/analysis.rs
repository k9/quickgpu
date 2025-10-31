use std::collections::HashMap;

use crate::AResult;
use anyhow::{Context, bail};
use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    prelude::StableGraph,
    visit::EdgeRef,
};
use syn::{Ident, ItemEnum, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType};

#[derive(Clone, Debug, Default)]
pub struct CrateRoot {
    pub extern_prelude: HashMap<String, NodeIndex>,
}

#[derive(Clone, Debug)]
pub struct AnalysisMod {
    pub item: ItemMod,
    pub crate_root: Option<CrateRoot>,
}

#[derive(Clone, Debug)]
pub struct AnalysisStruct {
    pub item: ItemStruct,
    pub impls: Vec<ItemImpl>,
    pub path: Vec<Ident>,
}

#[derive(Clone, Debug)]
pub struct AnalysisEnum {
    pub item: ItemEnum,
    pub impls: Vec<ItemImpl>,
    pub path: Vec<Ident>,
}

#[derive(Clone, Debug)]
pub struct AnalysisTypeAlias {
    pub item: ItemType,
    pub impls: Vec<ItemImpl>,
    pub path: Vec<Ident>,
}

#[derive(Clone, Debug)]
pub struct AnalysisTrait {
    pub item: ItemTrait,
    pub path: Vec<Ident>,
}

pub trait HasPath {
    fn get_path(&self) -> &[Ident];
}

impl HasPath for AnalysisStruct {
    fn get_path(&self) -> &[Ident] {
        &self.path
    }
}

impl HasPath for AnalysisEnum {
    fn get_path(&self) -> &[Ident] {
        &self.path
    }
}

impl HasPath for AnalysisTypeAlias {
    fn get_path(&self) -> &[Ident] {
        &self.path
    }
}

impl HasPath for AnalysisTrait {
    fn get_path(&self) -> &[Ident] {
        &self.path
    }
}

#[derive(Default)]
pub struct Analysis {
    pub graph: StableGraph<AnalysisEntry, AnalysisEdge>,
    pub structs: Vec<AnalysisStruct>,
    pub enums: Vec<AnalysisEnum>,
    pub types: Vec<AnalysisTypeAlias>,
    pub traits: Vec<AnalysisTrait>,
    pub impls: Vec<ItemImpl>,
    pub modules: Vec<AnalysisMod>,
}

#[derive(Debug, Clone)]
pub struct AnalysisEdge {
    pub from_use_statement: bool,
    pub from_extern_crate: bool,
    pub rename: Option<Ident>,
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
    Impl(usize),
    Mod(usize),
    None,
}

#[derive(Clone, Copy, Debug)]
pub enum AnalysisRef<'a> {
    Struct(&'a AnalysisStruct),
    Enum(&'a AnalysisEnum),
    Type(&'a AnalysisTypeAlias),
    Trait(&'a AnalysisTrait),
    Impl(&'a ItemImpl),
    Mod(&'a AnalysisMod),
}

impl<'a> AnalysisRef<'a> {
    pub fn ident(&self) -> Option<&Ident> {
        match self {
            AnalysisRef::Struct(entry) => Some(&entry.item.ident),
            AnalysisRef::Enum(entry) => Some(&entry.item.ident),
            AnalysisRef::Type(entry) => Some(&entry.item.ident),
            AnalysisRef::Trait(entry) => Some(&entry.item.ident),
            AnalysisRef::Mod(entry) => Some(&entry.item.ident),
            _ => None,
        }
    }
}

pub enum AnalysisRefMut<'a> {
    Struct(&'a mut AnalysisStruct),
    Enum(&'a mut AnalysisEnum),
    Type(&'a mut AnalysisTypeAlias),
    Trait(&'a mut AnalysisTrait),
    Impl(&'a mut ItemImpl),
    Mod(&'a mut AnalysisMod),
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
            AnalysisEntry::Trait(id) => {
                if let Some(item) = analysis.traits.get_mut(*id) {
                    return Ok(AnalysisRefMut::Trait(item));
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
    let mut neighbors = analysis.graph.neighbors(entry_tree_node).detach();
    while let Some((edge_index, neighbor)) = neighbors.next(&analysis.graph) {
        if let Some(neighbor_ident) = match AnalysisEntry::node_index_ref(analysis, neighbor) {
            Ok(AnalysisRef::Struct(entry)) => Some(&entry.item.ident),
            Ok(AnalysisRef::Enum(entry)) => Some(&entry.item.ident),
            Ok(AnalysisRef::Type(entry)) => Some(&entry.item.ident),
            Ok(AnalysisRef::Trait(entry)) => Some(&entry.item.ident),
            Ok(AnalysisRef::Mod(entry)) => Some(&entry.item.ident),
            _ => None,
        } {
            let edge = analysis.graph.edge_weight(edge_index)?;

            let neighbor_ident = edge.rename.as_ref().unwrap_or(neighbor_ident);
            if neighbor_ident.to_string() == ident.to_string() {
                return Some(neighbor);
            }
        }
    }

    None
}

pub fn update_edge<'a>(
    analysis: &'a mut Analysis,
    from: NodeIndex,
    to: NodeIndex,
    edge: AnalysisEdge,
) -> EdgeIndex {
    let mut connecting = analysis.graph.edges_connecting(from, to);
    while let Some(existing_index) = connecting.next() {
        let existing = existing_index.weight();
        if edge.from_use_statement == existing.from_use_statement
            && edge.from_extern_crate == existing.from_extern_crate
        {
            log::warn!("already-existing edge type {:?} to {:?}", existing, edge);
            return existing_index.id();
        }
    }

    analysis.graph.add_edge(from, to, edge)
}
