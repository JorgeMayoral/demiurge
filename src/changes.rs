use std::fmt::Display;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::config::{Config, PackagesConfig, SystemConfig};

#[derive(Debug)]
pub struct Changes {
    system_changes: SystemChanges,
    package_changes: PackageChanges,
}

impl Changes {
    pub fn new(new_config: Config, applied_config: Option<Config>) -> Self {
        match applied_config {
            Some(applied_config) => Self {
                system_changes: SystemChanges::new(
                    new_config.system_config(),
                    Some(applied_config.system_config()),
                ),
                package_changes: PackageChanges::new(
                    new_config.packages_config(),
                    Some(applied_config.packages_config()),
                ),
            },
            None => Self {
                system_changes: SystemChanges::new(new_config.system_config(), None),
                package_changes: PackageChanges::new(new_config.packages_config(), None),
            },
        }
    }

    pub fn apply(&self) -> Result<()> {
        log::info!("Applying system changes...");
        self.system_changes.apply()?;
        log::info!("Applying package changes...");
        self.package_changes.apply()?;

        Ok(())
    }
}

impl Display for Changes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "CHANGES".green().bold().underline().to_string();

        let system_title = "System".blue().bold().to_string();
        let hostname_change = match self.system_changes.hostname.clone() {
            Some(hostname) => hostname.green().to_string(),
            None => "Unchanged".yellow().to_string(),
        };

        let packages_title = "Packages".blue().bold().to_string();

        let mut packages_added = vec![];
        if self.package_changes.install.is_empty() {
            packages_added.push("No packages to install".yellow().to_string());
        } else {
            let add_symbol = "+".green().bold().to_string();
            self.package_changes.install.iter().for_each(|pkg| {
                let text = format!("[{add_symbol}] {pkg}");
                packages_added.push(text);
            });
        };
        let packages_added_text = packages_added.join("\n");

        let mut packages_removed = vec![];
        if self.package_changes.remove.is_empty() {
            packages_removed.push("No packages to remove".yellow().to_string());
        } else {
            let remove_symbol = "-".red().bold().to_string();
            self.package_changes.remove.iter().for_each(|pkg| {
                let text = format!("[{remove_symbol}] {pkg}");
                packages_removed.push(text);
            });
        };
        let packages_removed_text = packages_removed.join("\n");

        write!(
            f,
            "\n{title}\n{system_title}\nHostname: {hostname_change}\n\n{packages_title}\nInstall\n{packages_added_text}\n\nRemove\n{packages_removed_text}\n"
        )
    }
}

#[derive(Debug)]
pub struct SystemChanges {
    hostname: Option<String>,
}

impl SystemChanges {
    pub fn new(
        new_system_config: SystemConfig,
        applied_system_config: Option<SystemConfig>,
    ) -> Self {
        match applied_system_config {
            Some(applied_system_config) => {
                let hostname = if new_system_config.hostname() != applied_system_config.hostname() {
                    Some(new_system_config.hostname())
                } else {
                    None
                };
                Self { hostname }
            }
            None => Self {
                hostname: Some(new_system_config.hostname()),
            },
        }
    }

    pub fn apply(&self) -> Result<()> {
        if let Some(hostname) = self.hostname.clone() {
            let configured_hostname = hostname;
            let current_hostname = duct::cmd!("hostname").read()?;

            if configured_hostname != current_hostname {
                log::info!("Changing hostname from {current_hostname} to {configured_hostname}");
                duct::cmd!("sudo", "hostname", configured_hostname).run()?;
            }
        } else {
            log::info!("Hostname already configured.")
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct PackageChanges {
    install: Vec<String>,
    remove: Vec<String>,
}

impl PackageChanges {
    pub fn new(
        new_pkgs_config: PackagesConfig,
        applied_pkgs_config: Option<PackagesConfig>,
    ) -> Self {
        let new_pkgs = new_pkgs_config.pkgs();
        let applied_pkgs = applied_pkgs_config
            .map(|config| config.pkgs())
            .unwrap_or_default();

        let pkgs_to_install: Vec<String> = new_pkgs
            .iter()
            .filter(|pkg| !applied_pkgs.contains(pkg))
            .map(|pkg| pkg.to_owned())
            .collect();
        let pkgs_to_remove: Vec<String> = applied_pkgs
            .iter()
            .filter(|pkg| !new_pkgs.contains(pkg))
            .map(|pkg| pkg.to_owned())
            .collect();

        Self {
            install: pkgs_to_install,
            remove: pkgs_to_remove,
        }
    }

    pub fn apply(&self) -> Result<()> {
        if !self.install.is_empty() {
            let pkgs_to_install = self.install.clone();
            log::info!("Installing packages: {}", pkgs_to_install.join(" "));
            let mut args = vec!["-S".to_owned()];
            args.extend(pkgs_to_install);
            duct::cmd("paru", args).run()?;
        } else {
            log::info!("No packages to install.")
        }

        if !self.remove.is_empty() {
            let pkgs_to_remove = self.remove.clone();
            log::info!("Removing packages: {}", pkgs_to_remove.join(" "));
            let mut args = vec!["-Rs".to_owned()];
            args.extend(pkgs_to_remove);
            duct::cmd("paru", args).run()?;
        } else {
            log::info!("No packages to remove.")
        }

        Ok(())
    }
}
