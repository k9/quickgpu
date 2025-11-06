use anyhow::Context;
use petgraph::{
    algo::astar,
    dot::{Config, Dot},
    graph::{EdgeIndex, NodeIndex},
    visit::{EdgeRef, Walker},
};
use quote::quote as q;
use syn::{Ident, ImplItem, ImplItemConst};

use crate::{
    AResult, AnalysisEdge, AnalysisStruct,
    analysis::{AnalysisEntry, Ctx},
    utils::IsPublic,
};

pub fn filter_map_nodes<T>(
    ctx: &Ctx,
    filter_fn: impl FnMut(NodeIndex) -> Option<T>,
) -> AResult<impl Iterator<Item = T>> {
    let bfs = ctx.bfs()?;
    Ok(bfs.iter(ctx.graph()).filter_map(filter_fn))
}

pub fn for_each_node(ctx: &Ctx, item_fn: impl FnMut(NodeIndex)) -> AResult<()> {
    let bfs = ctx.bfs()?;
    bfs.iter(ctx.graph()).for_each(item_fn);
    Ok(())
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

pub fn full_path<'a>(ctx: &'a Ctx, node_index: NodeIndex) -> AResult<Vec<Ident>> {
    let graph_path = astar(
        ctx.graph(),
        ctx.crate_root,
        |x| x == node_index,
        |_| 1,
        |_| 0,
    )
    .context(format!("Couldn't get path {:?}", node_index))?;

    let mut path = vec![];
    let mut previous_segment = None;
    for node_index in graph_path.1.iter() {
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

pub fn keep_only_public<'a>(ctx: &'a mut Ctx) -> AResult<()> {
    let to_remove = keep_only_public_recurse(ctx, ctx.crate_root)?;

    for index in to_remove {
        ctx.graph_mut().remove_node(index);
    }

    Ok(())
}

pub fn keep_only_public_recurse<'a>(
    ctx: &'a mut Ctx,
    current: NodeIndex,
) -> AResult<Vec<NodeIndex>> {
    let mut neighbors = ctx.graph().neighbors(current).detach();
    let mut visited = vec![];
    let mut to_remove = vec![];
    while let Some((_, node_index)) = neighbors.next(&ctx.graph()) {
        if visited.contains(&node_index) {
            continue;
        } else {
            visited.push(node_index);
        }

        let public = match ctx.entry(node_index)? {
            AnalysisEntry::Struct(entry) => entry.vis.is_public(),
            AnalysisEntry::Enum(entry) => entry.vis.is_public(),
            AnalysisEntry::Type(entry) => entry.vis.is_public(),
            AnalysisEntry::Trait(entry) => entry.vis.is_public(),
            AnalysisEntry::Mod(entry) => entry.vis.is_public(),
            AnalysisEntry::ExternCrate(entry) => entry.vis.is_public(),
            AnalysisEntry::Variant => true,
            AnalysisEntry::Const(entry) => entry.vis.is_public(),
        };

        if !public {
            to_remove.push(node_index);
        }

        to_remove.append(&mut keep_only_public_recurse(ctx, node_index)?);
    }

    Ok(to_remove)
}

pub fn node_ident(ctx: &Ctx, index: NodeIndex) -> AResult<Ident> {
    full_path(&ctx, index)?
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
                    AnalysisEntry::ExternCrate(entry) => {
                        let vis = &entry.vis;
                        let name = &entry.name;
                        q!(#vis #name crate).to_string()
                    }
                    AnalysisEntry::Variant => "variant".to_string(),
                };

                format!("label = \"{}\"", label)
            }
        )
    );

    Ok(())
}
