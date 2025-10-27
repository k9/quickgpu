use std::path::PathBuf;

use crate::{
    AResult,
    analysis::{Analysis, AnalysisEdge, AnalysisItem, AnalysisRef, AnalysisStruct, VecPushIndex},
    utils::{dummy_module, token_string, write_expanded},
};
use anyhow::{Context, bail};
use petgraph::{graph::NodeIndex, visit::Bfs};
use quote::{ToTokens, quote as q};
use syn::{Item, Type, Visibility};

pub fn parse_crate(
    analysis: &mut Analysis,
    output_path: PathBuf,
    crate_path: PathBuf,
    crate_name: &str,
) -> AResult<NodeIndex> {
    write_expanded(&output_path, &crate_path)?;

    let Ok(contents) = std::fs::read_to_string(output_path.clone()) else {
        bail!("Failed to read {:?}", &output_path);
    };

    let file = syn::parse_file(&contents)?;

    let crate_root = dummy_module(crate_name, file.items);
    let id = analysis.modules.push_index(crate_root);
    let root_index = analysis.graph.add_node(AnalysisItem::Mod(id));

    analysis.crates.insert(crate_name.to_string(), root_index);

    process_subtree(analysis, root_index)?;

    Ok(root_index)
}

pub fn process_subtree(analysis: &mut Analysis, parent_mod: NodeIndex) -> AResult<()> {
    let AnalysisRef::Mod(module) = AnalysisItem::node_index_ref(analysis, parent_mod)? else {
        bail!("Couldn't get node")
    };

    let Some((_, content)) = &module.content else {
        bail!("Couldn't get node")
    };

    let content = content.clone();

    for item in content.into_iter() {
        match item {
            Item::Mod(mod_item) => {
                let id = analysis.modules.push_index(mod_item);
                let child_mod = analysis.graph.add_node(AnalysisItem::Mod(id));
                analysis
                    .graph
                    .update_edge(parent_mod, child_mod, AnalysisEdge::Normal);

                process_subtree(analysis, child_mod)?;
            }
            Item::Struct(struct_item) => {
                let id = analysis.structs.push_index(AnalysisStruct {
                    item: struct_item.clone(),
                    impls: vec![],
                });

                let child = analysis.graph.add_node(AnalysisItem::Struct(id));
                analysis
                    .graph
                    .update_edge(parent_mod, child, AnalysisEdge::Normal);
            }
            Item::Type(type_item) => {
                let id = analysis.types.push_index(type_item.clone());
                let child = analysis.graph.add_node(AnalysisItem::Type(id));
                analysis
                    .graph
                    .update_edge(parent_mod, child, AnalysisEdge::Normal);
            }
            Item::Impl(type_impl) => {
                let id = analysis.impls.push_index(type_impl.clone());
                let child = analysis.graph.add_node(AnalysisItem::Impl(id));
                analysis
                    .graph
                    .update_edge(parent_mod, child, AnalysisEdge::Normal);
            }
            _ => (),
        };
    }

    Ok(())
}

pub fn process_impls(analysis: &mut Analysis, parent_mod: NodeIndex) -> AResult<()> {
    let mut bfs = Bfs::new(&analysis.graph, parent_mod);
    while let Some(node_index) = bfs.next(&analysis.graph) {
        if let AnalysisRef::Mod(module) = AnalysisItem::node_index_ref(analysis, node_index)? {
            let module = module.clone();
            if let Some((_, items)) = &module.content {
                for item in items {
                    if let Item::Impl(item) = item {
                        if let Some((_, _, _)) = &item.trait_
                            && let Type::Path(ty_path) = *item.self_ty.clone()
                        {
                            let impl_ty = &ty_path
                                .path
                                .segments
                                .last()
                                .context("Couldn't process path")?
                                .ident
                                .clone()
                                .into_token_stream()
                                .to_string();

                            if let Some(struct_match) =
                                analysis.structs.iter().find(|&struct_item| {
                                    let ident = &struct_item.item.ident;
                                    let generics = &struct_item.item.generics;
                                    &q!(#ident #generics).to_string() == impl_ty
                                })
                            {
                                println!("{:?}", struct_match.item.ident);
                            } else {
                                println!("none {:?}", impl_ty);
                            };
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn find_neighbor<'a>(
    analysis: &'a Analysis,
    item_tree_node: NodeIndex,
    ident: &syn::Ident,
) -> Option<(NodeIndex, AnalysisRef<'a>)> {
    let neighbors = analysis.graph.neighbors(item_tree_node);
    for neighbor in neighbors {
        match AnalysisItem::node_index_ref(analysis, neighbor) {
            Ok(AnalysisRef::Struct(struct_item)) => {
                if token_string(&struct_item.item.ident) == token_string(&ident) {
                    return Some((neighbor, AnalysisRef::Struct(struct_item)));
                }
            }
            Ok(AnalysisRef::Type(type_item)) => {
                if token_string(&type_item.ident) == token_string(&ident) {
                    return Some((neighbor, AnalysisRef::Type(type_item)));
                }
            }
            Ok(AnalysisRef::Mod(mod_item)) => {
                if token_string(&mod_item.ident) == token_string(&ident) {
                    return Some((neighbor, AnalysisRef::Mod(mod_item)));
                }
            }
            _ => (),
        }
    }

    None
}

pub fn keep_only_pub(analysis: &mut Analysis, node_index: NodeIndex) -> AResult<()> {
    let to_keep = keep_only_gather(analysis, node_index)?;
    let node_indices = analysis.graph.node_indices().collect::<Vec<_>>();
    for node_index in node_indices {
        if !to_keep.contains(&node_index) {
            analysis.graph.remove_node(node_index);
        }
    }

    Ok(())
}

fn keep_only_gather(analysis: &mut Analysis, node_index: NodeIndex) -> AResult<Vec<NodeIndex>> {
    let mut to_keep = vec![node_index];
    let neighbors = analysis.graph.neighbors(node_index).collect::<Vec<_>>();

    for neighbor in neighbors {
        if let Some(item) = analysis.graph.node_weight(neighbor) {
            match item.get_ref(analysis) {
                Ok(AnalysisRef::Struct(struct_item)) => {
                    if matches!(struct_item.item.vis, Visibility::Public(_)) {
                        to_keep.push(neighbor);
                    }
                }
                Ok(AnalysisRef::Type(struct_item)) => {
                    if matches!(struct_item.vis, Visibility::Public(_)) {
                        to_keep.push(neighbor);
                    }
                }
                Ok(AnalysisRef::Mod(mod_item)) => {
                    if matches!(mod_item.vis, Visibility::Public(_)) {
                        let mut descendants = keep_only_gather(analysis, neighbor)?;
                        to_keep.append(&mut descendants);
                    }
                }
                _ => (),
            }
        }
    }

    Ok(to_keep)
}
