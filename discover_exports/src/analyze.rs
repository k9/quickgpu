use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use crate::{
    AResult,
    utils::{dummy_module, token_string, write_expanded},
};
use anyhow::{Context, bail};
use petgraph::{algo::astar, graph::NodeIndex, prelude::StableGraph, visit::Bfs};
use proc_macro2::Span;
use syn::{Item, ItemMod, ItemStruct, ItemType, UseTree, Visibility, spanned::Spanned};

#[derive(Default)]
pub struct Analysis {
    pub crates: HashMap<String, NodeIndex>,
    pub graph: StableGraph<AnalysisItem, AnalysisEdge>,
    pub structs: Vec<ItemStruct>,
    pub types: Vec<ItemType>,
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
    Type(usize),
    Mod(usize),
    None,
}

#[derive(Clone, Copy)]
pub enum AnalysisRef<'a> {
    Struct(&'a ItemStruct),
    Type(&'a ItemType),
    Mod(&'a ItemMod),
}

impl AnalysisItem {
    fn get_ref<'a>(&'a self, analysis: &'a Analysis) -> AResult<AnalysisRef<'a>> {
        match self {
            AnalysisItem::Struct(id) => {
                if let Some(item) = analysis.structs.get(*id) {
                    return Ok(AnalysisRef::Struct(item));
                };
            }
            AnalysisItem::Type(id) => {
                if let Some(item) = analysis.types.get(*id) {
                    return Ok(AnalysisRef::Type(item));
                };
            }
            AnalysisItem::Mod(id) => {
                if let Some(item) = analysis.modules.get(*id) {
                    return Ok(AnalysisRef::Mod(item));
                };
            }
            _ => (),
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

pub fn parse_crate(
    analysis: &mut Analysis,
    output_path: PathBuf,
    crate_path: PathBuf,
    crate_name: &str,
) -> AResult<NodeIndex> {
    write_expanded(&output_path, &crate_path)?;

    let file = syn::parse_file(&std::fs::read_to_string(output_path)?)?;
    let crate_root = dummy_module(crate_name, file.items);
    let id = analysis.modules.push_index(crate_root);
    let root_index = analysis.graph.add_node(AnalysisItem::Mod(id));

    analysis.crates.insert(crate_name.to_string(), root_index);

    process_subtree(analysis, root_index)?;

    Ok(root_index)
}

pub fn process_subtree(analysis: &mut Analysis, parent_mod: NodeIndex) -> AResult<()> {
    let AnalysisRef::Mod(module) = AnalysisItem::node_index_ref(analysis, parent_mod)? else {
        bail!("Couldn't get node")
    };

    let Some((_, content)) = &module.content else {
        bail!("Couldn't get node")
    };

    let content = content.clone();

    for item in content.into_iter() {
        match item {
            Item::Mod(mod_item) => {
                let id = analysis.modules.push_index(mod_item);
                let child_mod = analysis.graph.add_node(AnalysisItem::Mod(id));
                analysis
                    .graph
                    .update_edge(parent_mod, child_mod, AnalysisEdge::Normal);

                process_subtree(analysis, child_mod)?;
            }
            Item::Struct(struct_item) => {
                let id = analysis.structs.push_index(struct_item.clone());
                let child = analysis.graph.add_node(AnalysisItem::Struct(id));
                analysis
                    .graph
                    .update_edge(parent_mod, child, AnalysisEdge::Normal);
            }
            Item::Type(type_item) => {
                let id = analysis.types.push_index(type_item.clone());
                let child = analysis.graph.add_node(AnalysisItem::Type(id));
                analysis
                    .graph
                    .update_edge(parent_mod, child, AnalysisEdge::Normal);
            }
            _ => (),
        };
    }

    Ok(())
}

pub fn process_use_statements(
    analysis: &mut Analysis,
    parent_mod: NodeIndex,
    skipped_mods: &mut HashSet<String>,
) -> AResult<()> {
    let mut bfs = Bfs::new(&analysis.graph, parent_mod);
    while let Some(node_index) = bfs.next(&analysis.graph) {
        if let AnalysisRef::Mod(module) = AnalysisItem::node_index_ref(analysis, node_index)? {
            let module = module.clone();
            if let Some((_, items)) = &module.content {
                for item in items {
                    if let Item::Use(use_statement) = item {
                        let use_statement = use_statement.clone();
                        process_use_tree(
                            analysis,
                            node_index,
                            node_index,
                            &use_statement.tree,
                            skipped_mods,
                        )?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn process_use_tree(
    analysis: &mut Analysis,
    from_module: NodeIndex,
    to_module: NodeIndex,
    use_subtree: &UseTree,
    skipped_mods: &mut HashSet<String>,
) -> AResult<()> {
    match &use_subtree {
        syn::UseTree::Path(use_path) => {
            if let Some((neighbor, AnalysisRef::Mod(_))) =
                find_neighbor(&analysis, to_module, &use_path.ident)
            {
                process_use_tree(
                    analysis,
                    from_module,
                    neighbor,
                    &use_path.tree,
                    skipped_mods,
                )?;
            } else if let Some(krate) = analysis.crates.get(&token_string(&use_path.ident)) {
                process_use_tree(analysis, from_module, *krate, &use_path.tree, skipped_mods)?;
            } else if token_string(&use_path.ident) == "super" {
                let parent = analysis
                    .graph
                    .neighbors_directed(to_module, petgraph::Direction::Incoming)
                    .next()
                    .context("Couldn't get parent")?;

                process_use_tree(analysis, from_module, parent, &use_path.tree, skipped_mods)?;
            } else {
                skipped_mods.insert(use_path.ident.to_string());
            }
        }
        syn::UseTree::Group(use_group) => {
            for item in &use_group.items {
                process_use_tree(analysis, from_module, to_module, item, skipped_mods)?;
            }
        }
        syn::UseTree::Name(use_name) => {
            if let Some((neighbor, AnalysisRef::Struct(_))) =
                find_neighbor(&analysis, to_module, &use_name.ident)
            {
                analysis
                    .graph
                    .update_edge(from_module, neighbor, AnalysisEdge::Normal);
            }
        }
        syn::UseTree::Glob(_) => {
            let neighbors = analysis.graph.neighbors(to_module).collect::<Vec<_>>();

            for neighbor in neighbors {
                analysis
                    .graph
                    .update_edge(from_module, neighbor, AnalysisEdge::Normal);
            }
        }
        syn::UseTree::Rename(use_rename) => {
            if let Some((neighbor, AnalysisRef::Struct(_))) =
                find_neighbor(&analysis, to_module, &use_rename.ident)
            {
                analysis.graph.update_edge(
                    from_module,
                    neighbor,
                    AnalysisEdge::Rename(use_rename.rename.to_string()),
                );
            }
        }
    };

    Ok(())
}

fn find_neighbor<'a>(
    analysis: &'a Analysis,
    item_tree_node: NodeIndex,
    ident: &syn::Ident,
) -> Option<(NodeIndex, AnalysisRef<'a>)> {
    let neighbors = analysis.graph.neighbors(item_tree_node);
    for neighbor in neighbors {
        match AnalysisItem::node_index_ref(analysis, neighbor) {
            Ok(AnalysisRef::Struct(struct_item)) => {
                if token_string(&struct_item.ident) == token_string(&ident) {
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

pub fn keep_only_pub(analysis: &mut Analysis, node_index: NodeIndex) -> AResult<()> {
    let to_keep = keep_only_gather(analysis, node_index)?;
    let node_indices = analysis.graph.node_indices().collect::<Vec<_>>();
    for node_index in node_indices {
        if !to_keep.contains(&node_index) {
            analysis.graph.remove_node(node_index);
        }
    }

    Ok(())
}

fn keep_only_gather(analysis: &mut Analysis, node_index: NodeIndex) -> AResult<Vec<NodeIndex>> {
    let mut to_keep = vec![node_index];
    let neighbors = analysis.graph.neighbors(node_index).collect::<Vec<_>>();

    for neighbor in neighbors {
        if let Some(item) = analysis.graph.node_weight(neighbor) {
            match item.get_ref(analysis) {
                Ok(AnalysisRef::Struct(struct_item)) => {
                    if matches!(struct_item.vis, Visibility::Public(_)) {
                        to_keep.push(neighbor);
                    }
                }
                Ok(AnalysisRef::Type(struct_item)) => {
                    if matches!(struct_item.vis, Visibility::Public(_)) {
                        to_keep.push(neighbor);
                    }
                }
                Ok(AnalysisRef::Mod(mod_item)) => {
                    if matches!(mod_item.vis, Visibility::Public(_)) {
                        let mut descendants = keep_only_gather(analysis, neighbor)?;
                        to_keep.append(&mut descendants);
                    }
                }
                _ => (),
            }
        }
    }

    Ok(to_keep)
}

#[derive(Debug, Clone)]
pub struct ExportedItem {
    pub path: Vec<String>,
    pub span: Span,
}

pub fn list_exports<'a>(
    analysis: &'a Analysis,
    root_index: NodeIndex,
    filter: impl Fn(AnalysisRef<'a>) -> bool,
) -> AResult<Vec<ExportedItem>> {
    let mut exports = vec![];
    let mut bfs = Bfs::new(&analysis.graph, root_index);
    while let Some(node_index) = bfs.next(&analysis.graph) {
        if let Some(export_item) = list_export_item(analysis, root_index, node_index, &filter)? {
            exports.push(export_item);
        };
    }

    exports.sort_by(|a, b| a.path.join("").cmp(&b.path.join("")));
    exports.sort_by(|a, b| a.path.len().cmp(&b.path.len()));

    Ok(exports)
}

fn list_export_item<'a>(
    analysis: &'a Analysis,
    root_index: NodeIndex,
    node_index: NodeIndex,
    filter: &impl Fn(AnalysisRef<'a>) -> bool,
) -> AResult<Option<ExportedItem>> {
    if !filter(AnalysisItem::node_index_ref(analysis, node_index)?) {
        return Ok(None);
    };

    let graph_path = astar(
        &analysis.graph,
        root_index,
        |x| x == node_index,
        |_| 1,
        |_| 0,
    )
    .context("Couldn't get path")?;

    let mut path = vec![];
    let mut previous_segment = None;
    let mut span = None;
    for segment_index in graph_path.1 {
        let mut name = match AnalysisItem::node_index_ref(analysis, segment_index)? {
            AnalysisRef::Struct(struct_item) => {
                span = Some(struct_item.span());
                struct_item.ident.to_string()
            }
            AnalysisRef::Type(type_item) => {
                span = Some(type_item.span());
                type_item.ident.to_string()
            }
            AnalysisRef::Mod(mod_item) => mod_item.ident.to_string(),
        };

        if let Some(previous_segment) = previous_segment
            && let Some(edge) = analysis.graph.find_edge(previous_segment, previous_segment)
            && let Some(AnalysisEdge::Rename(edge)) = analysis.graph.edge_weight(edge)
        {
            name = edge.clone();
        };

        path.push(name);

        previous_segment = Some(segment_index);
    }

    let Some(span) = span else {
        bail!("No span for exported item");
    };

    Ok(Some(ExportedItem { path, span }))
}
