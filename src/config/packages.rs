use std::{collections::HashMap, io::Write, path::Path};

use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::config::CURRENT_PACKAGES_CONFIG_FILE_NAME;

#[derive(Debug, Clone, Deserialize, Serialize, Default, JsonSchema)]
pub struct Packages(HashMap<String, Vec<String>>);

impl Packages {
    pub fn package_managers(&self) -> Vec<String> {
        self.0.keys().map(ToOwned::to_owned).collect::<_>()
    }

    pub fn validate(&self) -> Vec<String> {
        let mut errors = vec![];
        for (pm, pkgs) in &self.0 {
            if pm.is_empty() {
                errors.push("package manager name must not be empty".to_owned());
            }
            for pkg in pkgs {
                if pkg.is_empty() {
                    errors.push(format!("package name in {pm:?} must not be empty"));
                }
            }
        }
        errors
    }

    pub fn get(&self, pkg_manager: &str) -> Option<Vec<String>> {
        self.0.get(pkg_manager).map(ToOwned::to_owned)
    }

    pub fn read_applied_config(data_path: &Path) -> Option<Self> {
        let data = std::fs::read(data_path.join(CURRENT_PACKAGES_CONFIG_FILE_NAME)).ok()?;
        let applied_config_data = bitcode::deserialize(&data).ok()?;
        Some(applied_config_data)
    }

    pub fn save_applied_config(self, data_path: &Path) -> Result<()> {
        let mut current_config_file =
            std::fs::File::create(data_path.join(CURRENT_PACKAGES_CONFIG_FILE_NAME))
                .context("create packages config file")?;
        let current_config_data = bitcode::serialize(&self).context("serialize packages config")?;
        current_config_file
            .write_all(&current_config_data)
            .context("write packages config file")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Packages;

    #[test]
    fn validate_valid_packages_is_ok() {
        let pkgs: Packages =
            serde_json::from_str(r#"{"apt": ["vim", "git"]}"#).expect("literal is well-formed JSON");
        assert!(pkgs.validate().is_empty());
    }

    #[test]
    fn validate_empty_package_manager_name_is_invalid() {
        let pkgs: Packages = serde_json::from_str(r#"{"": ["vim"]}"#).expect("literal is well-formed JSON");
        let errors = pkgs.validate();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("package manager name"));
    }

    #[test]
    fn validate_empty_package_name_is_invalid() {
        let pkgs: Packages = serde_json::from_str(r#"{"apt": ["vim", ""]}"#).expect("literal is well-formed JSON");
        let errors = pkgs.validate();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("package name"));
    }

    #[test]
    fn packages_persistence_round_trip() {
        let dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let packages: Packages =
            serde_json::from_str(r#"{"paru": ["vim", "git"], "cargo": ["ripgrep"]}"#).expect("literal is well-formed JSON");
        packages.save_applied_config(dir.path()).expect("temp dir is writable");
        let loaded = Packages::read_applied_config(dir.path()).expect("config was just saved");
        assert_eq!(
            loaded.get("paru").expect("\"paru\" key was just inserted"),
            vec!["vim".to_owned(), "git".to_owned()]
        );
        assert_eq!(loaded.get("cargo").expect("\"cargo\" key was just inserted"), vec!["ripgrep".to_owned()]);
    }

    #[test]
    fn read_applied_config_returns_none_when_missing() {
        let dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        assert!(Packages::read_applied_config(dir.path()).is_none());
    }
}
