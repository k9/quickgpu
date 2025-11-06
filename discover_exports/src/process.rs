use std::path::PathBuf;

use crate::{
    AResult, EntryIndex,
    analysis::{
        Analysis, AnalysisConst, AnalysisEdge, AnalysisEntry, AnalysisEnum, AnalysisMod,
        AnalysisStruct, AnalysisTrait, AnalysisTypeAlias, Ctx,
    },
    crate_graph::{find_neighbor, keep_only_public, update_edge},
    types::{resolve_type_paths, type_path},
    use_statements::process_use_statements,
    utils::{id, path_segments, write_expanded},
};
use anyhow::{Context, bail};
use petgraph::{Direction, graph::NodeIndex};
use syn::{Fields, Ident, ImplItem, Item, Type, visit_mut::VisitMut};

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
        let node = ctx
            .graph()
            .node_indices()
            .find(|n| {
                if let Ok(AnalysisEntry::ExternCrate(krate)) = ctx.entry(*n) {
                    return krate.name.to_string() == name;
                }

                return false;
            })
            .context(format!("Couldn't find dependency {}", name))?;

        ctx.graph_mut().add_edge(
            crate_root,
            node,
            AnalysisEdge {
                from_hierarchy: false,
                name: Some(id(name.as_str())),
            },
        );
    }

    process_subtree(&mut ctx, crate_root)?;
    discover_paths(&mut ctx)?;
    process_impls(&mut ctx)?;
    process_fields(&mut ctx)?;
    keep_only_public(&mut ctx)?;

    Ok(ctx)
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
    let content = match ctx.entry(parent_mod)? {
        AnalysisEntry::ExternCrate(entry) => entry.content.clone(),
        AnalysisEntry::Mod(entry) => entry.content.clone(),
        _ => bail!("Couldn't get subtree node"),
    };

    for item in content.into_iter() {
        match item {
            Item::Mod(mod_item) => {
                let ident = mod_item.ident.clone();

                let child_mod = ctx
                    .graph_mut()
                    .add_node(AnalysisEntry::Mod(AnalysisMod::new(mod_item)));

                update_edge(
                    ctx,
                    parent_mod,
                    child_mod,
                    AnalysisEdge::new(true, Some(ident)),
                )?;

                process_subtree(ctx, child_mod)?;
            }
            Item::Struct(item) => {
                let ident = item.ident.clone();

                let child = ctx
                    .graph_mut()
                    .add_node(AnalysisEntry::Struct(AnalysisStruct::new(
                        item.clone(),
                        vec![],
                    )));

                update_edge(ctx, parent_mod, child, AnalysisEdge::new(true, Some(ident)))?;
            }
            Item::Enum(item) => {
                let ident = item.ident.clone();

                let child = ctx
                    .graph_mut()
                    .add_node(AnalysisEntry::Enum(AnalysisEnum::new(item.clone(), vec![])));

                update_edge(ctx, parent_mod, child, AnalysisEdge::new(true, Some(ident)))?;

                for variant in &item.variants {
                    let variant_index = ctx.graph_mut().add_node(AnalysisEntry::Variant);

                    update_edge(
                        ctx,
                        child,
                        variant_index,
                        AnalysisEdge::new(true, Some(variant.ident.clone())),
                    )?;
                }
            }
            Item::Type(item) => {
                let ident = item.ident.clone();

                let child = ctx
                    .graph_mut()
                    .add_node(AnalysisEntry::Type(AnalysisTypeAlias::new(item.clone())));

                update_edge(ctx, parent_mod, child, AnalysisEdge::new(true, Some(ident)))?;
            }
            Item::Trait(item) => {
                let ident = item.ident.clone();
                let child = ctx
                    .graph_mut()
                    .add_node(AnalysisEntry::Trait(AnalysisTrait::new(item.clone())));

                update_edge(ctx, parent_mod, child, AnalysisEdge::new(true, Some(ident)))?;
            }
            _ => (),
        };
    }

    Ok(())
}

pub fn process_impls(ctx: &mut Ctx) -> AResult<()> {
    let mut to_add = vec![];
    let mut bfs = ctx.bfs()?;

    while let Some(node_index) = bfs.next(ctx.graph()) {
        if let AnalysisEntry::Mod(module) = ctx.entry(node_index)? {
            let content = module.content.clone();
            for item in content {
                if let Item::Impl(item_impl) = &item {
                    let mut item_impl = item_impl.clone();
                    let ty = (*item_impl.self_ty).clone();
                    let ty = resolve_type_paths(ty, ctx, node_index);
                    if let Type::Path(path) = &ty {
                        if let Ok(resolved_index) =
                            resolve_path(ctx, ctx.crate_root, &path_segments(&path.path))
                        {
                            item_impl.self_ty = Box::new(ty);

                            let mut resolver = TyResolve {
                                ctx,
                                item_index: node_index,
                            };

                            resolver.visit_item_impl_mut(&mut item_impl);

                            for item in &item_impl.items {
                                if let ImplItem::Const(const_item) = item {
                                    let entry = AnalysisConst::new(const_item.clone());
                                    let ident = const_item.ident.clone();
                                    to_add.push((entry, ident, resolved_index));
                                }
                            }

                            match ctx.entry_mut(resolved_index) {
                                Ok(AnalysisEntry::Struct(entry)) => entry.impls.push(item_impl),
                                Ok(AnalysisEntry::Enum(entry)) => entry.impls.push(item_impl),
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
    }

    for (entry, ident, resolved_index) in to_add.into_iter() {
        let const_index = ctx.graph_mut().add_node(AnalysisEntry::Const(entry));

        update_edge(
            ctx,
            resolved_index,
            const_index,
            AnalysisEdge::new(true, Some(ident)),
        )?;
    }

    Ok(())
}

struct TyResolve<'a> {
    ctx: &'a Ctx<'a>,
    item_index: EntryIndex,
}

impl<'a> VisitMut for TyResolve<'a> {
    fn visit_type_mut(&mut self, ty: &mut Type) {
        *ty = resolve_type_paths(ty.clone(), self.ctx, self.item_index);
    }

    fn visit_path_mut(&mut self, path: &mut syn::Path) {
        type_path(self.ctx, self.item_index, path);
    }
}

pub fn process_fields(ctx: &mut Ctx) -> AResult<()> {
    let mut bfs = ctx.bfs()?;
    while let Some(node_index) = bfs.next(ctx.graph()) {
        if let AnalysisEntry::Struct(entry) = ctx.entry_mut(node_index)? {
            let entry = entry.clone();
            if let Fields::Named(fields) = &entry.fields {
                for (field_index, field) in fields.named.iter().enumerate() {
                    let ty = field.ty.clone();
                    let ty = resolve_type_paths(ty, ctx, node_index);

                    if let AnalysisEntry::Struct(entry) = ctx.entry_mut(node_index)? {
                        if let Fields::Named(fields) = &mut entry.fields {
                            fields
                                .named
                                .get_mut(field_index)
                                .context("Error updating field")?
                                .ty = ty;
                        }
                    }
                }
            }
        }
    }

    Ok(())
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

    let module_index = if matches!(entry, AnalysisEntry::Mod(_))
        || matches!(entry, AnalysisEntry::ExternCrate(_))
    {
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
    } else if current == ctx.crate_root {
        Ok(ctx.crate_root)
    } else {
        bail!("extern crate not implemented");
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
