use std::collections::HashSet;

use petgraph::graph::NodeIndex;

pub use crate::analysis::{
    Analysis, AnalysisEdge, AnalysisEnum, AnalysisStruct, AnalysisTypeAlias,
};
use crate::exports::ExportedEntries;
pub use crate::exports::list_exports;
pub use crate::process::parse_crate;
use crate::process::{keep_only_pub, process_impls};
use crate::use_statements::process_use_statements;

mod analysis;
mod crate_graph;
mod exports;
mod process;
mod use_statements;
mod utils;

type AResult<T> = anyhow::Result<T>;

pub fn discover(analysis: &mut Analysis, root_index: NodeIndex) -> AResult<ExportedEntries> {
    let skipped_mods = discover_paths(analysis, root_index)?;

    process_impls(analysis, root_index)?;

    println!(
        "skipped mod names: {}",
        skipped_mods.into_iter().collect::<Vec<_>>().join(", ")
    );

    keep_only_pub(analysis, root_index)?;
    list_exports(&analysis, root_index)
}

fn discover_paths(analysis: &mut Analysis, root_index: NodeIndex) -> AResult<HashSet<String>> {
    let mut skipped_mods = HashSet::new();
    let mut num_edges: usize = analysis.graph.edge_count();
    loop {
        process_use_statements(analysis, root_index, &mut skipped_mods)?;
        let new_num_edges = analysis.graph.edge_count();
        if num_edges == new_num_edges {
            break;
        } else {
            num_edges = new_num_edges;
        }
    }

    Ok(skipped_mods)
}

#[cfg(test)]
mod test {
    use crate::{
        analysis::{Analysis, AnalysisEdge},
        process::parse_crate,
        utils::{id, relative_path},
    };

    fn fixture() -> (Analysis, petgraph::prelude::NodeIndex) {
        let mut analysis = Analysis::default();

        let root_index = parse_crate(
            &mut analysis,
            relative_path("../expanded/wgpu.rs"),
            relative_path("test_workspace/test_lib"),
            "crate",
        )
        .unwrap();

        let root_types_index = parse_crate(
            &mut analysis,
            relative_path("../expanded/wgpu_types.rs"),
            relative_path("test_workspace/test_lib_types"),
            "tlt",
        )
        .unwrap();

        analysis.graph.update_edge(
            root_index,
            root_types_index,
            AnalysisEdge {
                from_use_statement: true,
                rename: None,
            },
        );
        (analysis, root_index)
    }

    #[test]
    fn discover() {
        let (mut analysis, root_index) = fixture();

        let exports = super::discover(&mut analysis, root_index).unwrap();
        assert_eq!(exports.structs.len(), 8);
        assert_eq!(exports.types.len(), 1);
    }

    #[test]
    fn resolve_path() {
        let (mut analysis, root_index) = fixture();
        super::discover_paths(&mut analysis, root_index).unwrap();

        let resolution = super::process::resolve_path(
            &mut analysis,
            root_index,
            root_index,
            &[&id("abc"), &id("z"), &id("ZZ")],
        )
        .unwrap();

        assert_eq!(resolution.1, ["crate", "abc", "ZZ"]);

        let resolution = super::process::resolve_path(
            &mut analysis,
            root_index,
            resolution.0,
            &[&id("crate"), &id("abc"), &id("z"), &id("ZZ")],
        )
        .unwrap();

        assert_eq!(resolution.1, ["crate", "abc", "ZZ"]);

        let resolution =
            super::process::resolve_path(&mut analysis, root_index, resolution.0, &[&id("super")])
                .unwrap();

        assert_eq!(resolution.1, ["crate", "abc"]);

        let resolution =
            super::process::resolve_path(&mut analysis, root_index, resolution.0, &[&id("super")])
                .unwrap();

        assert_eq!(resolution.1, ["crate"]);
    }
}
