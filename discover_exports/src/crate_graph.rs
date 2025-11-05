use anyhow::Context;
use petgraph::{
    Direction,
    algo::astar,
    dot::{Config, Dot},
    graph::{EdgeIndex, NodeIndex},
    visit::{Bfs, EdgeRef, Walker},
};
use quote::quote as q;
use syn::{Ident, ImplItem, ImplItemConst};

use crate::{
    AResult, AnalysisEdge, AnalysisStruct,
    analysis::{Analysis, AnalysisEntry, AnalysisMod},
    utils::IsPublic,
};

pub fn filter_map_nodes<T>(
    analysis: &Analysis,
    filter_fn: impl FnMut(NodeIndex) -> Option<T>,
) -> impl Iterator<Item = T> {
    Bfs::new(&analysis.graph, analysis.root_index)
        .iter(&analysis.graph)
        .filter_map(filter_fn)
}

pub fn for_each_node(analysis: &Analysis, item_fn: impl FnMut(NodeIndex)) {
    Bfs::new(&analysis.graph, analysis.root_index)
        .iter(&analysis.graph)
        .for_each(item_fn)
}

pub fn get_entry(analysis: &Analysis, node_index: NodeIndex) -> AResult<&AnalysisEntry> {
    analysis
        .graph
        .node_weight(node_index)
        .context("Couldn't get node weight")
}

pub fn get_struct(analysis: &Analysis, node_index: NodeIndex) -> AResult<Option<&AnalysisStruct>> {
    if let AnalysisEntry::Struct(entry) = get_entry(analysis, node_index)? {
        Ok(Some(entry))
    } else {
        Ok(None)
    }
}
pub fn get_mod(analysis: &Analysis, node_index: NodeIndex) -> AResult<Option<&AnalysisMod>> {
    if let AnalysisEntry::Mod(entry) = get_entry(analysis, node_index)? {
        Ok(Some(entry))
    } else {
        Ok(None)
    }
}

pub fn get_entry_mut(
    analysis: &mut Analysis,
    node_index: NodeIndex,
) -> AResult<&mut AnalysisEntry> {
    analysis
        .graph
        .node_weight_mut(node_index)
        .context("Couldn't get node weight")
}

pub fn get_struct_const<'a>(
    entry: &'a AnalysisStruct,
    ident: &'a Ident,
) -> Option<&'a ImplItemConst> {
    for impl_ in &entry.impls {
        for item in &impl_.items {
            if let ImplItem::Const(c) = item {
                if ident.to_string() == c.ident.to_string() {
                    return Some(c);
                }
            }
        }
    }

    None
}

pub fn find_neighbor<'a>(
    analysis: &'a Analysis,
    entry_tree_node: NodeIndex,
    ident: &syn::Ident,
) -> Option<NodeIndex> {
    let mut neighbors = analysis.graph.neighbors(entry_tree_node).detach();
    while let Some((edge_index, neighbor)) = neighbors.next(&analysis.graph) {
        if let Some(edge) = analysis.graph.edge_weight(edge_index)
            && let Some(name) = edge.name.as_ref()
            && name.to_string() == ident.to_string()
        {
            return Some(neighbor);
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

        if edge.from_hierarchy == existing.from_hierarchy {
            log::debug!("already-existing edge type {:?} to {:?}", existing, edge);
            return existing_index.id();
        }
    }

    analysis.graph.add_edge(from, to, edge)
}

pub fn item_path<'a>(analysis: &'a Analysis, node_index: NodeIndex) -> AResult<Vec<Ident>> {
    let graph_path = astar(
        &analysis.graph,
        analysis.root_index,
        |x| x == node_index,
        |_| 1,
        |_| 0,
    )
    .context(format!("Couldn't get path {:?}", node_index))?;

    let mut path = vec![];
    let mut previous_segment = None;
    for node_index in graph_path.1.iter() {
        if let Some(from_index) = previous_segment {
            if let Some(edge) = analysis
                .graph
                .edges_connecting(from_index, *node_index)
                .next()
            {
                if let Some(name) = &edge.weight().name {
                    path.push(name.clone());
                }
            }
        } else {
            let edge = analysis
                .graph
                .edges_directed(analysis.root_index, Direction::Incoming)
                .next()
                .unwrap()
                .weight();

            path.push(edge.name.clone().unwrap());
        }

        previous_segment = Some(*node_index);
    }

    Ok(path)
}

