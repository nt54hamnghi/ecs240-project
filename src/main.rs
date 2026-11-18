use std::path::Path;

use clap::Parser;

use crate::cli::{Cli, Command};

mod cli;

fn main() -> color_eyre::Result<()> {
    let args = Cli::parse();
    match args.command {
        Command::Check => check(),
        Command::Run { path } => match path {
            Some(path) => run_file(&path),
            None => repl(),
        },
    }
}

fn run() -> color_eyre::Result<()> {
    todo!()
}

fn run_file(path: &Path) -> color_eyre::Result<()> {
    todo!()
}

fn repl() -> color_eyre::Result<()> {
    todo!()
}

fn check() -> color_eyre::Result<()> {
    todo!()
}
