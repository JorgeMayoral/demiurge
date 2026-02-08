use std::{io::Write, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const CURRENT_PACKAGES_CONFIG_FILE_NAME: &str = "current_packages_config";

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Packages {
    paru: Vec<String>,
}

impl Packages {
    #[must_use]
    pub fn new(paru_pkgs: Vec<String>) -> Self {
        Self { paru: paru_pkgs }
    }

    #[must_use]
    pub fn paru(&self) -> Vec<String> {
        self.paru.clone()
    }

    #[must_use]
    pub fn read_applied_config(data_path: &Path) -> Option<Self> {
        let data = std::fs::read(data_path.join(CURRENT_PACKAGES_CONFIG_FILE_NAME)).ok()?;
        let applied_config_data = bitcode::deserialize(&data).ok()?;
        Some(applied_config_data)
    }

    /// # Errors
    /// TODO
    pub fn save_applied_config(self, data_path: &Path) -> Result<()> {
        let mut current_config_file =
            std::fs::File::create(data_path.join(CURRENT_PACKAGES_CONFIG_FILE_NAME))?;
        let current_config_data = bitcode::serialize(&self)?;
        current_config_file.write_all(&current_config_data)?;
        Ok(())
    }
}
