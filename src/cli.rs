use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "alpha")]
#[command(author, version, about, long_about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Run an Alpha script
    #[command(visible_alias = "r")]
    Run {
        /// Path to the Alpha script to run
        path: Option<PathBuf>,
    },
    /// Check an Alpha script for errors
    #[command(visible_alias = "c")]
    Check,
}
