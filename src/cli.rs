use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use crate::{changes::Changes, config::Config};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Prints resulting Rust struct after evaluating Python config
    Eval,
    /// Applies the configuration
    Apply,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        match self.command {
            Command::Eval => {
                let config = crate::engine::run().context("Error processing python config.")?;
                println!("{config:#?}");
            }
            Command::Apply => {
                let config = crate::engine::run().context("Error processing python config.")?;
                let applied_config = Config::read_saved_config();
                let changes = Changes::new(config.clone(), applied_config);
                log::debug!("Changes: {changes:#?}");
                changes.apply()?;
                config.save_config().unwrap();
            }
        }

        Ok(())
    }
}
