use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};

use crate::{
    changes::Changes,
    config::{Demiurge, DemiurgeConfig},
};

mod init;

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
    /// Creates the initial configuration files
    Init,
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
    /// Name of the configuration to apply
    #[arg(short, long)]
    name: String,
    /// Allows overwriting already existing dotfile symlinks
    #[arg(long)]
    overwrite_symlink: bool,
}

impl Cli {
    /// # Errors
    /// TODO
    pub fn run(&self) -> Result<()> {
        match self.command.clone() {
            Command::Eval(args) => {
                let config = Demiurge::from_file(args.file);
                println!("{config:#?}");
            }
            Command::Apply(args) => {
                let configs = Demiurge::from_file(args.clone().file)?;
                let config = configs.get(&args.name).context(format!(
                    "Configuration \"{}\" not found",
                    args.clone().name.clone()
                ))?;
                let applied_config = DemiurgeConfig::read_applied_config();
                let changes = Changes::new(&config, &applied_config);
                if args.dry_run {
                    println!("{changes}");
                } else {
                    changes.apply(args.overwrite_symlink)?;
                    config.save_applied_config()?;
                }
            }
            Command::Init => {
                let cwd = std::env::current_dir()?;
                init::initialize_config(&cwd)?;
            }
        }

        Ok(())
    }
}
