use std::path::PathBuf;

use crate::{
    AResult, EntryIndex,
    analysis::{Analysis, AnalysisEdge, Ctx},
    analysis_entry::{
        AnalysisEntry, AnalysisEnum, AnalysisImpl, AnalysisImplConst, AnalysisMod, AnalysisStruct,
        AnalysisTrait, AnalysisType,
    },
    crate_graph::{find_neighbor, update_edge},
    types::type_path,
    use_statements::process_use_statements,
    utils::{id, path_segments, write_expanded},
};
use anyhow::{Context, bail};
use petgraph::{Direction, graph::NodeIndex};
use syn::{Ident, ImplItem, Item, Type, visit_mut::VisitMut};

pub fn parse_crate<'a>(
    analysis: &'a mut Analysis,
    output_path: PathBuf,
    crate_path: PathBuf,
    crate_name: &'a str,
    dependencies: Vec<String>,
) -> AResult<Ctx<'a>> {
    write_expanded(&output_path, &crate_path)?;

    let Ok(contents) = std::fs::read_to_string(output_path.clone()) else {
        bail!("Failed to read {:?}", &output_path);
    };

    process_crate(analysis, crate_name, contents, dependencies)
}

pub fn process_crate<'a>(
    analysis: &'a mut Analysis,
    crate_name: &'a str,
    contents: String,
    dependencies: Vec<String>,
) -> AResult<Ctx<'a>> {
    let mut ctx = analysis.add_crate(crate_name.to_string(), contents)?;
    let crate_root = ctx.crate_root.clone();

    for name in dependencies {
        add_extern_crate(&mut ctx, crate_root, id(name.as_str()), id(name.as_str()));
    }

    process_subtree(&mut ctx, crate_root)?;
    discover_paths(&mut ctx)?;
    link_impls(&mut ctx)?;

    Ok(ctx)
}

fn add_extern_crate(
    ctx: &mut Ctx<'_>,
    from_node: NodeIndex,
    extern_crate_name: Ident,
    extern_crate_rename: Ident,
) {
    let Some(node) = ctx.graph().node_indices().find(|n| {
        if let Ok(AnalysisEntry::Mod(krate)) = ctx.entry(*n)
            && let Some(name) = &krate.root_of_crate
        {
            return name.to_string() == extern_crate_name.to_string();
        }

        return false;
    }) else {
        log::debug!("Skipping extern crate {}", extern_crate_name);
        return;
    };

    ctx.graph_mut().add_edge(
        from_node,
        node,
        AnalysisEdge {
            from_hierarchy: false,
            name: Some(extern_crate_rename),
        },
    );
}

fn discover_paths(ctx: &mut Ctx) -> AResult<()> {
    let mut num_edges: usize = ctx.graph().edge_count();
    loop {
        process_use_statements(ctx)?;
        let new_num_edges = ctx.graph().edge_count();
        if num_edges == new_num_edges {
            break;
        } else {
            num_edges = new_num_edges;
        }
    }

    Ok(())
}

pub fn process_subtree(ctx: &mut Ctx, parent_mod: NodeIndex) -> AResult<()> {
    let content = if let AnalysisEntry::Mod(entry) = ctx.entry(parent_mod)? {
        entry.content()
    } else {
        bail!("Couldn't get subtree node")
    };

    for item in content.into_iter() {
        match item {
            Item::Mod(mod_item) => {
                let mod_item = mod_item.clone();
                let ident = mod_item.ident.clone();

                let child_mod = ctx
                    .graph_mut()
                    .add_node(AnalysisEntry::Mod(AnalysisMod::new(mod_item, None)));

                update_edge(
                    ctx,
                    parent_mod,
                    child_mod,
                    AnalysisEdge::new(true, Some(ident.clone())),
                )?;

                process_subtree(ctx, child_mod)?;
            }
            Item::Struct(item) => {
                let ident = item.ident.clone();

                let child = ctx
                    .graph_mut()
                    .add_node(AnalysisEntry::Struct(AnalysisStruct::new(item.clone())));

                update_edge(ctx, parent_mod, child, AnalysisEdge::new(true, Some(ident)))?;
            }
            Item::Enum(item) => {
                let ident = item.ident.clone();

                let child = ctx
                    .graph_mut()
                    .add_node(AnalysisEntry::Enum(AnalysisEnum::new(item.clone())));

                update_edge(ctx, parent_mod, child, AnalysisEdge::new(true, Some(ident)))?;

                for variant in item.variants {
                    let variant_node = ctx.graph_mut().add_node(AnalysisEntry::Variant);

                    update_edge(
                        ctx,
                        child,
                        variant_node,
                        AnalysisEdge::new(true, Some(variant.ident)),
                    )?;
                }
            }
            Item::Type(item) => {
                let ident = item.ident.clone();

                let child = ctx
                    .graph_mut()
                    .add_node(AnalysisEntry::Type(AnalysisType::new(item.clone())));

                update_edge(ctx, parent_mod, child, AnalysisEdge::new(true, Some(ident)))?;
            }
            Item::Trait(item) => {
                let ident = item.ident.clone();
                let child = ctx
                    .graph_mut()
                    .add_node(AnalysisEntry::Trait(AnalysisTrait::new(item.clone())));

                update_edge(ctx, parent_mod, child, AnalysisEdge::new(true, Some(ident)))?;
            }
            Item::Impl(item) => {
                let child = ctx
                    .graph_mut()
                    .add_node(AnalysisEntry::Impl(AnalysisImpl::new(item.clone())));

                update_edge(ctx, parent_mod, child, AnalysisEdge::new(true, None))?;

                for inner in item.items {
                    if let ImplItem::Const(c) = inner {
                        let ident = c.ident.clone();
                        let const_node = ctx
                            .graph_mut()
                            .add_node(AnalysisEntry::ImplConst(AnalysisImplConst::new(c)));

                        update_edge(ctx, child, const_node, AnalysisEdge::new(true, Some(ident)))?;
                    }
                }
            }
            Item::ExternCrate(item) => {
                let rename = item
                    .rename
                    .map_or_else(|| item.ident.clone(), |(_, rename)| rename);

                add_extern_crate(ctx, parent_mod, item.ident.clone(), rename);
            }
            _ => (),
        };
    }

    Ok(())
}

