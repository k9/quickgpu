use anyhow::bail;
use petgraph::{graph::NodeIndex, visit::Bfs};
use syn::{Item, UseTree};

use crate::{
    AResult, Analysis,
    analysis::{
        AnalysisEdge, AnalysisEntry, AnalysisRef, AnalysisRefMut, find_neighbor, update_edge,
    },
    process::resolve_next_segment,
};

pub fn process_use_statements(analysis: &mut Analysis, parent_mod: NodeIndex) -> AResult<()> {
    let mut bfs = Bfs::new(&analysis.graph, parent_mod);
    while let Some(node_index) = bfs.next(&analysis.graph) {
        if let AnalysisRef::Mod(module) = AnalysisEntry::node_index_ref(analysis, node_index)? {
            let module = module.clone();

            if let Some((_, items)) = &module.item.content {
                for item in items {
                    if let Item::Use(use_statement) = item {
                        let use_statement = use_statement.clone();
                        process_use_tree(analysis, node_index, node_index, &use_statement.tree)?;
                    } else if let Item::ExternCrate(extern_crate) = item {
                        let extern_crate = extern_crate.clone();
                        process_extern_crate(analysis, node_index, &extern_crate)?;
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
) -> AResult<()> {
    match &use_subtree {
        syn::UseTree::Path(use_path) => {
            process_use_path(analysis, from_module, to_module, use_path)?;
        }
        syn::UseTree::Group(use_group) => {
            for item in &use_group.items {
                process_use_tree(analysis, from_module, to_module, item)?;
            }
        }
        syn::UseTree::Name(use_name) => {
            if let Ok(next_index) = resolve_next_segment(analysis, to_module, &use_name.ident) {
                update_edge(
                    analysis,
                    from_module,
                    next_index,
                    AnalysisEdge {
                        from_use_statement: true,
                        from_extern_crate: false,
                        rename: None,
                    },
                );
            };
        }
        syn::UseTree::Glob(_) => {
            let neighbors = analysis.graph.neighbors(to_module).collect::<Vec<_>>();

            for neighbor in neighbors {
                update_edge(
                    analysis,
                    from_module,
                    neighbor,
                    AnalysisEdge {
                        from_use_statement: true,
                        from_extern_crate: false,
                        rename: None,
                    },
                );
            }
        }
        syn::UseTree::Rename(use_rename) => {
            if let Some(neighbor) = find_neighbor(&analysis, to_module, &use_rename.ident) {
                update_edge(
                    analysis,
                    from_module,
                    neighbor,
                    AnalysisEdge {
                        from_use_statement: true,
                        from_extern_crate: false,
                        rename: Some(use_rename.rename.clone()),
                    },
                );
            }
        }
    };

    Ok(())
}

fn process_use_path(
    analysis: &mut Analysis,
    from_module: NodeIndex,
    to_module: NodeIndex,
    use_path: &syn::UsePath,
) -> AResult<()> {
    if let Ok(next_index) = resolve_next_segment(analysis, to_module, &use_path.ident) {
        process_use_tree(analysis, from_module, next_index, &use_path.tree)?;
    };

    Ok(())
}

fn process_extern_crate(
    analysis: &mut Analysis,
    parent_index: NodeIndex,
    extern_crate: &syn::ItemExternCrate,
) -> AResult<()> {
    for node_index in analysis.graph.node_indices().collect::<Vec<_>>() {
        if let AnalysisRef::Mod(module) = AnalysisEntry::node_index_ref(analysis, node_index)?
            && module.crate_root.is_some()
            && module.item.ident.to_string() == extern_crate.ident.to_string()
        {
            let rename = extern_crate
                .rename
                .as_ref()
                .map_or(extern_crate.ident.clone(), |(_, rename)| rename.clone())
                .clone();

            update_edge(
                analysis,
                parent_index,
                node_index,
                AnalysisEdge {
                    from_use_statement: false,
                    from_extern_crate: true,
                    rename: Some(rename.clone()),
                },
            );

            let Ok(AnalysisRefMut::Mod(parent)) =
                AnalysisEntry::node_index_ref_mut(analysis, parent_index)
            else {
                bail!("Couldn't get parent node for extern crate");
            };

            if let Some(crate_root) = parent.crate_root.as_mut() {
                crate_root
                    .extern_prelude
                    .insert(rename.to_string(), node_index);
            }
        }
    }

    Ok(())
}
