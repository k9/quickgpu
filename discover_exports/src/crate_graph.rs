use petgraph::dot::{Config, Dot};

use crate::analysis::{Analysis, AnalysisItem};

#[allow(dead_code)]
pub fn print_dot(analysis: &Analysis) {
    println!(
        "{:?}",
        Dot::with_attr_getters(
            &analysis.graph,
            &[Config::EdgeNoLabel, Config::NodeNoLabel],
            &|_, _| "".to_string(),
            &|_, (_, s)| {
                let label = match s {
                    AnalysisItem::Struct(id) => {
                        let item = &analysis.structs[*id];
                        let name = &item.item.ident;

                        format!("struct {name}")
                    }
                    AnalysisItem::Enum(id) => {
                        let item = &analysis.enums[*id];
                        let name = &item.item.ident;

                        format!("enum {name}")
                    }
                    AnalysisItem::Type(id) => {
                        let item = &analysis.types[*id];
                        let name = &item.item.ident;

                        format!("{name}")
                    }
                    AnalysisItem::Impl(id) => {
                        let item = &analysis.impls[*id];
                        let name = &item.trait_;

                        format!("{name:?}")
                    }
                    AnalysisItem::Mod(id) => {
                        let item = &analysis.modules[*id];
                        let name = &item.ident;

                        format!("{name}")
                    }
                    AnalysisItem::None => "none".to_string(),
                };

                format!("label = \"{}\"", label)
            }
        )
    );
}
