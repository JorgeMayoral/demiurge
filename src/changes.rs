use std::fmt::Display;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::{
    changes::{
        dotfile::DotfileChanges, package::PackageChanges, service::ServiceChanges,
        system::SystemChanges,
    },
    config::DemiurgeConfig,
};

mod dotfile;
mod package;
mod service;
mod system;

#[derive(Debug)]
pub struct Changes {
    system: SystemChanges,
    package: PackageChanges,
    dotfile: DotfileChanges,
    service: ServiceChanges,
}

impl Changes {
    #[must_use]
    pub fn new(new_config: &DemiurgeConfig, applied_config: &DemiurgeConfig) -> Self {
        Self {
            system: SystemChanges::new(&new_config.system(), &applied_config.system()),
            package: PackageChanges::new(&new_config.packages(), &applied_config.packages()),
            dotfile: DotfileChanges::new(&new_config.dotfiles(), &applied_config.dotfiles()),
            service: ServiceChanges::new(&new_config.services(), &applied_config.services()),
        }
    }

    /// # Errors
    /// Return error if applying system or package changes fails
    pub fn apply(&self, overwrite_symlinks: bool) -> Result<()> {
        log::info!("Applying system changes...");
        self.system.apply()?;
        log::info!("Applying package changes...");
        self.package.apply()?;
        log::info!("Applying dotfiles changes...");
        self.dotfile.apply(overwrite_symlinks);
        log::info!("Applying service changes...");
        self.service.apply();

        Ok(())
    }
}

impl Display for Changes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "CHANGES".green().bold().underline().to_string();

        let system_changes = &self.system;
        let package_changes = &self.package;
        let dotfile_changes = &self.dotfile;
        let service_changes = &self.service;

        write!(
            f,
            "\n{title}\n{system_changes}\n\n{package_changes}\n\n{dotfile_changes}\n\n{service_changes}\n"
        )
    }
}
