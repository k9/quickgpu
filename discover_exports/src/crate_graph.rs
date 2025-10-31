use petgraph::dot::{Config, Dot};
use quote::{ToTokens, quote as q};

use crate::analysis::{Analysis, AnalysisEntry};

#[allow(dead_code)]
pub fn print_dot(analysis: &Analysis) {
    println!(
        "{:?}",
        Dot::with_attr_getters(
            &analysis.graph,
            &[Config::EdgeNoLabel, Config::NodeNoLabel],
            &|_, edge| {
                let label = edge
                    .weight()
                    .rename
                    .as_ref()
                    .map_or("".to_string(), |x| x.to_string());

                format!("label = \"{}\"", label)
            },
            &|_, (_, entry)| {
                let label = match entry {
                    AnalysisEntry::Struct(id) => {
                        let entry = &analysis.structs[*id];
                        let name = &entry.item.ident;
                        let vis = &entry.item.vis;

                        q!(#vis struct #name).to_string()
                    }
                    AnalysisEntry::Enum(id) => {
                        let entry = &analysis.enums[*id];
                        let name = &entry.item.ident;
                        let vis = &entry.item.vis;

                        q!(#vis enum #name).to_string()
                    }
                    AnalysisEntry::Type(id) => {
                        let entry = &analysis.types[*id];
                        let name = &entry.item.ident;
                        let vis = &entry.item.vis;

                        q!(#vis type #name).to_string()
                    }
                    AnalysisEntry::Trait(id) => {
                        let entry = &analysis.traits[*id];
                        let name = &entry.item.ident;
                        let vis = &entry.item.vis;

                        q!(#vis trait #name).to_string()
                    }
                    AnalysisEntry::Impl(id) => {
                        let entry = &analysis.impls[*id];
                        let name = entry.trait_.as_ref().map(|t| {
                            t.1.segments
                                .iter()
                                .map(|s| s.into_token_stream().to_string())
                                .collect::<Vec<_>>()
                        });

                        format!("{name:?}").replace("\"", "\\\"")
                    }
                    AnalysisEntry::Mod(id) => {
                        let entry = &analysis.modules[*id];
                        let name = &entry.item.ident;
                        let vis = &entry.item.vis;

                        q!(#vis mod #name).to_string()
                    }
                    AnalysisEntry::None => "none".to_string(),
                };

                format!("label = \"{}\"", label)
            }
        )
    );
}
