use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "opensync")]
#[command(about = "Example CLI application")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Sync(SyncArgs),
    Upload(SyncArgs),
}

#[derive(Args)]
pub struct SyncArgs {
    #[arg(short, long, default_value = ".")]
    pub path: PathBuf,
}
