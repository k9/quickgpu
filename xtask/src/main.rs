pub mod analyze;
pub mod data;
pub mod generate;
pub mod type_alias_helpers;
pub mod utils;

use clap::{Parser, Subcommand};
use duct::cmd;

use crate::utils::relative_path;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build,
    Generate,
    Document,
}

type AResult<T> = anyhow::Result<T>;

fn main() -> AResult<()> {
    env_logger::init();

    let cli = <Cli as clap::Parser>::parse();
    match &cli.command {
        Commands::Build => build(),
        Commands::Generate => generate::generate(),
        Commands::Document => document(),
    }?;

    Ok(())
}

pub fn build() -> AResult<()> {
    generate()?;
    document()?;
    Ok(())
}

pub fn generate() -> AResult<()> {
    generate::generate()?;
    Ok(())
}

pub fn document() -> AResult<()> {
    cmd!("cargo", "doc").dir(relative_path("quickgpu")).run()?;
    Ok(())
}
