use anyhow::Context;
use petgraph::graph::NodeIndex;
use syn::{Item, PathArguments, PathSegment, UseTree};

use crate::{
    AResult,
    analysis::{AnalysisEdge, Ctx},
    analysis_entry::AnalysisEntry,
    crate_graph::{find_neighbor, update_edge},
    resolve::resolve_next_segment,
};

pub fn process_use_statements(ctx: &mut Ctx) -> AResult<()> {
    let mut bfs = ctx.bfs()?;
    let mut to_process = vec![];
    while let Some(node_index) = bfs.next(&ctx.graph()) {
        if let AnalysisEntry::Mod(module) = ctx.entry(node_index)?
            && let Some(content) = module.content()
        {
            for item in content {
                if let Item::Use(use_statement) = item {
                    let use_statement = use_statement.clone();
                    to_process.push((node_index, use_statement));
                }
            }
        }
    }

    for (node_index, use_statement) in to_process {
        process_use_tree(ctx, node_index, node_index, &use_statement.tree)?;
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
            if let Ok(next_index) = resolve_next_segment(
                ctx,
                to_module,
                to_module,
                &PathSegment {
                    ident: use_name.ident.clone(),
                    arguments: PathArguments::None,
                },
            ) {
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
    if let Ok(next_index) = resolve_next_segment(
        ctx,
        to_module,
        to_module,
        &PathSegment {
            ident: use_path.ident.clone(),
            arguments: PathArguments::None,
        },
    ) {
        process_use_tree(ctx, from_module, next_index, &use_path.tree)?;
    };

    Ok(())
}
