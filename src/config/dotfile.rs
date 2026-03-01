use std::{
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::utils;

const CURRENT_DOTFILES_CONFIG_FILE_NAME: &str = "current_dotfiles_config";

#[derive(Debug, Clone, Deserialize, Serialize, Default, JsonSchema)]
pub struct Dotfiles(Vec<Dotfile>);

impl Dotfiles {
    pub fn new(dotfiles: Vec<Dotfile>) -> Self {
        Self(dotfiles)
    }

    pub fn dotfiles(&self) -> Vec<Dotfile> {
        self.0.clone()
    }

    pub fn read_applied_config(data_path: &Path) -> Option<Self> {
        let data = std::fs::read(data_path.join(CURRENT_DOTFILES_CONFIG_FILE_NAME)).ok()?;
        let applied_config_data = bitcode::deserialize(&data).ok()?;
        Some(applied_config_data)
    }

    pub fn save_applied_config(self, data_path: &Path) -> Result<()> {
        let mut current_config_file =
            std::fs::File::create(data_path.join(CURRENT_DOTFILES_CONFIG_FILE_NAME))?;
        let current_config_data = bitcode::serialize(&self)?;
        current_config_file.write_all(&current_config_data)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
pub struct Dotfile {
    source: PathBuf,
    target: PathBuf,
}

impl Dotfile {
    pub fn source(&self) -> PathBuf {
        self.source.clone()
    }

    pub fn target(&self) -> PathBuf {
        self.target.clone()
    }

    pub fn create_symlink(&self, overwrite: bool) -> Result<()> {
        let source_path = utils::path_tilde_expand(self.source()).canonicalize()?;
        let target_path = utils::path_tilde_expand(self.target());
        if !source_path.exists() {
            let error_msg = format!(
                "The source path \"{}\" doesn't exists.",
                source_path.display()
            );
            log::error!("{error_msg}");
            return Err(anyhow::anyhow!(error_msg));
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
                    let error_msg = format!(
                        "The target path \"{}\" already exists or is a symlink",
                        destination_path.display()
                    );
                    log::error!("{error_msg}");
                    return Err(anyhow::anyhow!(error_msg));
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

#[cfg(test)]
mod tests {
    use super::{Dotfile, Dotfiles};

    #[test]
    fn dotfiles_persistence_round_trip() {
        let dir = tempfile::TempDir::new().unwrap();
        let dotfiles = Dotfiles::new(vec![
            Dotfile {
                source: "/dotfiles/nvim".into(),
                target: "/home/user/.config/nvim".into(),
            },
            Dotfile {
                source: "/dotfiles/zsh".into(),
                target: "/home/user/.zshrc".into(),
            },
        ]);
        dotfiles.save_applied_config(dir.path()).unwrap();
        let loaded = Dotfiles::read_applied_config(dir.path()).unwrap();
        assert_eq!(loaded.dotfiles().len(), 2);
    }

    #[test]
    fn read_applied_config_returns_none_when_missing() {
        let dir = tempfile::TempDir::new().unwrap();
        assert!(Dotfiles::read_applied_config(dir.path()).is_none());
    }

    #[test]
    fn create_symlink_links_single_file() {
        let src_dir = tempfile::TempDir::new().unwrap();
        let tgt_dir = tempfile::TempDir::new().unwrap();
        let src_file = src_dir.path().join("file.txt");
        std::fs::write(&src_file, "hello").unwrap();

        let dotfile = Dotfile {
            source: src_dir.path().to_path_buf(),
            target: tgt_dir.path().to_path_buf(),
        };
        dotfile.create_symlink(false).unwrap();

        let link = tgt_dir.path().join("file.txt");
        assert!(
            link.is_symlink(),
            "expected a symlink at {}",
            link.display()
        );
        assert_eq!(std::fs::read_to_string(&link).unwrap(), "hello");
    }

    #[test]
    fn create_symlink_recurses_into_subdirectories() {
        let src_dir = tempfile::TempDir::new().unwrap();
        let tgt_dir = tempfile::TempDir::new().unwrap();
        std::fs::create_dir(src_dir.path().join("sub")).unwrap();
        std::fs::write(src_dir.path().join("root.txt"), "root").unwrap();
        std::fs::write(src_dir.path().join("sub").join("nested.txt"), "nested").unwrap();

        let dotfile = Dotfile {
            source: src_dir.path().to_path_buf(),
            target: tgt_dir.path().to_path_buf(),
        };
        dotfile.create_symlink(false).unwrap();

        assert!(tgt_dir.path().join("root.txt").is_symlink());
        assert!(tgt_dir.path().join("sub").join("nested.txt").is_symlink());
    }

    #[test]
    fn create_symlink_with_overwrite_replaces_existing() {
        let src_dir = tempfile::TempDir::new().unwrap();
        let tgt_dir = tempfile::TempDir::new().unwrap();
        let src_file = src_dir.path().join("file.txt");
        std::fs::write(&src_file, "new content").unwrap();
        // pre-place a regular file at the destination
        std::fs::write(tgt_dir.path().join("file.txt"), "old content").unwrap();

        let dotfile = Dotfile {
            source: src_dir.path().to_path_buf(),
            target: tgt_dir.path().to_path_buf(),
        };
        dotfile.create_symlink(true).unwrap();

        let link = tgt_dir.path().join("file.txt");
        assert!(link.is_symlink());
        assert_eq!(std::fs::read_to_string(&link).unwrap(), "new content");
    }

    #[test]
    fn remove_symlink_deletes_link_and_leaves_source_intact() {
        let src_dir = tempfile::TempDir::new().unwrap();
        let tgt_dir = tempfile::TempDir::new().unwrap();
        let src_file = src_dir.path().join("file.txt");
        std::fs::write(&src_file, "data").unwrap();

        let dotfile = Dotfile {
            source: src_dir.path().to_path_buf(),
            target: tgt_dir.path().to_path_buf(),
        };
        dotfile.create_symlink(false).unwrap();
        assert!(tgt_dir.path().join("file.txt").is_symlink());

        // remove_symlink operates on the target directory
        let remove_dotfile = Dotfile {
            source: src_dir.path().to_path_buf(),
            target: tgt_dir.path().join("file.txt"),
        };
        remove_dotfile.remove_symlink().unwrap();

        assert!(!tgt_dir.path().join("file.txt").exists());
        assert!(src_file.exists(), "source file must survive removal");
    }

    #[test]
    fn remove_symlink_is_no_op_when_target_is_absent() {
        let src_dir = tempfile::TempDir::new().unwrap();
        let tgt_dir = tempfile::TempDir::new().unwrap();
        let dotfile = Dotfile {
            source: src_dir.path().to_path_buf(),
            target: tgt_dir.path().join("nonexistent"),
        };
        // should not error
        dotfile.remove_symlink().unwrap();
    }
}