pub fn keep_only_public<'a>(analysis: &'a mut Analysis) -> AResult<()> {
    let to_remove = keep_only_public_recurse(analysis, analysis.root_index)?;

    for index in to_remove {
        analysis.graph.remove_node(index);
    }

    Ok(())
}

pub fn keep_only_public_recurse<'a>(
    analysis: &'a mut Analysis,
    current: NodeIndex,
) -> AResult<Vec<NodeIndex>> {
    let mut neighbors = analysis.graph.neighbors(current).detach();
    let mut visited = vec![];
    let mut to_remove = vec![];
    while let Some((_, node_index)) = neighbors.next(&analysis.graph) {
        if visited.contains(&node_index) {
            continue;
        } else {
            visited.push(node_index);
        }

        let (public, recurse) = match get_entry(analysis, node_index)? {
            AnalysisEntry::Struct(entry) => (entry.vis.is_public(), true),
            AnalysisEntry::Enum(entry) => (entry.vis.is_public(), true),
            AnalysisEntry::Type(entry) => (entry.vis.is_public(), true),
            AnalysisEntry::Trait(entry) => (entry.vis.is_public(), true),
            AnalysisEntry::Mod(entry) => {
                // If this is a crate root, but not this crate's root, skip it
                (
                    entry.vis.is_public(),
                    !(entry.crate_root.is_some() && node_index != analysis.root_index),
                )
            }
            AnalysisEntry::Variant => (true, true),
            AnalysisEntry::Const(entry) => (entry.vis.is_public(), true),
            AnalysisEntry::Origin => (true, true),
        };

        if !public {
            to_remove.push(node_index);
        } else if recurse {
            to_remove.append(&mut keep_only_public_recurse(analysis, node_index)?);
        }
    }

    Ok(to_remove)
}

pub fn node_ident(analysis: &Analysis, index: NodeIndex) -> AResult<Ident> {
    item_path(&analysis, index)?
        .last()
        .context("Invalid path")
        .cloned()
}

#[allow(dead_code)]
pub fn print_dot(analysis: &Analysis) {
    println!(
        "{:?}",
        Dot::with_attr_getters(
            &analysis.graph,
            &[Config::EdgeNoLabel, Config::NodeNoLabel],
            &|_, edge| {
                let label = edge
                    .weight()
                    .name
                    .as_ref()
                    .map_or("".to_string(), |x| x.to_string());

                format!("label = \"{}\"", label)
            },
            &|_, (_, entry)| {
                let label = match entry {
                    AnalysisEntry::Struct(entry) => {
                        let vis = &entry.vis;
                        q!(#vis struct).to_string()
                    }
                    AnalysisEntry::Const(entry) => {
                        let vis = &entry.vis;
                        q!(#vis const).to_string()
                    }
                    AnalysisEntry::Enum(entry) => {
                        let vis = &entry.vis;
                        q!(#vis enum).to_string()
                    }
                    AnalysisEntry::Type(entry) => {
                        let vis = &entry.vis;
                        q!(#vis type).to_string()
                    }
                    AnalysisEntry::Trait(entry) => {
                        let vis = &entry.vis;
                        q!(#vis trait).to_string()
                    }
                    AnalysisEntry::Mod(entry) => {
                        let vis = &entry.vis;
                        q!(#vis mod).to_string()
                    }
                    AnalysisEntry::Variant => "variant".to_string(),
                    AnalysisEntry::Origin => "origin".to_string(),
                };

                format!("label = \"{}\"", label)
            }
        )
    );
}
