use clap::{Command, Parser};

use crate::cli::{Cli, Commands};

mod cli;
mod commands;

pub fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    run(args)
}
pub fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Sync(args) => crate::commands::sync::run(args),
        Commands::Upload(args) => crate::commands::upload::run(args),
    }
}
