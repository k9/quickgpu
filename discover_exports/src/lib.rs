use petgraph::graph::NodeIndex;

pub use crate::analysis::{
    Analysis, AnalysisEdge, AnalysisEnum, AnalysisStruct, AnalysisTypeAlias,
};
use crate::exports::ExportedEntries;
pub use crate::exports::list_exports;
pub use crate::process::parse_crate;
use crate::process::{process_fields, process_impls};
use crate::use_statements::process_use_statements;

mod analysis;
mod crate_graph;
mod exports;
mod process;
pub mod types;
mod use_statements;
pub mod utils;

type AResult<T> = anyhow::Result<T>;

pub fn discover(analysis: &mut Analysis, root_index: NodeIndex) -> AResult<ExportedEntries> {
    discover_paths(analysis, root_index)?;

    process_impls(analysis, root_index)?;

    process_fields(analysis, root_index)?;

    list_exports(&analysis, root_index)
}

fn discover_paths(analysis: &mut Analysis, root_index: NodeIndex) -> AResult<()> {
    let mut num_edges: usize = analysis.graph.edge_count();
    loop {
        process_use_statements(analysis, root_index)?;
        let new_num_edges = analysis.graph.edge_count();
        if num_edges == new_num_edges {
            break;
        } else {
            num_edges = new_num_edges;
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{
        analysis::Analysis,
        crate_graph::print_dot,
        process::parse_crate,
        utils::{id, path_string, relative_path},
    };

    fn fixture() -> (Analysis, petgraph::prelude::NodeIndex) {
        let mut analysis = Analysis::default();

        let root_index = parse_crate(
            &mut analysis,
            relative_path("../expanded/wgpu.rs"),
            relative_path("test_workspace/test_lib"),
            "test_lib",
        )
        .unwrap();

        parse_crate(
            &mut analysis,
            relative_path("../expanded/wgpu_types.rs"),
            relative_path("test_workspace/test_lib_types"),
            "test_lib_types",
        )
        .unwrap();

        (analysis, root_index)
    }

    #[test]
    fn discover() {
        let (mut analysis, root_index) = fixture();
        print_dot(&analysis);

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

        assert_eq!(path_string(&resolution.1), "test_lib::abc::ZZ");

        let resolution = super::process::resolve_path(
            &mut analysis,
            root_index,
            resolution.0,
            &[&id("crate"), &id("abc"), &id("z"), &id("ZZ")],
        )
        .unwrap();

        assert_eq!(path_string(&resolution.1), "test_lib::abc::ZZ");

        let resolution =
            super::process::resolve_path(&mut analysis, root_index, resolution.0, &[&id("super")])
                .unwrap();

        assert_eq!(path_string(&resolution.1), "test_lib::abc");

        assert_eq!(
            path_string(
                &super::process::resolve_path(
                    &mut analysis,
                    root_index,
                    resolution.0,
                    &[&id("super")]
                )
                .unwrap()
                .1
            ),
            "test_lib"
        );

        assert_eq!(
            path_string(
                &super::process::resolve_path(
                    &mut analysis,
                    root_index,
                    resolution.0,
                    &[&id("super"), &id("tlt"), &id("counters"), &id("CounterA")]
                )
                .unwrap()
                .1
            ),
            "test_lib::tlt::CounterA"
        );
    }
}