struct TyResolve<'a> {
    ctx: &'a Ctx<'a>,
    item_index: EntryIndex,
}

impl<'a> VisitMut for TyResolve<'a> {
    fn visit_path_mut(&mut self, path: &mut syn::Path) {
        type_path(self.ctx, self.item_index, path);
    }
}

pub fn resolve_path(
    ctx: &Ctx,
    item_index: NodeIndex,
    relative_path: &[Ident],
) -> AResult<NodeIndex> {
    resolve_path_inner(ctx, item_index, relative_path)
}

pub fn resolve_path_inner(
    ctx: &Ctx,
    item_index: NodeIndex,
    relative_path: &[Ident],
) -> AResult<NodeIndex> {
    let entry = ctx
        .graph()
        .node_weight(item_index)
        .context("Couldn't get node")?;

    let module_index = if matches!(entry, AnalysisEntry::Mod(_)) {
        item_index
    } else {
        get_super(ctx, item_index)?
    };

    resolve_path_recurse(ctx, item_index, module_index, relative_path, 0)
}

pub fn resolve_path_recurse(
    ctx: &Ctx,
    self_index: NodeIndex,
    current: NodeIndex,
    relative_path: &[Ident],
    path_segment_index: usize,
) -> AResult<NodeIndex> {
    let Some(path_segment) = relative_path.get(path_segment_index) else {
        return Ok(current);
    };

    let next_index = resolve_next_segment(ctx, self_index, current, path_segment)?;
    resolve_path_recurse(
        ctx,
        self_index,
        next_index,
        relative_path,
        path_segment_index + 1,
    )
}

pub fn resolve_next_segment(
    ctx: &Ctx,
    self_index: NodeIndex,
    current: NodeIndex,
    path_segment: &Ident,
) -> AResult<NodeIndex> {
    if &path_segment.to_string() == "self" {
        Ok(current)
    } else if &path_segment.to_string() == "Self" {
        Ok(self_index)
    } else if &path_segment.to_string() == "crate" {
        Ok(ctx.crate_root)
    } else if &path_segment.to_string() == "super" {
        get_super(ctx, current)
    } else if let Some(child_index) = find_neighbor(ctx, current, &path_segment) {
        Ok(child_index)
    } else if let Ok(node) = resolve_prelude(ctx, path_segment) {
        Ok(node)
    } else {
        bail!("Couldn't resolve segment {:?}", path_segment);
    }
}

pub fn link_impls(ctx: &mut Ctx) -> AResult<()> {
    let mut bfs = ctx.bfs()?;

    let mut to_add = vec![];
    while let Some(impl_node) = bfs.next(&ctx.graph()) {
        if let AnalysisEntry::Impl(impl_entry) = ctx.entry(impl_node).unwrap()
            && let Type::Path(path) = *impl_entry.item.self_ty.clone()
        {
            let adt = resolve_path(ctx, impl_node, &path_segments(&path.path))?;

            let mut neighbors = ctx.graph().neighbors(impl_node).detach();
            while let Some((edge_index, neighbor)) = neighbors.next(ctx.graph()) {
                if let Some(edge) = ctx.graph().edge_weight(edge_index)
                    && let Some(name) = edge.name.as_ref()
                {
                    if let AnalysisEntry::ImplConst(_) = ctx.entry(neighbor)? {
                        to_add.push((adt, neighbor, AnalysisEdge::new(false, Some(name.clone()))));
                    }
                }
            }
        }
    }

    for (from_node, to_node, edge) in to_add {
        ctx.graph_mut().add_edge(from_node, to_node, edge);
    }

    Ok(())
}

fn resolve_prelude(ctx: &Ctx, path_segment: &Ident) -> AResult<NodeIndex> {
    let root = ctx.crate_root;
    if let AnalysisEntry::Mod(module) = ctx.krate()?
        && let Some(root_of) = &module.root_of_crate
        && root_of.to_string() == path_segment.to_string()
    {
        Ok(root)
    } else if let Some(neighbor) = find_neighbor(ctx, root, path_segment)
        && let AnalysisEntry::Mod(entry) = ctx.entry(neighbor)?
        && entry.root_of_crate.is_some()
    {
        Ok(neighbor)
    } else {
        bail!("Couldn't find extern crate {:?}", path_segment);
    }
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
