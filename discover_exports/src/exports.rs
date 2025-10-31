use anyhow::Context;
use petgraph::{
    algo::astar,
    graph::NodeIndex,
    visit::{Bfs, NodeFiltered},
};
use syn::Ident;

use crate::{
    AResult, Analysis,
    analysis::{
        AnalysisEntry, AnalysisEnum, AnalysisRef, AnalysisStruct, AnalysisTrait, AnalysisTypeAlias,
        HasPath,
    },
    utils::IsPublic,
};

fn sort_shortest_first(exported_list: &mut Vec<impl HasPath>) {
    exported_list.sort_by(|a, b| {
        a.get_path()
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join("::")
            .cmp(
                &b.get_path()
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join("::"),
            )
    });
    exported_list.sort_by(|a, b| a.get_path().len().cmp(&b.get_path().len()));
}

#[derive(Default)]
pub struct ExportedEntries {
    pub structs: Vec<AnalysisStruct>,
    pub enums: Vec<AnalysisEnum>,
    pub types: Vec<AnalysisTypeAlias>,
    pub traits: Vec<AnalysisTrait>,
}

pub fn list_exports<'a>(analysis: &'a Analysis, root_index: NodeIndex) -> AResult<ExportedEntries> {
    let mut exports = ExportedEntries::default();
    let mut bfs = Bfs::new(&analysis.graph, root_index);
    while let Some(node_index) = bfs.next(&analysis.graph) {
        if let Ok(path) = item_path(analysis, root_index, node_index, false) {
            match AnalysisEntry::node_index_ref(analysis, node_index)? {
                AnalysisRef::Struct(node) => {
                    let mut node = node.clone();
                    node.path = path;
                    exports.structs.push(node);
                }
                AnalysisRef::Enum(node) => {
                    let mut node = node.clone();
                    node.path = path;
                    exports.enums.push(node);
                }
                AnalysisRef::Type(node) => {
                    let mut node = node.clone();
                    node.path = path;
                    exports.types.push(node);
                }
                AnalysisRef::Trait(node) => {
                    let mut node = node.clone();
                    node.path = path;
                    exports.traits.push(node);
                }
                _ => (),
            };
        }
    }

    sort_shortest_first(&mut exports.structs);
    sort_shortest_first(&mut exports.enums);
    sort_shortest_first(&mut exports.types);
    sort_shortest_first(&mut exports.traits);

    Ok(exports)
}

pub fn item_path<'a>(
    analysis: &'a Analysis,
    root_index: NodeIndex,
    node_index: NodeIndex,
    allow_non_public: bool,
) -> AResult<Vec<Ident>> {
    let filtered = NodeFiltered::from_fn(&analysis.graph, |n| {
        allow_non_public
            || AnalysisEntry::node_index_ref(&analysis, n).is_ok_and(|n| match n {
                AnalysisRef::Struct(entry) => entry.item.vis.is_public(),
                AnalysisRef::Enum(entry) => entry.item.vis.is_public(),
                AnalysisRef::Type(entry) => entry.item.vis.is_public(),
                AnalysisRef::Trait(entry) => entry.item.vis.is_public(),
                AnalysisRef::Impl(_) => true,
                AnalysisRef::Mod(entry) => entry.item.vis.is_public(),
            })
    });

    let graph_path = astar(&filtered, root_index, |x| x == node_index, |_| 1, |_| 0)
        .context(format!("Couldn't get path {:?}", node_index))?;

    let mut path = vec![];
    let mut previous_segment = None;
    for segment_index in graph_path.1 {
        let name = match AnalysisEntry::node_index_ref(analysis, segment_index)? {
            AnalysisRef::Struct(entry) => Some(&entry.item.ident),
            AnalysisRef::Enum(entry) => Some(&entry.item.ident),
            AnalysisRef::Type(entry) => Some(&entry.item.ident),
            AnalysisRef::Trait(entry) => Some(&entry.item.ident),
            AnalysisRef::Impl(_) => None,
            AnalysisRef::Mod(entry) => Some(&entry.item.ident),
        };

        if let Some(mut name) = name {
            if let Some(previous_segment) = previous_segment
                && let Some(edge) = analysis.graph.find_edge(previous_segment, segment_index)
                && let Some(Some(edge)) = analysis.graph.edge_weight(edge).map(|e| &e.rename)
            {
                name = edge;
            };

            path.push(name.clone());

            previous_segment = Some(segment_index);
        }
    }

    Ok(path)
}
