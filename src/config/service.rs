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

    pub fn enable(&self) -> Result<()> {
        let service = self.0.clone();
        duct::cmd!("sudo", "systemctl", "start", &service)
            .run()
            .with_context(|| format!("start service {service}"))?;
        duct::cmd!("sudo", "systemctl", "enable", &service)
            .run()
            .with_context(|| format!("enable service {service}"))?;
        Ok(())
    }

    pub fn disable(&self) -> Result<()> {
        let service = self.0.clone();
        duct::cmd!("sudo", "systemctl", "stop", &service)
            .run()
            .with_context(|| format!("stop service {service}"))?;
        duct::cmd!("sudo", "systemctl", "disable", &service)
            .run()
            .with_context(|| format!("disable service {service}"))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Service, Services};

    #[test]
    fn services_persistence_round_trip() {
        let dir = tempfile::TempDir::new().unwrap();
        let services = Services::new(vec![
            Service("nginx".to_owned()),
            Service("docker".to_owned()),
        ]);
        services.save_applied_config(dir.path()).unwrap();
        let loaded = Services::read_applied_config(dir.path()).unwrap();
        let names: Vec<String> = loaded.services().iter().map(|s| s.service()).collect();
        assert!(names.contains(&"nginx".to_owned()));
        assert!(names.contains(&"docker".to_owned()));
    }

    #[test]
    fn read_applied_config_returns_none_when_missing() {
        let dir = tempfile::TempDir::new().unwrap();
        assert!(Services::read_applied_config(dir.path()).is_none());
    }
}
