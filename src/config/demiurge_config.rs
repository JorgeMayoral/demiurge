use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::config::{Dotfile, Packages, System, dotfile::Dotfiles};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct DemiurgeConfig {
    system: System,
    packages: Packages,
    dotfiles: Dotfiles,
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
        self.dotfiles.dotfiles()
    }

    #[must_use]
    pub fn read_applied_config() -> Self {
        let data_path = Self::get_data_dir();
        let applied_system_config = System::read_applied_config(&data_path).unwrap_or_default();
        let applied_packages_config = Packages::read_applied_config(&data_path).unwrap_or_default();
        let applied_dotfiles_config = Dotfiles::read_applied_config(&data_path).unwrap_or_default();
        Self {
            system: applied_system_config,
            packages: applied_packages_config,
            dotfiles: applied_dotfiles_config,
        }
    }

    /// # Errors
    /// TODO
    pub fn save_applied_config(self) -> Result<()> {
        let data_path = Self::get_data_dir();
        std::fs::create_dir_all(&data_path)?;
        log::info!("Saving applied configuration in {}", &data_path.display());
        self.system.save_applied_config(&data_path)?;
        self.packages.save_applied_config(&data_path)?;
        self.dotfiles.save_applied_config(&data_path)?;
        Ok(())
    }

    fn get_data_dir() -> PathBuf {
        let project_dir = directories::ProjectDirs::from("dev", "yorch", "demiurge").unwrap();
        project_dir.data_dir().to_path_buf()
    }
}
