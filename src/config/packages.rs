use std::{collections::HashMap, io::Write, path::Path};

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const CURRENT_PACKAGES_CONFIG_FILE_NAME: &str = "current_packages_config";

#[derive(Debug, Clone, Deserialize, Serialize, Default, JsonSchema)]
pub struct Packages(HashMap<String, Vec<String>>);

impl Packages {
    pub fn package_managers(&self) -> Vec<String> {
        self.0.keys().map(ToOwned::to_owned).collect::<_>()
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
            std::fs::File::create(data_path.join(CURRENT_PACKAGES_CONFIG_FILE_NAME))?;
        let current_config_data = bitcode::serialize(&self)?;
        current_config_file.write_all(&current_config_data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Packages;

    #[test]
    fn packages_persistence_round_trip() {
        let dir = tempfile::TempDir::new().unwrap();
        let packages: Packages =
            serde_json::from_str(r#"{"paru": ["vim", "git"], "cargo": ["ripgrep"]}"#).unwrap();
        packages.save_applied_config(dir.path()).unwrap();
        let loaded = Packages::read_applied_config(dir.path()).unwrap();
        assert_eq!(
            loaded.get("paru").unwrap(),
            vec!["vim".to_owned(), "git".to_owned()]
        );
        assert_eq!(loaded.get("cargo").unwrap(), vec!["ripgrep".to_owned()]);
    }

    #[test]
    fn read_applied_config_returns_none_when_missing() {
        let dir = tempfile::TempDir::new().unwrap();
        assert!(Packages::read_applied_config(dir.path()).is_none());
    }
}
