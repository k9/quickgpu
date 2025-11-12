use std::collections::HashMap;

use anyhow::{Context, bail};
use fixedbitset::FixedBitSet;
use petgraph::{graph::NodeIndex, prelude::StableGraph, visit::Bfs};
use proc_macro2::Span;
use syn::{Ident, ItemMod, Path, Token, Visibility, token};

use crate::{
    AResult, EntryIndex,
    analysis_entry::{AnalysisEntry, AnalysisMod, AnalysisStruct},
    utils::id,
};

pub type AnalysisGraph = StableGraph<AnalysisEntry, AnalysisEdge>;

pub struct Analysis {
    graph: AnalysisGraph,
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

        let crate_root = self.graph.add_node(AnalysisEntry::Mod(AnalysisMod::new(
            ItemMod {
                attrs: vec![],
                vis: Visibility::Public(Token![pub](Span::call_site())),
                unsafety: None,
                mod_token: Token![mod](Span::call_site()),
                ident: id(&name),
                content: Some((token::Brace::default(), file.items)),
                semi: None,
            },
            Some(id(name)),
        )));

        Ok(Ctx {
            analysis: self,
            crate_root,
            public_paths: HashMap::new(),
            top_level_paths: HashMap::new(),
        })
    }
}

pub struct Ctx<'a> {
    pub analysis: &'a mut Analysis,
    pub crate_root: EntryIndex,
    pub public_paths: HashMap<NodeIndex, Path>,
    pub top_level_paths: HashMap<NodeIndex, Path>,
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

    pub fn struct_entry(&self, index: NodeIndex) -> AResult<&AnalysisStruct> {
        if let Ok(AnalysisEntry::Struct(entry)) = self
            .graph()
            .node_weight(index)
            .context("Couldn't get entry")
        {
            Ok(entry)
        } else {
            bail!("Couldn't get struct entry")
        }
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
