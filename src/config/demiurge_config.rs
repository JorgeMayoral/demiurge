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
    pub fn system(&self) -> &System {
        &self.system
    }

    pub fn packages(&self) -> &Packages {
        &self.packages
    }

    pub fn dotfiles(&self) -> &Dotfiles {
        &self.dotfiles
    }

    pub fn services(&self) -> &Services {
        &self.services
    }

    pub fn users(&self) -> &Users {
        &self.users
    }

    pub fn validate(&self) -> Result<()> {
        let errors: Vec<String> = [
            self.system.validate(),
            self.packages.validate(),
            self.dotfiles.validate(),
            self.services.validate(),
            self.users.validate(),
        ]
        .into_iter()
        .flatten()
        .collect();

        if errors.is_empty() {
            Ok(())
        } else {
            anyhow::bail!("config validation failed:\n{}", errors.join("\n"))
        }
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
        let config: DemiurgeConfig =
            serde_json::from_str(FULL_CONFIG_JSON).expect("FULL_CONFIG_JSON is valid JSON");
        assert_eq!(config.system().hostname(), "testhost");
        assert_eq!(
            config
                .packages()
                .get("paru")
                .expect("\"paru\" key exists in the config we just parsed"),
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
        let original: serde_json::Value =
            serde_json::from_str(FULL_CONFIG_JSON).expect("FULL_CONFIG_JSON is valid JSON");
        let config: DemiurgeConfig = serde_json::from_value(original.clone())
            .expect("value was just parsed from valid JSON");
        let serialized: serde_json::Value =
            serde_json::to_value(&config).expect("DemiurgeConfig is always serializable");
        assert_eq!(original, serialized);
    }

    #[test]
    fn empty_config_deserializes_to_defaults() {
        let json = r#"{"system": {"hostname": ""}, "packages": {}, "dotfiles": [], "services": [], "users": []}"#;
        let config: DemiurgeConfig =
            serde_json::from_str(json).expect("literal is well-formed JSON");
        assert!(config.packages().package_managers().is_empty());
        assert!(config.dotfiles().dotfiles().is_empty());
        assert!(config.services().services().is_empty());
        assert!(config.users().users().is_empty());
    }

    #[test]
    fn validate_valid_config_is_ok() {
        let config: DemiurgeConfig =
            serde_json::from_str(FULL_CONFIG_JSON).expect("FULL_CONFIG_JSON is valid JSON");
        assert!(config.validate().is_ok());
    }

    #[test]
    fn validate_aggregates_errors_from_all_sub_configs() {
        let json = r#"{
            "system": {"hostname": "bad/host"},
            "packages": {"apt": [""]},
            "dotfiles": [],
            "services": [""],
            "users": []
        }"#;
        let config: DemiurgeConfig =
            serde_json::from_str(json).expect("literal is well-formed JSON");
        let err = config.validate().unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("hostname"),
            "expected hostname error in: {msg}"
        );
        assert!(
            msg.contains("package name"),
            "expected package error in: {msg}"
        );
        assert!(
            msg.contains("service name"),
            "expected service error in: {msg}"
        );
    }
}
