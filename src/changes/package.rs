use std::{collections::HashMap, fmt::Display};

use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::config::Packages;

#[derive(Debug, Clone)]
pub struct PackageChanges(HashMap<String, Changes>);

#[derive(Debug, Clone)]
struct Changes {
    pub install: Vec<String>,
    pub remove: Vec<String>,
}

impl PackageChanges {
    pub fn new(new_pkgs_config: &Packages, applied_pkgs_config: &Packages) -> Self {
        let mut changes = HashMap::new();
        for package_manager in new_pkgs_config.package_managers() {
            let pkgs_to_install = new_pkgs_config
                .get(&package_manager)
                .expect("package_manager is obtained from new_pkgs_config's own keys")
                .iter()
                .filter(|pkg| {
                    !applied_pkgs_config
                        .get(&package_manager)
                        .unwrap_or_default()
                        .contains(pkg)
                })
                .map(ToOwned::to_owned)
                .collect::<_>();

            let pkgs_to_remove = applied_pkgs_config
                .get(&package_manager)
                .unwrap_or_default()
                .iter()
                .filter(|pkg| {
                    !new_pkgs_config
                        .get(&package_manager)
                        .unwrap_or_default()
                        .contains(pkg)
                })
                .map(ToOwned::to_owned)
                .collect::<_>();

            changes.insert(
                package_manager,
                Changes {
                    install: pkgs_to_install,
                    remove: pkgs_to_remove,
                },
            );
        }
        Self(changes)
    }

    pub fn is_empty(&self) -> bool {
        self.0.values().all(|c| c.install.is_empty() && c.remove.is_empty())
    }

