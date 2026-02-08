use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Args;

use crate::{
    changes::Changes,
    config::{Demiurge, DemiurgeConfig},
};

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

impl ApplyArgs {
    pub fn run(self) -> Result<()> {
        let configs = Demiurge::from_file(self.file)?;
        let config = configs
            .get(&self.name)
            .context(format!("Configuration \"{}\" not found", self.name))?;
        let applied_config = DemiurgeConfig::read_applied_config();
        let changes = Changes::new(&config, &applied_config);
        if self.dry_run {
            println!("{changes}");
        } else {
            changes.apply(self.overwrite_symlink)?;
            config.save_applied_config()?;
        }

        Ok(())
    }
}
