use std::collections::HashSet;

use anyhow::Context;
use petgraph::{graph::NodeIndex, visit::Bfs};
use syn::{Item, UseTree};

use crate::{
    AResult, Analysis,
    analysis::{AnalysisEdge, AnalysisItem, AnalysisRef},
    analyze::find_neighbor,
    utils::token_string,
};

pub fn process_use_statements(
    analysis: &mut Analysis,
    parent_mod: NodeIndex,
    skipped_mods: &mut HashSet<String>,
) -> AResult<()> {
    let mut bfs = Bfs::new(&analysis.graph, parent_mod);
    while let Some(node_index) = bfs.next(&analysis.graph) {
        if let AnalysisRef::Mod(module) = AnalysisItem::node_index_ref(analysis, node_index)? {
            let module = module.clone();
            if let Some((_, items)) = &module.content {
                for item in items {
                    if let Item::Use(use_statement) = item {
                        let use_statement = use_statement.clone();
                        process_use_tree(
                            analysis,
                            node_index,
                            node_index,
                            &use_statement.tree,
                            skipped_mods,
                        )?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn process_use_tree(
    analysis: &mut Analysis,
    from_module: NodeIndex,
    to_module: NodeIndex,
    use_subtree: &UseTree,
    skipped_mods: &mut HashSet<String>,
) -> AResult<()> {
    match &use_subtree {
        syn::UseTree::Path(use_path) => {
            process_path(analysis, from_module, to_module, skipped_mods, use_path)?;
        }
        syn::UseTree::Group(use_group) => {
            for item in &use_group.items {
                process_use_tree(analysis, from_module, to_module, item, skipped_mods)?;
            }
        }
        syn::UseTree::Name(use_name) => {
            if let Some((neighbor, AnalysisRef::Struct(_))) =
                find_neighbor(&analysis, to_module, &use_name.ident)
            {
                analysis
                    .graph
                    .update_edge(from_module, neighbor, AnalysisEdge::Normal);
            }
        }
        syn::UseTree::Glob(_) => {
            let neighbors = analysis.graph.neighbors(to_module).collect::<Vec<_>>();

            for neighbor in neighbors {
                analysis
                    .graph
                    .update_edge(from_module, neighbor, AnalysisEdge::Normal);
            }
        }
        syn::UseTree::Rename(use_rename) => {
            if let Some((neighbor, AnalysisRef::Struct(_))) =
                find_neighbor(&analysis, to_module, &use_rename.ident)
            {
                analysis.graph.update_edge(
                    from_module,
                    neighbor,
                    AnalysisEdge::Rename(use_rename.rename.to_string()),
                );
            }
        }
    };

    Ok(())
}

fn process_path(
    analysis: &mut Analysis,
    from_module: NodeIndex,
    to_module: NodeIndex,
    skipped_mods: &mut HashSet<String>,
    use_path: &syn::UsePath,
) -> AResult<()> {
    if let Some((neighbor, AnalysisRef::Mod(_))) =
        find_neighbor(&analysis, to_module, &use_path.ident)
    {
        process_use_tree(
            analysis,
            from_module,
            neighbor,
            &use_path.tree,
            skipped_mods,
        )?;
    } else if let Some(krate) = analysis.crates.get(&token_string(&use_path.ident)) {
        process_use_tree(analysis, from_module, *krate, &use_path.tree, skipped_mods)?;
    } else if token_string(&use_path.ident) == "super" {
        let parent = analysis
            .graph
            .neighbors_directed(to_module, petgraph::Direction::Incoming)
            .next()
            .context("Couldn't get parent")?;

        process_use_tree(analysis, from_module, parent, &use_path.tree, skipped_mods)?;
    } else {
        skipped_mods.insert(use_path.ident.to_string());
    }

    Ok(())
}
