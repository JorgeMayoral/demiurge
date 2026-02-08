use std::{collections::HashMap, io::Write, path::PathBuf};

use anyhow::{Context, Result};
use rustyscript::{Module, Runtime, RuntimeOptions};
use serde::{Deserialize, Serialize};

pub use crate::config::dotfile::Dotfile;

mod dotfile;

const CURRENT_CONFIG_FILE_NAME: &str = "current_config";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Demiurge(HashMap<String, DemiurgeConfig>);

impl Demiurge {
    /// # Errors
    /// TODO
    pub fn from_file(file: PathBuf) -> Result<Self> {
        let module = Module::load(file)?;
        Runtime::execute_module(&module, vec![], RuntimeOptions::default(), &())
            .context("Failed to execute config module")
    }

    #[must_use]
    pub fn read_saved_config() -> Option<Self> {
        let data_dir = Self::get_data_dir();
        if !data_dir.exists() {
            return None;
        }
        let data = std::fs::read(data_dir.join(CURRENT_CONFIG_FILE_NAME)).ok()?;
        let applied_config_data = bitcode::deserialize(&data).ok()?;
        Some(applied_config_data)
    }

    /// # Errors
    /// TODO
    pub fn save_config(self) -> Result<()> {
        let data_dir = Self::get_data_dir();
        std::fs::create_dir_all(&data_dir)?;
        log::info!("Saving applied configuration in {}", &data_dir.display());
        let mut current_config_file =
            std::fs::File::create(data_dir.join(CURRENT_CONFIG_FILE_NAME))?;
        let current_config_data = bitcode::serialize(&self)?;
        current_config_file.write_all(&current_config_data)?;
        Ok(())
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<DemiurgeConfig> {
        self.0.get(name).cloned()
    }

    fn get_data_dir() -> PathBuf {
        let project_dir = directories::ProjectDirs::from("dev", "yorch", "demiurge").unwrap();
        project_dir.data_dir().to_path_buf()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DemiurgeConfig {
    system: System,
    packages: Packages,
    dotfiles: Vec<Dotfile>,
}

impl DemiurgeConfig {
    #[must_use]
    pub fn system(&self) -> System {
        self.system.clone()
    }

    #[must_use]
    pub fn packages(&self) -> Packages {
        self.packages.clone()
    }

    #[must_use]
    pub fn dotfiles(&self) -> Vec<Dotfile> {
        self.dotfiles.clone()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct System {
    hostname: String,
}

impl System {
    #[must_use]
    pub fn hostname(&self) -> String {
        self.hostname.clone()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
}
