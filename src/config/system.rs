use std::{io::Write, path::Path};

use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::config::CURRENT_SYSTEM_CONFIG_FILE_NAME;

#[derive(Debug, Clone, Deserialize, Serialize, Default, JsonSchema)]
pub struct System {
    hostname: String,
}

impl System {
    pub fn hostname(&self) -> String {
        self.hostname.clone()
    }

    pub fn read_applied_config(data_path: &Path) -> Option<Self> {
        let data = std::fs::read(data_path.join(CURRENT_SYSTEM_CONFIG_FILE_NAME)).ok()?;
        let applied_config_data = bitcode::deserialize(&data).ok()?;
        Some(applied_config_data)
    }

    pub fn save_applied_config(self, data_path: &Path) -> Result<()> {
        let mut current_config_file =
            std::fs::File::create(data_path.join(CURRENT_SYSTEM_CONFIG_FILE_NAME))
                .context("create system config file")?;
        let current_config_data = bitcode::serialize(&self).context("serialize system config")?;
        current_config_file
            .write_all(&current_config_data)
            .context("write system config file")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::System;

    #[test]
    fn system_persistence_round_trip() {
        let dir = tempfile::TempDir::new().unwrap();
        let system = System {
            hostname: "persistedhost".to_owned(),
        };
        system.save_applied_config(dir.path()).unwrap();
        let loaded = System::read_applied_config(dir.path()).unwrap();
        assert_eq!(loaded.hostname(), "persistedhost");
    }

    #[test]
    fn read_applied_config_returns_none_when_missing() {
        let dir = tempfile::TempDir::new().unwrap();
        let result = System::read_applied_config(dir.path());
        assert!(result.is_none());
    }
}
