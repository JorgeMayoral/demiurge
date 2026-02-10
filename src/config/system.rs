use std::{io::Write, path::Path};

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const CURRENT_SYSTEM_CONFIG_FILE_NAME: &str = "current_system_config";

#[derive(Debug, Clone, Deserialize, Serialize, Default, JsonSchema)]
pub struct System {
    hostname: String,
}

impl System {
    #[must_use]
    pub fn hostname(&self) -> String {
        self.hostname.clone()
    }

    #[must_use]
    pub fn read_applied_config(data_path: &Path) -> Option<Self> {
        let data = std::fs::read(data_path.join(CURRENT_SYSTEM_CONFIG_FILE_NAME)).ok()?;
        let applied_config_data = bitcode::deserialize(&data).ok()?;
        Some(applied_config_data)
    }

    /// # Errors
    /// TODO
    pub fn save_applied_config(self, data_path: &Path) -> Result<()> {
        let mut current_config_file =
            std::fs::File::create(data_path.join(CURRENT_SYSTEM_CONFIG_FILE_NAME))?;
        let current_config_data = bitcode::serialize(&self)?;
        current_config_file.write_all(&current_config_data)?;
        Ok(())
    }
}
