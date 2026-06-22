pub mod generate;
pub mod type_helpers;
pub mod utils;

use clap::{Parser, Subcommand};
use duct::cmd;

use crate::generate::Version;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        #[arg(value_enum)]
        version_filter: Option<Version>,
    },
    Generate {
        #[arg(value_enum)]
        version_filter: Option<Version>,
    },
    Document {
        #[arg(value_enum)]
        version_filter: Option<Version>,
    },
}

type AResult<T> = anyhow::Result<T>;

fn main() -> AResult<()> {
    env_logger::init();

    let cli = <Cli as clap::Parser>::parse();
    match &cli.command {
        Commands::Build { version_filter } => build(*version_filter),
        Commands::Generate { version_filter } => generate(*version_filter),
        Commands::Document { version_filter } => document(*version_filter),
    }?;

    Ok(())
}

pub(crate) fn versions(version_filter: Option<Version>) -> Vec<Version> {
    match version_filter {
        Some(Version::V27) => vec![Version::V27],
        Some(Version::V28) => vec![Version::V28],
        Some(Version::V29) => vec![Version::V29],
        None => vec![Version::V27, Version::V28, Version::V29],
    }
}

pub(crate) fn build(version_filter: Option<Version>) -> AResult<()> {
    for version in versions(version_filter) {
        generate(Some(version))?;
    }

    document(version_filter)?;
    Ok(())
}

pub(crate) fn generate(version_filter: Option<Version>) -> AResult<()> {
    for version in versions(version_filter) {
        generate::generate(version)?;
    }
    Ok(())
}

pub(crate) fn document(version_filter: Option<Version>) -> AResult<()> {
    let version_filter = version_filter.unwrap_or(Version::latest());

    cmd!(
        "cargo",
        "+nightly",
        "doc",
        "-p",
        "quickgpu",
        "--no-deps",
        "--features",
        version_filter.version_mod().to_string()
    )
    .env("RUSTDOCFLAGS", "--cfg docsrs")
    .run()?;

    Ok(())
}
