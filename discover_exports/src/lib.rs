use petgraph::graph::NodeIndex;

pub use crate::analysis::{
    Analysis, AnalysisEdge, AnalysisEnum, AnalysisStruct, AnalysisTypeAlias,
};
pub use crate::process::parse_crate;
use crate::process::{process_fields, process_impls};
use crate::use_statements::process_use_statements;

pub mod analysis;
pub mod crate_graph;
mod exports;
mod process;
pub mod types;
mod use_statements;
pub mod utils;

type AResult<T> = anyhow::Result<T>;

pub fn discover(analysis: &mut Analysis, root_index: NodeIndex) -> AResult<()> {
    discover_paths(analysis, root_index)?;
    process_impls(analysis, root_index)?;
    process_fields(analysis, root_index)?;
    Ok(())
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
    use std::collections::HashMap;

    use crate::{
        analysis::{Analysis, AnalysisEntry, AnalysisRef},
        crate_graph::{filter_map_nodes, for_each_node, keep_only_public, node_ident, print_dot},
        process::{parse_crate, process_crate},
        utils::{id, path_string, relative_path},
    };

    use quote::quote as q;
    use syn::{ImplItem, ImplItemConst};

    use super::discover;

    fn test_workspace() -> (Analysis, petgraph::prelude::NodeIndex) {
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
    fn test_discover() {
        let (mut analysis, root_index) = test_workspace();

        discover(&mut analysis, root_index).unwrap();
        keep_only_public(&mut analysis, root_index).unwrap();
        print_dot(&analysis);

        assert_eq!(
            filter_map_nodes(&analysis, root_index, |index| matches!(
                AnalysisEntry::node_index_ref(&analysis, index).unwrap(),
                AnalysisRef::Struct(_)
            )
            .then_some(index))
            .count(),
            5
        );

        let mut consts: HashMap<String, Vec<String>> = HashMap::new();
        for_each_node(&analysis, root_index, |index| {
            if let AnalysisRef::Struct(entry) =
                AnalysisEntry::node_index_ref(&analysis, index).unwrap()
            {
                println!("{:?}", node_ident(&analysis, root_index, index));

                consts.insert(
                    node_ident(&analysis, root_index, index)
                        .unwrap()
                        .to_string(),
                    struct_consts(entry)
                        .iter()
                        .map(|c| c.ident.to_string())
                        .collect::<Vec<_>>(),
                );
            }
        });

        assert_eq!(consts.len(), 5);

        let abc_consts = consts.get("A").unwrap();
        assert_eq!(abc_consts.len(), 0);

        let abc_consts = consts.get("Abc").unwrap();
        assert_eq!(abc_consts.len(), 2);
        assert!(abc_consts.contains(&"XYZ".to_string()));
        assert!(abc_consts.contains(&"ZZZZ".to_string()));
    }

    fn struct_consts(entry: &crate::AnalysisStruct) -> Vec<&ImplItemConst> {
        let mut consts = vec![];
        for impl_item in &entry.impls {
            for item in &impl_item.items {
                if let ImplItem::Const(c) = item {
                    consts.push(c);
                };
            }
        }

        consts
    }

    #[test]
    fn resolve_path() {
        let (mut analysis, root_index) = test_workspace();
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

    #[test]
    fn type_alias() {
        let mut analysis = Analysis::default();

        let root_index = process_crate(
            &mut analysis,
            "base",
            q!(
                extern crate extra as ext;
                pub mod api {
                    pub use ext::A as ABase;
                    pub type A = ABase;
                }
                pub use api::*;
            )
            .to_string(),
        )
        .unwrap();

        process_crate(
            &mut analysis,
            "extra",
            q!(
                pub struct A {}
            )
            .to_string(),
        )
        .unwrap();

        super::discover_paths(&mut analysis, root_index).unwrap();

        keep_only_public(&mut analysis, root_index).unwrap();
        print_dot(&analysis);

        let mut structs = filter_map_nodes(&analysis, root_index, |node_index| {
            if let Ok(AnalysisRef::Struct(_)) = AnalysisEntry::node_index_ref(&analysis, node_index)
            {
                Some(node_index)
            } else {
                None
            }
        });

        assert_eq!(
            node_ident(&analysis, root_index, structs.next().unwrap())
                .unwrap()
                .to_string(),
            "ABase"
        );

        assert!(structs.next().is_none());

        /*let mut types = filter_nodes(&analysis, root_index, |node_index| {
            if let Ok(AnalysisRef::Type(_)) = AnalysisEntry::node_index_ref(&analysis, node_index) {
                Some(node_index)
            } else {
                None
            }
        });


        assert_eq!(
            node_ident(&analysis, root_index, types.next().unwrap())
                .unwrap()
                .to_string(),
            "A"
        );

        assert!(structs.next().is_none());*/
    }
}
