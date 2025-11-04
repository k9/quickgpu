use anyhow::Context;
use petgraph::{
    algo::{astar, has_path_connecting},
    dot::{Config, Dot},
    graph::{EdgeIndex, NodeIndex},
    visit::{Bfs, EdgeRef, Walker},
};
use quote::quote as q;
use syn::Ident;

use crate::{
    AResult, AnalysisEdge,
    analysis::{Analysis, AnalysisEntry, AnalysisRef},
    utils::IsPublic,
};

#[allow(dead_code)]
pub fn filter_map_nodes<T>(
    analysis: &Analysis,
    root_index: NodeIndex,
    filter_fn: impl FnMut(NodeIndex) -> Option<T>,
) -> impl Iterator<Item = T> {
    Bfs::new(&analysis.graph, root_index)
        .iter(&analysis.graph)
        .filter_map(filter_fn)
}

#[allow(dead_code)]
pub fn for_each_node(analysis: &Analysis, root_index: NodeIndex, item_fn: impl FnMut(NodeIndex)) {
    Bfs::new(&analysis.graph, root_index)
        .iter(&analysis.graph)
        .for_each(item_fn)
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
            log::warn!("already-existing edge type {:?} to {:?}", existing, edge);
            return existing_index.id();
        }
    }

    analysis.graph.add_edge(from, to, edge)
}

pub fn item_path<'a>(
    analysis: &'a Analysis,
    root_index: NodeIndex,
    node_index: NodeIndex,
) -> AResult<Vec<Ident>> {
    let graph_path = astar(
        &analysis.graph,
        root_index,
        |x| x == node_index,
        |_| 1,
        |_| 0,
    )
    .context(format!("Couldn't get path {:?}", node_index))?;

    let mut path = vec![];
    let mut previous_segment = None;
    for node_index in graph_path.1.iter() {
        let from_index = match previous_segment {
            Some(index) => index,
            None => analysis
                .graph
                .edges_directed(*node_index, petgraph::Direction::Incoming)
                .next()
                .map(|edge| edge.source())
                .context("Can't get path edge")?,
        };

        if let Some(edge) = analysis
            .graph
            .edges_connecting(from_index, *node_index)
            .next()
        {
            if let Some(name) = &edge.weight().name {
                path.push(name.clone());
            }
        }

        previous_segment = Some(*node_index);
    }

    Ok(path)
}

#[allow(dead_code)]
pub fn keep_only_public<'a>(analysis: &'a mut Analysis, root_index: NodeIndex) -> AResult<()> {
    let indices = analysis.graph.node_indices().collect::<Vec<_>>();
    for node_index in indices {
        let public = match AnalysisEntry::node_index_ref(analysis, node_index)? {
            AnalysisRef::Struct(entry) => entry.vis.is_public(),
            AnalysisRef::Enum(entry) => entry.vis.is_public(),
            AnalysisRef::Type(entry) => entry.vis.is_public(),
            AnalysisRef::Trait(entry) => entry.vis.is_public(),
            AnalysisRef::Mod(entry) => {
                // If this is a crate root, but not this crate's root, skip it
                entry.vis.is_public() && !(entry.crate_root.is_some() && node_index != root_index)
            }
            AnalysisRef::Origin => true,
        };

        if !public {
            analysis.graph.remove_node(node_index);
        }
    }

    let indices = analysis.graph.node_indices().collect::<Vec<_>>();
    for node_index in indices {
        if !(has_path_connecting(&analysis.graph, root_index, node_index, None)
            || has_path_connecting(&analysis.graph, node_index, root_index, None))
        {
            analysis.graph.remove_node(node_index);
        }
    }

    Ok(())
}

#[allow(dead_code)]
pub fn node_ident(analysis: &Analysis, root_index: NodeIndex, index: NodeIndex) -> AResult<Ident> {
    item_path(&analysis, root_index, index)?
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
                    AnalysisEntry::Struct(id) => {
                        let entry = &analysis.structs[*id];
                        let vis = &entry.vis;
                        q!(#vis struct).to_string()
                    }
                    AnalysisEntry::Enum(id) => {
                        let entry = &analysis.enums[*id];
                        let vis = &entry.vis;

                        q!(#vis enum).to_string()
                    }
                    AnalysisEntry::Type(id) => {
                        let entry = &analysis.types[*id];
                        let vis = &entry.vis;

                        q!(#vis type).to_string()
                    }
                    AnalysisEntry::Trait(id) => {
                        let entry = &analysis.traits[*id];
                        let vis = &entry.vis;

                        q!(#vis trait).to_string()
                    }
                    AnalysisEntry::Mod(id) => {
                        let entry = &analysis.modules[*id];
                        let vis = &entry.vis;

                        q!(#vis mod).to_string()
                    }
                    AnalysisEntry::Origin => "origin".to_string(),
                    AnalysisEntry::None => "none".to_string(),
                };

                format!("label = \"{}\"", label)
            }
        )
    );
}