    pub fn apply(&self) -> Result<()> {
        let supported_package_managers = PackageManagers::default();
        for (pkg_manager, changes) in self.0.clone() {
            log::info!("Managing {pkg_manager} packages");

            if changes.remove.is_empty() {
                log::info!("No packages to remove.");
            } else {
                let remove_args = supported_package_managers.remove_args(&pkg_manager);
                match remove_args {
                    None => {
                        log::error!("Removing {pkg_manager} packages not supported. Skipping.");
                    }
                    Some(args) => {
                        let pkgs_to_remove = changes.remove;
                        let mut args = args;
                        log::info!("Removing packages: {}", pkgs_to_remove.join(" "));
                        args.extend(pkgs_to_remove);
                        duct::cmd(&pkg_manager, args)
                            .run()
                            .with_context(|| format!("remove packages via {pkg_manager}"))?;
                    }
                }
            }

            if changes.install.is_empty() {
                log::info!("No packages to install.");
            } else {
                let install_args = supported_package_managers.install_args(&pkg_manager);
                match install_args {
                    None => {
                        log::error!("Installing {pkg_manager} packages not supported. Skipping.");
                    }
                    Some(args) => {
                        let pkgs_to_install = changes.install;
                        let mut args = args;
                        log::info!("Installing packages: {}", pkgs_to_install.join(" "));
                        args.extend(pkgs_to_install);
                        duct::cmd(&pkg_manager, args)
                            .run()
                            .with_context(|| format!("install packages via {pkg_manager}"))?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl Display for PackageChanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "Packages".blue().bold().to_string();

        let package_managers_text = self
            .0
            .iter()
            .map(|(pkg_manager, changes)| {
                let pkg_manager_title = pkg_manager.blue().underline().to_string();
                let mut packages_added = vec![];
                if changes.install.is_empty() {
                    packages_added.push("No packages to install".yellow().to_string());
                } else {
                    let add_symbol = "+".green().bold().to_string();
                    changes.install.iter().for_each(|pkg| {
                        let text = format!("[{add_symbol}] {pkg}");
                        packages_added.push(text);
                    });
                }
                let packages_added_text = packages_added.join("\n");

                let mut packages_removed = vec![];
                if changes.remove.is_empty() {
                    packages_removed.push("No packages to remove".yellow().to_string());
                } else {
                    let remove_symbol = "-".red().bold().to_string();
                    changes.remove.iter().for_each(|pkg| {
                        let text = format!("[{remove_symbol}] {pkg}");
                        packages_removed.push(text);
                    });
                }
                let packages_removed_text = packages_removed.join("\n");
                format!("{pkg_manager_title}\n{packages_added_text}\n\n{packages_removed_text}")
            })
            .collect::<Vec<String>>()
            .join("\n\n");

        let text = format!("{title}\n{package_managers_text}");
        write!(f, "{text}")
    }
}

#[derive(Debug, Clone)]
struct PackageManagers(HashMap<String, (Vec<String>, Vec<String>)>);

impl PackageManagers {
    fn install_args(&self, pkg_manager: &str) -> Option<Vec<String>> {
        self.0.get(pkg_manager).map(|(args, _)| args.to_owned())
    }

    fn remove_args(&self, pkg_manager: &str) -> Option<Vec<String>> {
        self.0.get(pkg_manager).map(|(_, args)| args.to_owned())
    }
}

impl Default for PackageManagers {
    fn default() -> Self {
        let mut list = HashMap::new();
        list.insert(
            "paru".to_owned(),
            (vec!["-S".to_owned()], vec!["-Rs".to_owned()]),
        );
        list.insert(
            "cargo".to_owned(),
            (
                vec!["install".to_owned(), "--locked".to_owned()],
                vec!["uninstall".to_owned()],
            ),
        );
        Self(list)
    }
}

#[cfg(test)]
mod tests {
    use super::PackageChanges;
    use crate::config::Packages;

    fn pkgs(json: &str) -> Packages {
        serde_json::from_str(json).expect("literal is well-formed JSON")
    }

    #[test]
    fn apply_does_nothing_when_no_changes() {
        let pkgs_config = pkgs(r#"{"paru": ["vim", "git"]}"#);
        let changes = PackageChanges::new(&pkgs_config, &pkgs_config);
        assert!(changes.apply().is_ok());
    }

    #[test]
    fn apply_skips_unsupported_package_manager_gracefully() {
        let new = pkgs(r#"{"unknown-pm": ["some-package"]}"#);
        let applied = pkgs(r#"{}"#);
        let changes = PackageChanges::new(&new, &applied);
        assert!(changes.apply().is_ok());
    }

    #[test]
    fn new_package_goes_to_install_set() {
        let new = pkgs(r#"{"paru": ["vim", "git"]}"#);
        let applied = pkgs(r#"{"paru": ["vim"]}"#);
        let changes = PackageChanges::new(&new, &applied);
        let paru = changes.0.get("paru").expect("\"paru\" key was obtained from this map's iterator");
        assert!(paru.install.contains(&"git".to_owned()));
        assert!(paru.remove.is_empty());
    }

    #[test]
    fn removed_package_goes_to_remove_set() {
        let new = pkgs(r#"{"paru": ["vim"]}"#);
        let applied = pkgs(r#"{"paru": ["vim", "git"]}"#);
        let changes = PackageChanges::new(&new, &applied);
        let paru = changes.0.get("paru").expect("\"paru\" key was obtained from this map's iterator");
        assert!(paru.remove.contains(&"git".to_owned()));
        assert!(paru.install.is_empty());
    }

    #[test]
    fn unchanged_packages_produce_empty_sets() {
        let pkgs_config = pkgs(r#"{"paru": ["vim", "git"]}"#);
        let changes = PackageChanges::new(&pkgs_config, &pkgs_config);
        let paru = changes.0.get("paru").expect("\"paru\" key was obtained from this map's iterator");
        assert!(paru.install.is_empty());
        assert!(paru.remove.is_empty());
    }

    #[test]
    fn new_package_manager_installs_all_its_packages() {
        let new = pkgs(r#"{"cargo": ["ripgrep", "fd"]}"#);
        let applied = pkgs(r#"{}"#);
        let changes = PackageChanges::new(&new, &applied);
        let cargo = changes.0.get("cargo").expect("\"cargo\" key was obtained from this map's iterator");
        assert!(cargo.install.contains(&"ripgrep".to_owned()));
        assert!(cargo.install.contains(&"fd".to_owned()));
        assert!(cargo.remove.is_empty());
    }
}
