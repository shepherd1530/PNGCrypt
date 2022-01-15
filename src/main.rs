mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use anyhow::{Result};
use clap::Parser;

fn main() -> Result<()> {
    let arguments = args::Args::parse();

    commands::Commands::from_args(arguments)?;

    Ok(())
}
