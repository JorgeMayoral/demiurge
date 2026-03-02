use std::fmt::Display;

use anyhow::Context;
use owo_colors::OwoColorize;

use crate::{
    changes::{
        dotfile::DotfileChanges, outcome::ApplyOutcome, package::PackageChanges,
        service::ServiceChanges, system::SystemChanges, user::UsersChanges,
    },
    config::DemiurgeConfig,
};

mod dotfile;
mod outcome;
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
            system: SystemChanges::new(new_config.system(), applied_config.system()),
            package: PackageChanges::new(new_config.packages(), applied_config.packages()),
            dotfile: DotfileChanges::new(new_config.dotfiles(), applied_config.dotfiles()),
            service: ServiceChanges::new(new_config.services(), applied_config.services()),
            user: UsersChanges::new(new_config.users(), applied_config.users()),
        }
    }

    pub fn apply(&self, overwrite_symlinks: bool) -> ApplyOutcome {
        log::info!("Applying system changes...");
        let system = self
            .system
            .apply()
            .context("apply system changes")
            .map_err(|e| {
                log::error!("{e:?}");
                e
            })
            .err();

        log::info!("Applying package changes...");
        let packages = self
            .package
            .apply()
            .context("apply package changes")
            .map_err(|e| {
                log::error!("{e:?}");
                e
            })
            .err();

        log::info!("Applying dotfile changes...");
        let dotfiles = self
            .dotfile
            .apply(overwrite_symlinks)
            .context("apply dotfile changes")
            .map_err(|e| {
                log::error!("{e:?}");
                e
            })
            .err();

        log::info!("Applying service changes...");
        let services = self
            .service
            .apply()
            .context("apply service changes")
            .map_err(|e| {
                log::error!("{e:?}");
                e
            })
            .err();

        log::info!("Applying user changes...");
        let users = self
            .user
            .apply()
            .context("apply user changes")
            .map_err(|e| {
                log::error!("{e:?}");
                e
            })
            .err();

        ApplyOutcome {
            system,
            packages,
            dotfiles,
            services,
            users,
        }
    }
}

impl Display for Changes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "CHANGES".green().bold().underline().to_string();

        let sections: Vec<String> = [
            (!self.system.is_empty()).then(|| self.system.to_string()),
            (!self.package.is_empty()).then(|| self.package.to_string()),
            (!self.dotfile.is_empty()).then(|| self.dotfile.to_string()),
            (!self.service.is_empty()).then(|| self.service.to_string()),
            (!self.user.is_empty()).then(|| self.user.to_string()),
        ]
        .into_iter()
        .flatten()
        .collect();

        let body = if sections.is_empty() {
            "No changes.".yellow().to_string()
        } else {
            sections.join("\n\n")
        };

        write!(f, "\n{title}\n{body}\n")
    }
}
