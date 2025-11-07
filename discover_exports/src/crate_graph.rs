use anyhow::{Context, bail};
use petgraph::{
    algo::astar,
    dot::{Config, Dot},
    graph::{EdgeIndex, NodeIndex},
    visit::{EdgeRef, Walker},
};
use quote::quote as q;
use syn::Ident;

use crate::{
    AResult,
    analysis::{AnalysisEdge, Ctx},
    analysis_entry::AnalysisEntry,
    utils::IsPublic,
};

pub fn filter_map_nodes<T>(
    ctx: &Ctx,
    filter_fn: impl FnMut(NodeIndex) -> Option<T>,
    path_type: PathType,
) -> AResult<impl Iterator<Item = T>> {
    let bfs = ctx.bfs()?;
    Ok(bfs
        .iter(ctx.graph())
        .filter(move |node| {
            if path_type == PathType::PublicOnly {
                full_path(ctx, *node, path_type).is_ok()
            } else {
                true
            }
        })
        .filter_map(filter_fn))
}

pub fn for_each_node(
    ctx: &Ctx,
    item_fn: impl FnMut(NodeIndex),
    path_type: PathType,
) -> AResult<()> {
    let bfs = ctx.bfs()?;
    bfs.iter(ctx.graph())
        .filter(move |node| {
            if path_type == PathType::PublicOnly {
                full_path(ctx, *node, path_type).is_ok()
            } else {
                true
            }
        })
        .for_each(item_fn);
    Ok(())
}

pub fn find_neighbor<'a>(
    ctx: &Ctx,
    entry_tree_node: NodeIndex,
    ident: &syn::Ident,
) -> Option<NodeIndex> {
    let mut neighbors = ctx.graph().neighbors(entry_tree_node).detach();
    while let Some((edge_index, neighbor)) = neighbors.next(ctx.graph()) {
        if let Some(edge) = ctx.graph().edge_weight(edge_index)
            && let Some(name) = edge.name.as_ref()
        {
            if name.to_string() == ident.to_string() {
                return Some(neighbor);
            }
        }
    }

    None
}

pub fn update_edge<'a>(
    ctx: &'a mut Ctx,
    from: NodeIndex,
    to: NodeIndex,
    edge: AnalysisEdge,
) -> AResult<EdgeIndex> {
    let mut connecting = ctx.graph().edges_connecting(from, to);
    while let Some(existing_index) = connecting.next() {
        let existing = existing_index.weight();

        if edge.from_hierarchy == existing.from_hierarchy {
            log::debug!("already-existing edge type {:?} to {:?}", existing, edge);
            return Ok(existing_index.id());
        }
    }

    Ok(ctx.graph_mut().add_edge(from, to, edge))
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PathType {
    Any,
    PublicOnly,
}

pub fn full_path<'a>(
    ctx: &'a Ctx,
    node_index: NodeIndex,
    path_type: PathType,
) -> AResult<Vec<Ident>> {
    let (cost, graph_path) = astar(
        ctx.graph(),
        ctx.crate_root,
        |x| x == node_index,
        |e| {
            let entry = ctx.entry(e.target()).unwrap();
            let is_impl = matches!(entry, AnalysisEntry::Impl(_));
            let no_visibility = path_type == PathType::PublicOnly && !entry.vis().is_public();

            if no_visibility || is_impl { 1 } else { 0 }
        },
        |_| 0,
    )
    .context(format!("Couldn't get path {:?}", node_index))?;

    if cost > 0 {
        bail!("Can't find public path to item");
    }

    let mut path = vec![];
    let mut previous_segment = None;
    for node_index in graph_path.iter() {
        if let Some(from_index) = previous_segment {
            if let Some(edge) = ctx.graph().edges_connecting(from_index, *node_index).next() {
                if let Some(name) = &edge.weight().name {
                    path.push(name.clone());
                }
            }
        }

        previous_segment = Some(*node_index);
    }

    Ok(path)
}

pub fn node_ident(ctx: &Ctx, index: NodeIndex, path_type: PathType) -> AResult<Ident> {
    full_path(&ctx, index, path_type)?
        .last()
        .context("Invalid path")
        .cloned()
}

#[allow(dead_code)]
pub fn print_dot(ctx: &Ctx) -> AResult<()> {
    println!(
        "{:?}",
        Dot::with_attr_getters(
            ctx.graph(),
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
                let vis = &entry.vis();
                let label = match entry {
                    AnalysisEntry::Struct(_) => q!(#vis struct).to_string(),
                    AnalysisEntry::Enum(_) => q!(#vis enum).to_string(),
                    AnalysisEntry::Type(_) => q!(#vis type).to_string(),
                    AnalysisEntry::Trait(_) => q!(#vis trait).to_string(),
                    AnalysisEntry::Impl(_) => q!(impl).to_string(),
                    AnalysisEntry::ImplConst(_) => q!(#vis const).to_string(),
                    AnalysisEntry::Variant => q!(variant).to_string(),
                    AnalysisEntry::Mod(module) => {
                        let root = &module.root_of_crate;
                        q!(#vis mod #root).to_string()
                    }
                };

                format!("label = \"{}\"", label)
            }
        )
    );

    Ok(())
}
