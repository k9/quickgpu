use std::{fs, path::Path};

use anyhow::Context;
use clap::{Parser, Subcommand};
use jsonpath_rust::JsonPath;
use serde_json::Value;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Generate => {
            let path =
                Path::new(env!("CARGO_MANIFEST_DIR")).join("../quickgpu/wgpu/wgpu/Cargo.toml");

            dbg!(&path);

            let json_path = rustdoc_json::Builder::default()
                .toolchain("nightly")
                .manifest_path(path)
                .build()
                .unwrap();

            {
                let json_src = fs::read_to_string(json_path).unwrap();
                let json: serde_json::Value = serde_json::from_str(&json_src).unwrap();

                //let json: serde_json::Value =
                //    serde_json::from_str("{ \"index\": { \"A\": 4 }}").unwrap();

                let structs = json.query("$.index[?(@.inner.struct.kind.plain)]").unwrap();

                for x in structs {
                    process_struct(&json, x).expect("Error processing struct");
                }
            }
        }
    };
}

/*
{
  "attrs": [],
  "crate_id": 0,
  "deprecation": null,
  "docs": "Advanced options for use when a pipeline is compiled\n\nThis implements `Default`, and for most users can be set to `Default::defaul
  "id": 983,
  "inner": {
    "struct": {
      "generics": {
        "params": [
          {
            "kind": {
              "lifetime": {
                "outlives": []
              }
            },
            "name": "'a"
          }
        ],
        "where_predicates": []
      },
      "impls": [
        984,
      ],
      "kind": {
        "plain": {
          "fields": [
            981,
            982
          ],
          "has_stripped_fields": false
        }
      }
    }
  },
  "links": {},
  "name": "PipelineCompilationOptions",
  "span": ...,
  "visibility": "public"
}
*/
fn process_struct(doc: &serde_json::Value, x: &serde_json::Value) -> anyhow::Result<()> {
    let has_private_fields = query_one(x, "$.inner.struct.kind.plain.has_stripped_fields")?;

    if has_private_fields == false {
        let mut name = query_one(x, "$.name")?
            .as_str()
            .context("Struct with no name")?
            .to_string();

        let generics: Vec<String> = query(x, "$.inner.struct.generics.params[*].name")?
            .into_iter()
            .map(|v| v.as_str().expect("Generic with no name").to_string())
            .collect();

        if !generics.is_empty() {
            name = format!("{}<{}>", name, generics.join(", "));
        }

        println!("{name}");
        let fields = query_one(x, "$.inner.struct.kind.plain.fields")?;

        for field in fields.as_array().context("Fields is not array")? {
            let field = query_one(doc, &format!("$.index['{field}'].inner.struct_field"))?;

            println!("    {field}");
        }
        println!();
    };

    Ok(())
}

fn query<'v>(v: &'v Value, q: &str) -> anyhow::Result<Vec<&'v Value>> {
    v.query(q).context("query_one failed")
}

fn query_one<'v>(v: &'v Value, q: &str) -> anyhow::Result<&'v Value> {
    Ok(v.query(q).context("query_one failed")?[0])
}
