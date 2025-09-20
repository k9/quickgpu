use std::{collections::HashMap, fs, process::Stdio};

use anyhow::anyhow;
use cargo_metadata::Message;
use clap::{Parser, Subcommand, command};
use scraper::Html;

use crate::{
    generator::Generator,
    idl_parser::parse_idl,
    utils::{doc_path, output},
};

pub mod customize;
pub mod generator;
pub mod idl_parser;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate,
    DefaultBuild,
}

fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    match &cli.command {
        Commands::Generate => {
            let mut idl = include_str!("../../webgpu.idl");

            let idl_hints = parse_idl(&mut idl)
                .map_err(|e| anyhow!(e))?
                .into_iter()
                .collect::<HashMap<_, _>>();

            let path = doc_path("index.html")?;
            let mut generator =
                Generator::new(Html::parse_document(&fs::read_to_string(path)?), idl_hints);

            output(&generator.generate()?, true);
        }
        Commands::DefaultBuild => {
            let mut command = std::process::Command::new("cargo")
                .args(["build", "-p", "quickgpu", "--message-format=json"])
                .stderr(Stdio::null())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            let reader = std::io::BufReader::new(command.stdout.take().unwrap());
            for message in cargo_metadata::Message::parse_stream(reader) {
                if let Message::CompilerMessage(compiler_message) = message.unwrap() {
                    if compiler_message
                        .message
                        .code
                        .as_ref()
                        .is_some_and(|c| c.code == "E0599")
                    {
                        println!(
                            "{:?}",
                            compiler_message.message.spans.first().unwrap().line_start
                        );
                    }
                }
            }

            let output = command.wait().expect("Couldn't get cargo's exit status");
        }
    };
    Ok(())
}
