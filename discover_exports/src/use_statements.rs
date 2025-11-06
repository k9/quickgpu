use anyhow::{Context, bail};
use petgraph::graph::NodeIndex;
use syn::{Item, UseTree};

use crate::{
    AResult,
    analysis::{AnalysisEdge, AnalysisEntry, Ctx},
    crate_graph::{find_neighbor, update_edge},
    process::resolve_next_segment,
};

pub fn process_use_statements(ctx: &mut Ctx) -> AResult<()> {
    let mut bfs = ctx.bfs()?;
    while let Some(node_index) = bfs.next(&ctx.graph()) {
        if let AnalysisEntry::Mod(module) = ctx.entry(node_index)? {
            let module = module.clone();

            for item in module.content {
                if let Item::Use(use_statement) = item {
                    let use_statement = use_statement.clone();
                    process_use_tree(ctx, node_index, node_index, &use_statement.tree)?;
                } else if let Item::ExternCrate(extern_crate) = item {
                    let extern_crate = extern_crate.clone();
                    process_extern_crate(ctx, node_index, &extern_crate)?;
                }
            }
        }
    }

    Ok(())
}

fn process_use_tree(
    ctx: &mut Ctx,
    from_module: NodeIndex,
    to_module: NodeIndex,
    use_subtree: &UseTree,
) -> AResult<()> {
    match &use_subtree {
        syn::UseTree::Path(use_path) => {
            process_use_path(ctx, from_module, to_module, use_path)?;
        }
        syn::UseTree::Group(use_group) => {
            for item in &use_group.items {
                process_use_tree(ctx, from_module, to_module, item)?;
            }
        }
        syn::UseTree::Name(use_name) => {
            if let Ok(next_index) = resolve_next_segment(ctx, to_module, to_module, &use_name.ident)
            {
                update_edge(
                    ctx,
                    from_module,
                    next_index,
                    AnalysisEdge::new(false, Some(use_name.ident.clone())),
                )?;
            };
        }
        syn::UseTree::Glob(_) => {
            let mut neighbors = ctx.graph().neighbors(to_module).detach();

            while let Some((edge, neighbor)) = neighbors.next(ctx.graph()) {
                let rename = ctx
                    .graph()
                    .edge_weight(edge)
                    .context("Coulnd't get edge weight")?
                    .name
                    .clone();

                update_edge(ctx, from_module, neighbor, AnalysisEdge::new(false, rename))?;
            }
        }
        syn::UseTree::Rename(use_rename) => {
            if let Some(neighbor) = find_neighbor(ctx, to_module, &use_rename.ident) {
                update_edge(
                    ctx,
                    from_module,
                    neighbor,
                    AnalysisEdge::new(false, Some(use_rename.rename.clone())),
                )?;
            }
        }
    };

    Ok(())
}

fn process_use_path(
    ctx: &mut Ctx,
    from_module: NodeIndex,
    to_module: NodeIndex,
    use_path: &syn::UsePath,
) -> AResult<()> {
    if let Ok(next_index) = resolve_next_segment(ctx, to_module, to_module, &use_path.ident) {
        process_use_tree(ctx, from_module, next_index, &use_path.tree)?;
    };

    Ok(())
}

fn process_extern_crate(
    ctx: &mut Ctx,
    parent_index: NodeIndex,
    extern_crate: &syn::ItemExternCrate,
) -> AResult<()> {
    for edge_index in ctx.graph().edge_indices().collect::<Vec<_>>() {
        let edge = ctx
            .graph()
            .edge_weight(edge_index)
            .context("Couldn't get edge weight")?;

        if edge.name.as_ref().map(|name| name.to_string()) == Some(extern_crate.ident.to_string()) {
            let name = extern_crate
                .rename
                .as_ref()
                .map_or(extern_crate.ident.clone(), |(_, rename)| rename.clone())
                .clone();

            let Some((_, node_index)) = ctx.graph().edge_endpoints(edge_index) else {
                bail!("Couldn't get edge endpoints");
            };

            update_edge(
                ctx,
                parent_index,
                node_index,
                AnalysisEdge::new(false, Some(name.clone())),
            )?;

            bail!("extern crate not implemented");
        }
    }

    Ok(())
}
