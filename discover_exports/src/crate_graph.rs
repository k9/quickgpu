use anyhow::{Context, bail};
use petgraph::{
    Direction,
    dot::{Config, Dot},
    graph::{EdgeIndex, NodeIndex},
    visit::{EdgeRef, Walker},
};
use quote::quote as q;
use syn::Path;

use crate::resolve::{PathType, get_public_path, get_top_level_path};
use crate::{
    AResult,
    analysis::{AnalysisEdge, Ctx},
    analysis_entry::AnalysisEntry,
};

pub fn filter_map_nodes<T>(
    ctx: &Ctx,
    filter_fn: impl FnMut((NodeIndex, Path)) -> Option<T>,
    path_type: PathType,
) -> AResult<impl Iterator<Item = T>> {
    let bfs = ctx.bfs()?;
    Ok(bfs
        .iter(ctx.graph())
        .filter_map(move |node| {
            let path = if path_type == PathType::PublicOnly {
                get_public_path(ctx, node)
            } else {
                get_top_level_path(ctx, node)
            };

            if let Ok(path) = path {
                Some((node, path))
            } else {
                None
            }
        })
        .filter_map(filter_fn))
}

pub fn for_each_node(
    ctx: &Ctx,
    item_fn: impl FnMut((NodeIndex, Path)),
    path_type: PathType,
) -> AResult<()> {
    let bfs = ctx.bfs()?;
    bfs.iter(ctx.graph())
        .filter_map(|node| {
            let path = if path_type == PathType::PublicOnly {
                get_public_path(ctx, node)
            } else {
                get_top_level_path(ctx, node)
            };

            if let Ok(path) = path {
                Some((node, path))
            } else {
                None
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

pub fn get_super(ctx: &Ctx, node_index: NodeIndex) -> AResult<NodeIndex> {
    let mut parents = ctx
        .graph()
        .neighbors_directed(node_index, Direction::Incoming)
        .detach();

    while let Ok((edge_index, node_index)) =
        parents.next(ctx.graph()).context("Couldn't get parent")
    {
        if let Some(edge) = ctx.graph().edge_weight(edge_index)
            && edge.from_hierarchy
        {
            return Ok(node_index);
        }
    }

    bail!("Couldn't get parent");
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
                    AnalysisEntry::ImplFn(_) => q!(#vis fn).to_string(),
                    AnalysisEntry::Variant(_) => q!(variant).to_string(),
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
