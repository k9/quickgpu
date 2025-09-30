use clap::{Parser, Subcommand, command};

use crate::process::process;

pub mod default_overrides;
pub mod process;
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
    env_logger::init();

    let cli = <Cli as clap::Parser>::parse();
    match &cli.command {
        Commands::Generate => process()?,
    };
    Ok(())
}
