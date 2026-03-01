use std::path::PathBuf;

use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::config::{Dotfiles, Packages, Services, System, Users};

#[derive(Debug, Clone, Deserialize, Serialize, Default, JsonSchema)]
pub struct DemiurgeConfig {
    system: System,
    packages: Packages,
    dotfiles: Dotfiles,
    services: Services,
    users: Users,
}

impl DemiurgeConfig {
    pub fn system(&self) -> System {
        self.system.clone()
    }

    pub fn packages(&self) -> Packages {
        self.packages.clone()
    }

    pub fn dotfiles(&self) -> Dotfiles {
        self.dotfiles.clone()
    }

    pub fn services(&self) -> Services {
        self.services.clone()
    }

    pub fn users(&self) -> Users {
        self.users.clone()
    }

    pub fn read_applied_config() -> Option<Self> {
        let data_path = Self::get_data_dir().ok()?;
        let applied_system_config = System::read_applied_config(&data_path).unwrap_or_default();
        let applied_packages_config = Packages::read_applied_config(&data_path).unwrap_or_default();
        let applied_dotfiles_config = Dotfiles::read_applied_config(&data_path).unwrap_or_default();
        let applied_services_config = Services::read_applied_config(&data_path).unwrap_or_default();
        let applied_users_config = Users::read_applied_config(&data_path).unwrap_or_default();
        Some(Self {
            system: applied_system_config,
            packages: applied_packages_config,
            dotfiles: applied_dotfiles_config,
            services: applied_services_config,
            users: applied_users_config,
        })
    }

    pub fn save_applied_config(self) -> Result<()> {
        let data_path = Self::get_data_dir().context("resolve data directory")?;
        std::fs::create_dir_all(&data_path).context("create data directory")?;
        log::info!("Saving applied configuration in {}", &data_path.display());
        self.system
            .save_applied_config(&data_path)
            .context("save system applied config")?;
        self.packages
            .save_applied_config(&data_path)
            .context("save packages applied config")?;
        self.dotfiles
            .save_applied_config(&data_path)
            .context("save dotfiles applied config")?;
        self.services
            .save_applied_config(&data_path)
            .context("save services applied config")?;
        self.users
            .save_applied_config(&data_path)
            .context("save users applied config")?;
        Ok(())
    }

    fn get_data_dir() -> Result<PathBuf> {
        let project_dir = directories::ProjectDirs::from("dev", "yorch", "demiurge")
            .ok_or(anyhow::anyhow!("Could not get project directory."))?;
        Ok(project_dir.data_dir().to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use super::DemiurgeConfig;

    const FULL_CONFIG_JSON: &str = r#"{
        "system": {"hostname": "testhost"},
        "packages": {"paru": ["vim", "git"], "cargo": ["ripgrep"]},
        "dotfiles": [{"source": "/dotfiles/nvim", "target": "/home/user/.config/nvim"}],
        "services": ["nginx", "docker"],
        "users": [{"name": "alice", "groups": ["wheel", "docker"]}]
    }"#;

    #[test]
    fn full_config_deserializes_correctly() {
        let config: DemiurgeConfig = serde_json::from_str(FULL_CONFIG_JSON).unwrap();
        assert_eq!(config.system().hostname(), "testhost");
        assert_eq!(
            config.packages().get("paru").unwrap(),
            vec!["vim".to_owned(), "git".to_owned()]
        );
        assert_eq!(config.dotfiles().dotfiles().len(), 1);
        let service_names: Vec<String> = config
            .services()
            .services()
            .iter()
            .map(|s| s.service())
            .collect();
        assert!(service_names.contains(&"nginx".to_owned()));
        assert_eq!(config.users().users().len(), 1);
        assert_eq!(config.users().users()[0].name(), "alice");
    }

    #[test]
    fn config_round_trip_is_stable() {
        let original: serde_json::Value = serde_json::from_str(FULL_CONFIG_JSON).unwrap();
        let config: DemiurgeConfig = serde_json::from_value(original.clone()).unwrap();
        let serialized: serde_json::Value = serde_json::to_value(&config).unwrap();
        assert_eq!(original, serialized);
    }

    #[test]
    fn empty_config_deserializes_to_defaults() {
        let json = r#"{"system": {"hostname": ""}, "packages": {}, "dotfiles": [], "services": [], "users": []}"#;
        let config: DemiurgeConfig = serde_json::from_str(json).unwrap();
        assert!(config.packages().package_managers().is_empty());
        assert!(config.dotfiles().dotfiles().is_empty());
        assert!(config.services().services().is_empty());
        assert!(config.users().users().is_empty());
    }
}
