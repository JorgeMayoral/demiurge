use std::fmt::Display;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::config::Packages;

#[derive(Debug)]
pub struct PackageChanges {
    pub install: Packages,
    pub remove: Packages,
}

impl PackageChanges {
    pub fn new(new_pkgs_config: &Packages, applied_pkgs_config: &Packages) -> Self {
        let new_paru_pkgs = new_pkgs_config.paru();
        let applied_paru_pkgs = applied_pkgs_config.paru();

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

    pub fn apply(&self) -> Result<()> {
        if self.remove.paru().is_empty() {
            log::info!("No packages to remove.");
        } else {
            let pkgs_to_remove = self.remove.clone();
            log::info!("Removing packages: {}", pkgs_to_remove.paru().join(" "));
            let mut args = vec!["-Rs".to_owned()];
            args.extend(pkgs_to_remove.paru());
            duct::cmd("paru", args).run()?;
        }

        if self.install.paru().is_empty() {
            log::info!("No packages to install.");
        } else {
            let pkgs_to_install = self.install.clone();
            log::info!("Installing packages: {}", pkgs_to_install.paru().join(" "));
            let mut args = vec!["-S".to_owned()];
            args.extend(pkgs_to_install.paru());
            duct::cmd("paru", args).run()?;
        }

        Ok(())
    }
}

impl Display for PackageChanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "Packages".blue().bold().to_string();

        let mut packages_added = vec![];
        if self.install.paru().is_empty() {
            packages_added.push("No packages to install".yellow().to_string());
        } else {
            let add_symbol = "+".green().bold().to_string();
            self.install.paru().iter().for_each(|pkg| {
                let text = format!("[{add_symbol}] {pkg}");
                packages_added.push(text);
            });
        }
        let packages_added_text = packages_added.join("\n");

        let mut packages_removed = vec![];
        if self.remove.paru().is_empty() {
            packages_removed.push("No packages to remove".yellow().to_string());
        } else {
            let remove_symbol = "-".red().bold().to_string();
            self.remove.paru().iter().for_each(|pkg| {
                let text = format!("[{remove_symbol}] {pkg}");
                packages_removed.push(text);
            });
        }
        let packages_removed_text = packages_removed.join("\n");

        let text = format!("{title}\n{packages_added_text}\n\n{packages_removed_text}");
        write!(f, "{text}")
    }
}
