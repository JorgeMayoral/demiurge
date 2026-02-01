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
                log::info!("Changes: {changes:#?}");
                let configured_hostname = config.system_config().hostname();
                let current_hostname = duct::cmd!("hostname").read()?;

                if configured_hostname != current_hostname {
                    log::info!(
                        "Changing hostname from {current_hostname} to {configured_hostname}"
                    );
                    duct::cmd!("sudo", "hostname", configured_hostname).run()?;
                }

                let pkgs_to_install = config.packages_config().pkgs();
                log::info!("Installing packages: {}", pkgs_to_install.join(" "));
                let mut args = vec!["-S".to_owned()];
                args.extend(pkgs_to_install);
                duct::cmd("paru", args).run()?;

                config.save_config().unwrap();
            }
        }

        Ok(())
    }
}
