use petgraph::dot::{Config, Dot};
use quote::ToTokens;

use crate::analysis::{Analysis, AnalysisEntry};

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
                    AnalysisEntry::Struct(id) => {
                        let entry = &analysis.structs[*id];
                        let name = &entry.item.ident;

                        format!("struct {name}")
                    }
                    AnalysisEntry::Enum(id) => {
                        let entry = &analysis.enums[*id];
                        let name = &entry.item.ident;

                        format!("enum {name}")
                    }
                    AnalysisEntry::Type(id) => {
                        let entry = &analysis.types[*id];
                        let name = &entry.item.ident;

                        format!("{name}")
                    }
                    AnalysisEntry::Impl(id) => {
                        let entry = &analysis.impls[*id];
                        let name = entry.trait_.as_ref().map(|t| {
                            t.1.segments
                                .iter()
                                .map(|s| s.into_token_stream().to_string())
                                .collect::<Vec<_>>()
                        });

                        format!("{name:?}")
                    }
                    AnalysisEntry::Mod(id) => {
                        let entry = &analysis.modules[*id];
                        let name = &entry.ident;

                        format!("{name}")
                    }
                    AnalysisEntry::None => "none".to_string(),
                };

                format!("label = \"{}\"", label)
            }
        )
    );
}
