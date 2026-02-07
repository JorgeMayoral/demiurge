use std::fmt::Display;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::config::{DemiurgeConfig, Packages, System};

#[derive(Debug)]
pub struct Changes {
    system_changes: SystemChanges,
    package_changes: PackageChanges,
}

impl Changes {
    #[must_use]
    pub fn new(new_config: &DemiurgeConfig, applied_config: Option<DemiurgeConfig>) -> Self {
        match applied_config {
            Some(applied_config) => Self {
                system_changes: SystemChanges::new(
                    &new_config.system(),
                    Some(applied_config.system()),
                ),
                package_changes: PackageChanges::new(
                    &new_config.packages(),
                    Some(applied_config.packages()),
                ),
            },
            None => Self {
                system_changes: SystemChanges::new(&new_config.system(), None),
                package_changes: PackageChanges::new(&new_config.packages(), None),
            },
        }
    }

    /// # Errors
    /// Return error if applying system or package changes fails
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
        if self.package_changes.install.paru().is_empty() {
            packages_added.push("No packages to install".yellow().to_string());
        } else {
            let add_symbol = "+".green().bold().to_string();
            self.package_changes.install.paru().iter().for_each(|pkg| {
                let text = format!("[{add_symbol}] {pkg}");
                packages_added.push(text);
            });
        }
        let packages_added_text = packages_added.join("\n");

        let mut packages_removed = vec![];
        if self.package_changes.remove.paru().is_empty() {
            packages_removed.push("No packages to remove".yellow().to_string());
        } else {
            let remove_symbol = "-".red().bold().to_string();
            self.package_changes.remove.paru().iter().for_each(|pkg| {
                let text = format!("[{remove_symbol}] {pkg}");
                packages_removed.push(text);
            });
        }
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
    #[must_use]
    pub fn new(new_system_config: &System, applied_system_config: Option<System>) -> Self {
        match applied_system_config {
            Some(applied_system_config) => {
                let hostname = if new_system_config.hostname() == applied_system_config.hostname() {
                    None
                } else {
                    Some(new_system_config.hostname())
                };
                Self { hostname }
            }
            None => Self {
                hostname: Some(new_system_config.hostname()),
            },
        }
    }

    /// # Errors
    /// Returns error if `hostname` or `sudo hostname {name}` fail.
    pub fn apply(&self) -> Result<()> {
        if let Some(hostname) = self.hostname.clone() {
            let configured_hostname = hostname;
            let current_hostname = duct::cmd!("hostname").read()?;

            if configured_hostname != current_hostname {
                log::info!("Changing hostname from {current_hostname} to {configured_hostname}");
                duct::cmd!("sudo", "hostname", configured_hostname).run()?;
            }
        } else {
            log::info!("Hostname already configured.");
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct PackageChanges {
    install: Packages,
    remove: Packages,
}

impl PackageChanges {
    #[must_use]
    pub fn new(new_pkgs_config: &Packages, applied_pkgs_config: Option<Packages>) -> Self {
        let new_paru_pkgs = new_pkgs_config.paru();
        let applied_paru_pkgs = applied_pkgs_config
            .map(|config| config.paru())
            .unwrap_or_default();

        let pkgs_to_install: Vec<String> = new_paru_pkgs
            .iter()
            .filter(|pkg| !applied_paru_pkgs.contains(pkg))
            .map(ToOwned::to_owned)
            .collect();
        let pkgs_to_remove: Vec<String> = applied_paru_pkgs
            .iter()
            .filter(|pkg| !new_paru_pkgs.contains(pkg))
            .map(ToOwned::to_owned)
            .collect();

        Self {
            install: Packages::new(pkgs_to_install),
            remove: Packages::new(pkgs_to_remove),
        }
    }

    /// # Errors
    /// Return error if `paru` command fails
    pub fn apply(&self) -> Result<()> {
        if self.install.paru().is_empty() {
            log::info!("No packages to install.");
        } else {
            let pkgs_to_install = self.install.clone();
            log::info!("Installing packages: {}", pkgs_to_install.paru().join(" "));
            let mut args = vec!["-S".to_owned()];
            args.extend(pkgs_to_install.paru());
            duct::cmd("paru", args).run()?;
        }

        if self.remove.paru().is_empty() {
            log::info!("No packages to remove.");
        } else {
            let pkgs_to_remove = self.remove.clone();
            log::info!("Removing packages: {}", pkgs_to_remove.paru().join(" "));
            let mut args = vec!["-Rs".to_owned()];
            args.extend(pkgs_to_remove.paru());
            duct::cmd("paru", args).run()?;
        }

        Ok(())
    }
}
