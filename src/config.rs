use std::io::Write;

use anyhow::Result;
use pyo3::FromPyObject;
use serde::{Deserialize, Serialize};

#[derive(FromPyObject, Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    system: System,
    packages: Packages,
}

impl Config {
    pub fn get_system_config(&self) -> System {
        self.system.clone()
    }

    pub fn get_packages_config(&self) -> Packages {
        self.packages.clone()
    }

    pub fn save_config(self) -> Result<()> {
        let project_dir = directories::ProjectDirs::from("dev", "yorch", "demiurge").unwrap();
        let data_dir = project_dir.data_dir();
        std::fs::create_dir_all(data_dir).unwrap();
        log::info!("Saving applied configuration in {}", data_dir.display());
        let mut current_config_file =
            std::fs::File::create(data_dir.join("current_config")).unwrap();
        let current_config_data = bitcode::serialize(&self).unwrap();
        current_config_file.write(&current_config_data).unwrap();
        Ok(())
    }
}

#[derive(FromPyObject, Debug, Clone, Serialize, Deserialize)]
pub struct System {
    hostname: String,
}

impl System {
    pub fn get_configured_hostname(&self) -> String {
        self.hostname.clone()
    }
}

#[derive(FromPyObject, Debug, Clone, Default, Serialize, Deserialize)]
pub struct Packages {
    pacman: Vec<String>,
    aur: Vec<String>,
}

impl Packages {
    pub fn get_configured_pacman_pkgs(&self) -> Vec<String> {
        self.pacman.clone()
    }

    pub fn get_configured_aur_pkgs(&self) -> Vec<String> {
        self.aur.clone()
    }

    pub fn get_configured_pkgs(&self) -> Vec<String> {
        let mut pkgs = self.pacman.clone();
        pkgs.extend_from_slice(&self.aur);
        pkgs
    }
}
