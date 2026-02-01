use std::{io::Write, path::PathBuf};

use anyhow::Result;
use pyo3::FromPyObject;
use serde::{Deserialize, Serialize};

const CURRENT_CONFIG_FILE_NAME: &'static str = "current_config";

#[derive(FromPyObject, Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    system: System,
    packages: Packages,
}

impl Config {
    pub fn system_config(&self) -> System {
        self.system.clone()
    }

    pub fn packages_config(&self) -> Packages {
        self.packages.clone()
    }

    pub fn read_saved_config() -> Option<Self> {
        let data_dir = Self::get_data_dir();
        if !data_dir.exists() {
            return None;
        }
        let data = std::fs::read(data_dir.join(CURRENT_CONFIG_FILE_NAME)).ok()?;
        let applied_config_data = bitcode::deserialize(&data).ok()?;
        Some(applied_config_data)
    }

    pub fn save_config(self) -> Result<()> {
        let data_dir = Self::get_data_dir();
        std::fs::create_dir_all(&data_dir).unwrap();
        log::info!("Saving applied configuration in {}", &data_dir.display());
        let mut current_config_file =
            std::fs::File::create(data_dir.join(CURRENT_CONFIG_FILE_NAME)).unwrap();
        let current_config_data = bitcode::serialize(&self).unwrap();
        current_config_file.write(&current_config_data).unwrap();
        Ok(())
    }

    fn get_data_dir() -> PathBuf {
        let project_dir = directories::ProjectDirs::from("dev", "yorch", "demiurge").unwrap();
        project_dir.data_dir().to_path_buf()
    }
}

#[derive(FromPyObject, Debug, Clone, Serialize, Deserialize)]
pub struct System {
    hostname: String,
}

impl System {
    pub fn hostname(&self) -> String {
        self.hostname.clone()
    }
}

#[derive(FromPyObject, Debug, Clone, Default, Serialize, Deserialize)]
pub struct Packages {
    pacman: Vec<String>,
    aur: Vec<String>,
}

impl Packages {
    pub fn pacman_pkgs(&self) -> Vec<String> {
        self.pacman.clone()
    }

    pub fn aur_pkgs(&self) -> Vec<String> {
        self.aur.clone()
    }

    pub fn pkgs(&self) -> Vec<String> {
        let mut pkgs = self.pacman.clone();
        pkgs.extend_from_slice(&self.aur);
        pkgs
    }
}
