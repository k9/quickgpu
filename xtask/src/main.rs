pub mod analyze;
pub mod data;
pub mod generate;
pub mod output;
pub mod type_alias_helpers;
pub mod utils;

use clap::{Parser, Subcommand};

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
    env_logger::init();

    let cli = <Cli as clap::Parser>::parse();
    match &cli.command {
        Commands::Generate => generate::generate(),
    }?;

    Ok(())
}
