use std::{collections::HashMap, fs};

use anyhow::anyhow;
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
}

fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    match &cli.command {
        Commands::Generate => {}
    };

    let mut idl = include_str!("../../webgpu.idl");

    let idl_hints = parse_idl(&mut idl)
        .map_err(|e| anyhow!(e))?
        .into_iter()
        .collect::<HashMap<_, _>>();

    let path = doc_path("index.html")?;
    let mut generator = Generator::new(Html::parse_document(&fs::read_to_string(path)?), idl_hints);

    output(&generator.generate()?, true);

    Ok(())
}
