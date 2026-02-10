use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::cli::{apply::ApplyArgs, eval::EvalArgs, init::InitArgs};

mod apply;
mod eval;
mod init;

#[derive(Debug, Parser, Clone)]
#[command(version, about, author)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    /// Prints resulting Rust struct after evaluating Typescript config
    Eval(EvalArgs),
    /// Applies the configuration
    Apply(ApplyArgs),
    /// Creates the initial configuration files
    Init(InitArgs),
}

impl Cli {
    /// # Errors
    /// TODO
    pub fn run(&self) -> Result<()> {
        match self.command.clone() {
            Command::Eval(args) => {
                args.run()?;
            }
            Command::Apply(args) => {
                args.run()?;
            }
            Command::Init(args) => {
                args.run()?;
            }
        }

        Ok(())
    }
}
