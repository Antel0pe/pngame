// use anyhow::Result;
use clap::Parser;
use crate::{commands::process_cli_args, args::Cli};

mod chunk_type;
mod chunk;
mod png;
mod args;
mod commands;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Cli::parse();

    process_cli_args(args);

    Ok(())
}