use std::{
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{config::CURRENT_DOTFILES_CONFIG_FILE_NAME, utils};

#[derive(Debug, Clone, Deserialize, Serialize, Default, JsonSchema)]
pub struct Dotfiles(Vec<Dotfile>);

impl Dotfiles {
    pub fn new(dotfiles: Vec<Dotfile>) -> Self {
        Self(dotfiles)
    }

    pub fn dotfiles(&self) -> Vec<Dotfile> {
        self.0.clone()
    }

    pub fn validate(&self) -> Vec<String> {
        let mut errors = vec![];
        for dotfile in &self.0 {
            if dotfile.source.as_os_str().is_empty() {
                errors.push("dotfile source path must not be empty".to_owned());
            }
            if dotfile.target.as_os_str().is_empty() {
                errors.push("dotfile target path must not be empty".to_owned());
            }
        }
        errors
    }

    pub fn read_applied_config(data_path: &Path) -> Option<Self> {
        let data = std::fs::read(data_path.join(CURRENT_DOTFILES_CONFIG_FILE_NAME)).ok()?;
        let applied_config_data = bitcode::deserialize(&data).ok()?;
        Some(applied_config_data)
    }

    pub fn save_applied_config(self, data_path: &Path) -> Result<()> {
        let mut current_config_file =
            std::fs::File::create(data_path.join(CURRENT_DOTFILES_CONFIG_FILE_NAME))
                .context("create dotfiles config file")?;
        let current_config_data = bitcode::serialize(&self).context("serialize dotfiles config")?;
        current_config_file
            .write_all(&current_config_data)
            .context("write dotfiles config file")?;
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
        let source_path = utils::path_tilde_expand(self.source())
            .context("expand source dotfile path")?
            .canonicalize()
            .context("canonicalize source dotfile path")?;
        let target_path =
            utils::path_tilde_expand(self.target()).context("expand target dotfile path")?;
        if !source_path.exists() {
            let error_msg = format!(
                "The source path \"{}\" doesn't exist.",
                source_path.display()
            );
            log::error!("{error_msg}");
            return Err(anyhow::anyhow!(error_msg));
        }
        for entry in walkdir::WalkDir::new(&source_path) {
            let entry = entry.context("read directory entry")?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                continue;
            }
            let relative_path = entry_path
                .strip_prefix(&source_path)
                .context("compute relative dotfile path")?;
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
                    std::fs::remove_dir_all(&destination_path)
                        .context("remove existing directory at symlink destination")?;
                } else {
                    std::fs::remove_file(&destination_path)
                        .context("remove existing file at symlink destination")?;
                }
            }
            if let Some(parent_dir) = destination_path.parent() {
                std::fs::create_dir_all(parent_dir)
                    .context("create parent directories for symlink target")?;
            }
            std::os::unix::fs::symlink(entry_path, &destination_path).with_context(|| {
                format!(
                    "create symlink from \"{}\" to \"{}\"",
                    entry_path.display(),
                    destination_path.display()
                )
            })?;
            log::info!(
                "Symlink created from \"{}\" to \"{}\"",
                entry_path.display(),
                destination_path.display()
            );
        }
        Ok(())
    }

    pub fn remove_symlink(&self) -> Result<()> {
        let target_path = utils::path_tilde_expand(self.target())
            .context("expand target dotfile path for removal")?;
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
    fn validate_valid_dotfiles_is_ok() {
        let dotfiles = Dotfiles::new(vec![Dotfile {
            source: "/dotfiles/nvim".into(),
            target: "/home/user/.config/nvim".into(),
        }]);
        assert!(dotfiles.validate().is_empty());
    }

    #[test]
    fn validate_empty_source_is_invalid() {
        let dotfiles = Dotfiles::new(vec![Dotfile {
            source: "".into(),
            target: "/home/user/.config/nvim".into(),
        }]);
        let errors = dotfiles.validate();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("source"));
    }

    #[test]
    fn validate_empty_target_is_invalid() {
        let dotfiles = Dotfiles::new(vec![Dotfile {
            source: "/dotfiles/nvim".into(),
            target: "".into(),
        }]);
        let errors = dotfiles.validate();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("target"));
    }

    #[test]
    fn validate_both_empty_reports_two_errors() {
        let dotfiles = Dotfiles::new(vec![Dotfile {
            source: "".into(),
            target: "".into(),
        }]);
        assert_eq!(dotfiles.validate().len(), 2);
    }

    #[test]
    fn dotfiles_persistence_round_trip() {
        let dir = tempfile::TempDir::new().expect("OS can create a temp directory");
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
        dotfiles
            .save_applied_config(dir.path())
            .expect("temp dir is writable");
        let loaded = Dotfiles::read_applied_config(dir.path()).expect("config was just saved");
        assert_eq!(loaded.dotfiles().len(), 2);
    }

    #[test]
    fn read_applied_config_returns_none_when_missing() {
        let dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        assert!(Dotfiles::read_applied_config(dir.path()).is_none());
    }

    #[test]
    fn create_symlink_links_single_file() {
        let src_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let tgt_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let src_file = src_dir.path().join("file.txt");
        std::fs::write(&src_file, "hello").expect("temp dir is writable");

        let dotfile = Dotfile {
            source: src_dir.path().to_path_buf(),
            target: tgt_dir.path().to_path_buf(),
        };
        dotfile
            .create_symlink(false)
            .expect("source exists and target dir is empty");

        let link = tgt_dir.path().join("file.txt");
        assert!(
            link.is_symlink(),
            "expected a symlink at {}",
            link.display()
        );
        assert_eq!(
            std::fs::read_to_string(&link).expect("file was just written"),
            "hello"
        );
    }

    #[test]
    fn create_symlink_recurses_into_subdirectories() {
        let src_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let tgt_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        std::fs::create_dir(src_dir.path().join("sub")).expect("temp dir is writable");
        std::fs::write(src_dir.path().join("root.txt"), "root").expect("temp dir is writable");
        std::fs::write(src_dir.path().join("sub").join("nested.txt"), "nested")
            .expect("temp dir is writable");

        let dotfile = Dotfile {
            source: src_dir.path().to_path_buf(),
            target: tgt_dir.path().to_path_buf(),
        };
        dotfile
            .create_symlink(false)
            .expect("source exists and target dir is empty");

        assert!(tgt_dir.path().join("root.txt").is_symlink());
        assert!(tgt_dir.path().join("sub").join("nested.txt").is_symlink());
    }

    #[test]
    fn create_symlink_with_overwrite_replaces_existing() {
        let src_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let tgt_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let src_file = src_dir.path().join("file.txt");
        std::fs::write(&src_file, "new content").expect("temp dir is writable");
        // pre-place a regular file at the destination
        std::fs::write(tgt_dir.path().join("file.txt"), "old content")
            .expect("temp dir is writable");

        let dotfile = Dotfile {
            source: src_dir.path().to_path_buf(),
            target: tgt_dir.path().to_path_buf(),
        };
        dotfile
            .create_symlink(true)
            .expect("source exists and target dir is empty");

        let link = tgt_dir.path().join("file.txt");
        assert!(link.is_symlink());
        assert_eq!(
            std::fs::read_to_string(&link).expect("file was just written"),
            "new content"
        );
    }

    #[test]
    fn remove_symlink_deletes_link_and_leaves_source_intact() {
        let src_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let tgt_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let src_file = src_dir.path().join("file.txt");
        std::fs::write(&src_file, "data").expect("temp dir is writable");

        let dotfile = Dotfile {
            source: src_dir.path().to_path_buf(),
            target: tgt_dir.path().to_path_buf(),
        };
        dotfile
            .create_symlink(false)
            .expect("source exists and target dir is empty");
        assert!(tgt_dir.path().join("file.txt").is_symlink());

        // remove_symlink operates on the target directory
        let remove_dotfile = Dotfile {
            source: src_dir.path().to_path_buf(),
            target: tgt_dir.path().join("file.txt"),
        };
        remove_dotfile
            .remove_symlink()
            .expect("target path is valid");

        assert!(!tgt_dir.path().join("file.txt").exists());
        assert!(src_file.exists(), "source file must survive removal");
    }

    #[test]
    fn remove_symlink_is_no_op_when_target_is_absent() {
        let src_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let tgt_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let dotfile = Dotfile {
            source: src_dir.path().to_path_buf(),
            target: tgt_dir.path().join("nonexistent"),
        };
        // should not error
        dotfile.remove_symlink().expect("target path is valid");
    }
}
