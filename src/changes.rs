use crate::config::{Config, Packages, System};

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
}

#[derive(Debug)]
pub struct SystemChanges {
    hostname: Option<String>,
}

impl SystemChanges {
    pub fn new(new_system_config: System, applied_system_config: Option<System>) -> Self {
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
}

#[derive(Debug)]
pub struct PackageChanges {
    install: Vec<String>,
    remove: Vec<String>,
}

impl PackageChanges {
    pub fn new(new_pkgs_config: Packages, applied_pkgs_config: Option<Packages>) -> Self {
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
}
