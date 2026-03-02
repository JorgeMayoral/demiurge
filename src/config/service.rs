use std::{io::Write, path::Path};

use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::config::CURRENT_SERVICES_CONFIG_FILE_NAME;

#[derive(Debug, Clone, Deserialize, Serialize, Default, JsonSchema)]
pub struct Services(Vec<Service>);

impl Services {
    pub fn new(services: Vec<Service>) -> Self {
        Self(services)
    }

    pub fn services(&self) -> Vec<Service> {
        self.0.clone()
    }

    pub fn validate(&self) -> Vec<String> {
        self.0
            .iter()
            .filter(|s| s.service().is_empty())
            .map(|_| "service name must not be empty".to_owned())
            .collect()
    }

    pub fn read_applied_config(data_path: &Path) -> Option<Self> {
        let data = std::fs::read(data_path.join(CURRENT_SERVICES_CONFIG_FILE_NAME)).ok()?;
        let applied_config_data = bitcode::deserialize(&data).ok()?;
        Some(applied_config_data)
    }

    pub fn save_applied_config(self, data_path: &Path) -> Result<()> {
        let mut current_config_file =
            std::fs::File::create(data_path.join(CURRENT_SERVICES_CONFIG_FILE_NAME))
                .context("create services config file")?;
        let current_config_data = bitcode::serialize(&self).context("serialize services config")?;
        current_config_file
            .write_all(&current_config_data)
            .context("write services config file")?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, JsonSchema)]
pub struct Service(String);

impl Service {
    pub fn service(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::{Service, Services};

    #[test]
    fn validate_valid_services_is_ok() {
        let services = Services::new(vec![Service("nginx".to_owned())]);
        assert!(services.validate().is_empty());
    }

    #[test]
    fn validate_empty_service_name_is_invalid() {
        let services = Services::new(vec![Service("".to_owned())]);
        let errors = services.validate();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("service name"));
    }

    #[test]
    fn services_persistence_round_trip() {
        let dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let services = Services::new(vec![
            Service("nginx".to_owned()),
            Service("docker".to_owned()),
        ]);
        services
            .save_applied_config(dir.path())
            .expect("temp dir is writable");
        let loaded = Services::read_applied_config(dir.path()).expect("config was just saved");
        let names: Vec<String> = loaded.services().iter().map(|s| s.service()).collect();
        assert!(names.contains(&"nginx".to_owned()));
        assert!(names.contains(&"docker".to_owned()));
    }

    #[test]
    fn read_applied_config_returns_none_when_missing() {
        let dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        assert!(Services::read_applied_config(dir.path()).is_none());
    }
}
