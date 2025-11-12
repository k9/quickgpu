use std::path::PathBuf;

use crate::{
    AResult,
    analysis::{Analysis, AnalysisEdge, Ctx},
    analysis_entry::{
        AnalysisEntry, AnalysisEnum, AnalysisImpl, AnalysisImplConst, AnalysisImplFn, AnalysisMod,
        AnalysisStruct, AnalysisTrait, AnalysisType, AnalysisVariant,
    },
    crate_graph::update_edge,
    resolve::{calculate_paths, resolve_path},
    use_statements::process_use_statements,
    utils::{id, write_expanded},
};
use anyhow::bail;
use petgraph::graph::NodeIndex;
use syn::{
    Ident, ImplItem, Item, ItemEnum, ItemExternCrate, ItemImpl, ItemMod, ItemStruct, ItemTrait,
    ItemType, Type,
};

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

    calculate_paths(&mut ctx)?;

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
    let AnalysisEntry::Mod(entry) = ctx.entry(parent_mod)? else {
        bail!("Couldn't get subtree node")
    };

    let Some(content) = entry.content() else {
        bail!("Couldn't get subtree node")
    };

    let content = content.iter().cloned().collect::<Vec<_>>();

    for item in content {
        match item {
            Item::Mod(mod_item) => {
                process_mod(ctx, parent_mod, mod_item)?;
            }
            Item::Struct(item) => {
                process_struct(ctx, parent_mod, item)?;
            }
            Item::Enum(item) => {
                process_enum(ctx, parent_mod, item)?;
            }
            Item::Type(item) => {
                process_type(ctx, parent_mod, item)?;
            }
            Item::Trait(item) => {
                process_trait(ctx, parent_mod, item)?;
            }
            Item::Impl(item) => {
                process_impl(ctx, parent_mod, item)?;
            }
            Item::ExternCrate(item) => {
                process_extern_crate(ctx, parent_mod, item);
            }
            _ => (),
        };
    }

    Ok(())
}

fn process_mod(
    ctx: &mut Ctx<'_>,
    parent_mod: NodeIndex,
    mod_item: ItemMod,
) -> Result<(), anyhow::Error> {
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
    Ok(())
}

fn process_struct(
    ctx: &mut Ctx<'_>,
    parent_mod: NodeIndex,
    item: ItemStruct,
) -> Result<(), anyhow::Error> {
    let ident = item.ident.clone();
    let child = ctx
        .graph_mut()
        .add_node(AnalysisEntry::Struct(AnalysisStruct::new(item.clone())));

    update_edge(ctx, parent_mod, child, AnalysisEdge::new(true, Some(ident)))?;

    Ok(())
}

fn process_enum(
    ctx: &mut Ctx<'_>,
    parent_mod: NodeIndex,
    item: ItemEnum,
) -> Result<(), anyhow::Error> {
    let ident = item.ident.clone();
    let child = ctx
        .graph_mut()
        .add_node(AnalysisEntry::Enum(AnalysisEnum::new(item.clone())));

    update_edge(ctx, parent_mod, child, AnalysisEdge::new(true, Some(ident)))?;

    Ok(for variant in &item.variants {
        let variant_node = ctx
            .graph_mut()
            .add_node(AnalysisEntry::Variant(AnalysisVariant::new(
                variant.clone(),
            )));

        update_edge(
            ctx,
            child,
            variant_node,
            AnalysisEdge::new(true, Some(variant.ident.clone())),
        )?;
    })
}

fn process_type(
    ctx: &mut Ctx<'_>,
    parent_mod: NodeIndex,
    item: ItemType,
) -> Result<(), anyhow::Error> {
    let ident = item.ident.clone();
    let child = ctx
        .graph_mut()
        .add_node(AnalysisEntry::Type(AnalysisType::new(item.clone())));

    update_edge(ctx, parent_mod, child, AnalysisEdge::new(true, Some(ident)))?;

    Ok(())
}

fn process_trait(
    ctx: &mut Ctx<'_>,
    parent_mod: NodeIndex,
    item: ItemTrait,
) -> Result<(), anyhow::Error> {
    let ident = item.ident.clone();
    let child = ctx
        .graph_mut()
        .add_node(AnalysisEntry::Trait(AnalysisTrait::new(item.clone())));

    update_edge(ctx, parent_mod, child, AnalysisEdge::new(true, Some(ident)))?;

    Ok(())
}

fn process_impl(
    ctx: &mut Ctx<'_>,
    parent_mod: NodeIndex,
    item: ItemImpl,
) -> Result<(), anyhow::Error> {
    let child = ctx
        .graph_mut()
        .add_node(AnalysisEntry::Impl(AnalysisImpl::new(item.clone())));

    update_edge(ctx, parent_mod, child, AnalysisEdge::new(true, None))?;

    Ok(for inner in &item.items {
        match inner {
            ImplItem::Const(c) => {
                let ident = c.ident.clone();
                let const_node =
                    ctx.graph_mut()
                        .add_node(AnalysisEntry::ImplConst(AnalysisImplConst::new(
                            c.clone(),
                            item.trait_.is_some(),
                        )));

                if item.trait_.is_some() {}

                update_edge(ctx, child, const_node, AnalysisEdge::new(true, Some(ident)))?;
            }
            ImplItem::Fn(f) => {
                let ident = f.sig.ident.clone();
                let fn_node = ctx
                    .graph_mut()
                    .add_node(AnalysisEntry::ImplFn(AnalysisImplFn::new(
                        f.clone(),
                        item.trait_.is_some(),
                    )));

                update_edge(ctx, child, fn_node, AnalysisEdge::new(true, Some(ident)))?;
            }
            _ => (),
        }
    })
}

fn process_extern_crate(ctx: &mut Ctx<'_>, parent_mod: NodeIndex, item: ItemExternCrate) {
    let rename = item
        .rename
        .as_ref()
        .map_or_else(|| item.ident.clone(), |(_, rename)| rename.clone());

    add_extern_crate(ctx, parent_mod, item.ident.clone(), rename);
}

pub fn link_impls(ctx: &mut Ctx) -> AResult<()> {
    let mut bfs = ctx.bfs()?;

    let mut to_add = vec![];
    while let Some(impl_node) = bfs.next(&ctx.graph()) {
        if let AnalysisEntry::Impl(impl_entry) = ctx.entry(impl_node).unwrap()
            && let Type::Path(path) = *impl_entry.item.self_ty.clone()
            && let Ok(adt) = resolve_path(ctx, impl_node, &path.path)
        {
            to_add.push((adt, impl_node, AnalysisEdge::new(false, None)));

            let mut neighbors = ctx.graph().neighbors(impl_node).detach();
            while let Some((edge_index, neighbor)) = neighbors.next(ctx.graph()) {
                if let Some(edge) = ctx.graph().edge_weight(edge_index)
                    && let Some(name) = edge.name.as_ref()
                {
                    match ctx.entry(neighbor)? {
                        AnalysisEntry::ImplConst(_) => {
                            to_add.push((
                                adt,
                                neighbor,
                                AnalysisEdge::new(false, Some(name.clone())),
                            ));
                        }
                        AnalysisEntry::ImplFn(_) => {
                            to_add.push((
                                adt,
                                neighbor,
                                AnalysisEdge::new(false, Some(name.clone())),
                            ));
                        }
                        _ => (),
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
