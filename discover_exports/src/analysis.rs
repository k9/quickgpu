use anyhow::Context;
use fixedbitset::FixedBitSet;
use petgraph::{graph::NodeIndex, prelude::StableGraph, visit::Bfs};
use proc_macro2::Span;
use syn::{
    Attribute, Fields, Generics, Ident, ImplItemConst, ImplRestriction, Item, ItemEnum, ItemImpl,
    ItemMod, ItemStruct, ItemTrait, ItemType, Token, TraitItem, Type, TypeParamBound, Variant,
    Visibility,
};

use crate::{AResult, EntryIndex, utils::id};

#[derive(Clone, Debug)]
pub struct AnalysisCrate {
    pub content: Vec<Item>,
    pub vis: Visibility,
    pub name: Ident,
}

impl AnalysisCrate {
    pub fn new(name: Ident, vis: Visibility, content: Vec<Item>) -> Self {
        Self { name, vis, content }
    }
}

#[derive(Clone, Debug)]
pub struct AnalysisMod {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub content: Vec<Item>,
}

impl AnalysisMod {
    pub fn new(
        ItemMod {
            attrs,
            vis,
            content,
            ..
        }: ItemMod,
    ) -> Self {
        Self {
            attrs,
            vis,
            content: content.map_or(vec![], |c| c.1),
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

pub type AnalysisGraph = StableGraph<AnalysisEntry, AnalysisEdge>;

pub struct Analysis {
    pub graph: AnalysisGraph,
}

impl Default for Analysis {
    fn default() -> Self {
        Self {
            graph: AnalysisGraph::new(),
        }
    }
}

impl Analysis {
    pub fn add_crate(&'_ mut self, name: String, contents: String) -> AResult<Ctx<'_>> {
        let file = syn::parse_file(&contents)?;

        let crate_root = self
            .graph
            .add_node(AnalysisEntry::ExternCrate(AnalysisCrate {
                name: id(name.as_str()),
                vis: syn::Visibility::Public(Token![pub](Span::call_site())),
                content: file.items,
            }));

        Ok(Ctx {
            analysis: self,
            crate_root,
        })
    }
}

pub struct Ctx<'a> {
    pub analysis: &'a mut Analysis,
    pub crate_root: EntryIndex,
}

impl<'a> Ctx<'a> {
    pub fn graph(&self) -> &AnalysisGraph {
        &self.analysis.graph
    }
    pub fn graph_mut(&mut self) -> &mut AnalysisGraph {
        &mut self.analysis.graph
    }

    pub fn entry(&self, index: NodeIndex) -> AResult<&AnalysisEntry> {
        self.graph()
            .node_weight(index)
            .context("Couldn't get entry")
    }

    pub fn entry_mut(&mut self, index: NodeIndex) -> AResult<&mut AnalysisEntry> {
        self.graph_mut()
            .node_weight_mut(index)
            .context("Couldn't get entry")
    }

    pub fn krate(&self) -> AResult<&AnalysisEntry> {
        self.entry(self.crate_root)
    }

    pub fn bfs(&self) -> AResult<Bfs<NodeIndex, FixedBitSet>> {
        Ok(Bfs::new(&self.graph(), self.crate_root))
    }
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
    ExternCrate(AnalysisCrate),
    Struct(AnalysisStruct),
    Enum(AnalysisEnum),
    Type(AnalysisTypeAlias),
    Trait(AnalysisTrait),
    Mod(AnalysisMod),
    Variant,
    Const(AnalysisConst),
}
