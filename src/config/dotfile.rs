use std::{
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::utils;

const CURRENT_DOTFILES_CONFIG_FILE_NAME: &str = "current_dotfiles_config";

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Dotfiles(Vec<Dotfile>);

impl Dotfiles {
    #[must_use]
    pub fn dotfiles(&self) -> Vec<Dotfile> {
        self.0.clone()
    }

    #[must_use]
    pub fn read_applied_config(data_path: &Path) -> Option<Self> {
        let data = std::fs::read(data_path.join(CURRENT_DOTFILES_CONFIG_FILE_NAME)).ok()?;
        let applied_config_data = bitcode::deserialize(&data).ok()?;
        Some(applied_config_data)
    }

    /// # Errors
    /// TODO
    pub fn save_applied_config(self, data_path: &Path) -> Result<()> {
        let mut current_config_file =
            std::fs::File::create(data_path.join(CURRENT_DOTFILES_CONFIG_FILE_NAME))?;
        let current_config_data = bitcode::serialize(&self)?;
        current_config_file.write_all(&current_config_data)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Dotfile {
    source: PathBuf,
    target: PathBuf,
}

impl Dotfile {
    #[must_use]
    pub fn new(source: PathBuf, target: PathBuf) -> Self {
        Self { source, target }
    }

    #[must_use]
    pub fn source(&self) -> PathBuf {
        self.source.clone()
    }

    #[must_use]
    pub fn target(&self) -> PathBuf {
        self.target.clone()
    }

    /// # Errors
    /// TODO
    pub fn create_symlink(&self, overwrite: bool) -> Result<()> {
        let source_path = utils::path_tilde_expand(self.source()).canonicalize()?;
        let target_path = utils::path_tilde_expand(self.target());
        if !source_path.exists() {
            log::error!(
                "The source path \"{}\" doesn't exists.",
                source_path.display()
            );
            std::process::exit(1);
        }
        for entry in walkdir::WalkDir::new(&source_path) {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                continue;
            }
            let relative_path = entry_path.strip_prefix(&source_path)?;
            let destination_path = target_path.join(relative_path);
            if destination_path.exists() || destination_path.is_symlink() {
                if !overwrite {
                    log::error!(
                        "The target path \"{}\" already exists or is a symlink",
                        destination_path.display()
                    );
                    std::process::exit(1);
                }
                log::warn!("Removing {}", destination_path.display());
                if destination_path.is_dir() && !destination_path.is_symlink() {
                    std::fs::remove_dir_all(&destination_path)?;
                } else {
                    std::fs::remove_file(&destination_path)?;
                }
            }
            if let Some(parent_dir) = destination_path.parent() {
                std::fs::create_dir_all(parent_dir)?;
            }
            std::os::unix::fs::symlink(entry_path, &destination_path)?;
            log::info!(
                "Symlink created from \"{}\" to \"{}\"",
                entry_path.display(),
                destination_path.display()
            );
        }
        Ok(())
    }

    /// # Errors
    /// TODO
    pub fn remove_symlink(&self) -> Result<()> {
        let target_path = utils::path_tilde_expand(self.target());
        if !target_path.exists() && !target_path.is_symlink() {
            return Ok(());
        }
        std::fs::remove_dir_all(target_path.clone()).context(format!(
            "Couldn't remove target path \"{}\"",
            target_path.display()
        ))
    }
}
