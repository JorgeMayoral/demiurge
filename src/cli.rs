use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};

use crate::{changes::Changes, config::Config};

#[derive(Debug, Parser, Clone, Copy)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand, Clone, Copy)]
pub enum Command {
    /// Prints resulting Rust struct after evaluating Python config
    Eval,
    /// Applies the configuration
    Apply(ApplyArgs),
}

#[derive(Debug, Args, Clone, Copy)]
pub struct ApplyArgs {
    /// Show the list of changes that would be made without applying them.
    #[arg(short, long)]
    dry_run: bool,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        match self.command {
            Command::Eval => {
                let config = crate::engine::run().context("Error processing python config.")?;
                println!("{config:#?}");
            }
            Command::Apply(args) => {
                let config = crate::engine::run().context("Error processing python config.")?;
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
