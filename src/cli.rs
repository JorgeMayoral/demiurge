use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

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
                let configured_hostname = config.get_system_config().get_configured_hostname();
                println!("Configured hostname: {configured_hostname}");

                let current_hostname = hostname::get()?.into_string().unwrap();
                println!("Current hostname: {current_hostname}");

                if configured_hostname != current_hostname {
                    hostname::set(configured_hostname)?;
                }
            }
        }

        Ok(())
    }
}
