use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};

use crate::{changes::Changes, config::Config};

#[derive(Debug, Parser, Clone)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    /// Prints resulting Rust struct after evaluating Python config
    Eval(EvalArgs),
    /// Applies the configuration
    Apply(ApplyArgs),
}

#[derive(Debug, Args, Clone)]
pub struct EvalArgs {
    /// The path to the python file containing the configuration
    #[arg(short, long)]
    file: PathBuf,
}

#[derive(Debug, Args, Clone)]
pub struct ApplyArgs {
    /// Show the list of changes that would be made without applying them.
    #[arg(short, long)]
    dry_run: bool,
    /// The path to the file containing the configuration
    #[arg(short, long)]
    file: PathBuf,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        match self.command.clone() {
            Command::Eval(args) => {
                let config =
                    crate::engine::run(args.file).context("Error processing python config.")?;
                println!("{config:#?}");
            }
            Command::Apply(args) => {
                let config =
                    crate::engine::run(args.file).context("Error processing python config.")?;
                let applied_config = Config::read_saved_config();
                let changes = Changes::new(config.clone(), applied_config);
                if args.dry_run {
                    println!("{changes}");
                } else {
                    changes.apply()?;
                    config.save_config().unwrap();
                }
            }
        }

        Ok(())
    }
}
