use std::fmt::Display;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::{
    changes::{
        dotfile::DotfileChanges, package::PackageChanges, service::ServiceChanges,
        system::SystemChanges, user::UsersChanges,
    },
    config::DemiurgeConfig,
};

mod dotfile;
mod package;
mod service;
mod system;
mod user;

#[derive(Debug)]
pub struct Changes {
    system: SystemChanges,
    package: PackageChanges,
    dotfile: DotfileChanges,
    service: ServiceChanges,
    user: UsersChanges,
}

impl Changes {
    pub fn new(new_config: &DemiurgeConfig, applied_config: &DemiurgeConfig) -> Self {
        Self {
            system: SystemChanges::new(&new_config.system(), &applied_config.system()),
            package: PackageChanges::new(&new_config.packages(), &applied_config.packages()),
            dotfile: DotfileChanges::new(&new_config.dotfiles(), &applied_config.dotfiles()),
            service: ServiceChanges::new(&new_config.services(), &applied_config.services()),
            user: UsersChanges::new(&new_config.users(), &applied_config.users()),
        }
    }

    pub fn apply(&self, overwrite_symlinks: bool) -> Result<()> {
        log::info!("Applying system changes...");
        self.system.apply()?;
        log::info!("Applying package changes...");
        self.package.apply()?;
        log::info!("Applying dotfiles changes...");
        self.dotfile.apply(overwrite_symlinks)?;
        log::info!("Applying service changes...");
        self.service.apply()?;
        log::info!("Applying users changes...");
        self.user.apply()?;

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
        let user_changes = &self.user;

        write!(
            f,
            "\n{title}\n{system_changes}\n\n{package_changes}\n\n{dotfile_changes}\n\n{service_changes}\n\n{user_changes}\n"
        )
    }
}
