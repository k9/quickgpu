use anyhow::Context;
use petgraph::{algo::astar, graph::NodeIndex, visit::Bfs};

use crate::{
    AResult, Analysis,
    analysis::{
        AnalysisEntry, AnalysisEnum, AnalysisRef, AnalysisStruct, AnalysisTypeAlias, HasPath,
    },
};

fn sort_shortest_first(exported_list: &mut Vec<impl HasPath>) {
    exported_list.sort_by(|a, b| a.get_path().join("").cmp(&b.get_path().join("")));
    exported_list.sort_by(|a, b| a.get_path().len().cmp(&b.get_path().len()));
}

#[derive(Default)]
pub struct ExportedEntries {
    pub structs: Vec<AnalysisStruct>,
    pub enums: Vec<AnalysisEnum>,
    pub types: Vec<AnalysisTypeAlias>,
}

pub fn list_exports<'a>(analysis: &'a Analysis, root_index: NodeIndex) -> AResult<ExportedEntries> {
    let mut exports = ExportedEntries::default();
    let mut bfs = Bfs::new(&analysis.graph, root_index);
    while let Some(node_index) = bfs.next(&analysis.graph) {
        if let Some(path) = item_path(analysis, root_index, node_index)? {
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
                _ => (),
            };
        }
    }

    sort_shortest_first(&mut exports.structs);
    sort_shortest_first(&mut exports.enums);
    sort_shortest_first(&mut exports.types);

    Ok(exports)
}

pub fn item_path<'a>(
    analysis: &'a Analysis,
    root_index: NodeIndex,
    node_index: NodeIndex,
) -> AResult<Option<Vec<String>>> {
    let graph_path = astar(
        &analysis.graph,
        root_index,
        |x| x == node_index,
        |_| 1,
        |_| 0,
    )
    .context("Couldn't get path")?;

    let mut path = vec![];
    let mut previous_segment = None;
    for segment_index in graph_path.1 {
        let name = match AnalysisEntry::node_index_ref(analysis, segment_index)? {
            AnalysisRef::Struct(struct_entry) => Some(struct_entry.item.ident.to_string()),
            AnalysisRef::Enum(enum_entry) => Some(enum_entry.item.ident.to_string()),
            AnalysisRef::Type(type_entry) => Some(type_entry.item.ident.to_string()),
            AnalysisRef::Impl(_) => None,
            AnalysisRef::Mod(mod_entry) => Some(mod_entry.ident.to_string()),
        };

        if let Some(mut name) = name {
            if let Some(previous_segment) = previous_segment
                && let Some(edge) = analysis.graph.find_edge(previous_segment, previous_segment)
                && let Some(Some(edge)) = analysis.graph.edge_weight(edge).map(|e| &e.rename)
            {
                name = edge.clone();
            };

            path.push(name);

            previous_segment = Some(segment_index);
        }
    }

    Ok(Some(path))
}
