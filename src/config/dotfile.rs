use std::{
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::config::CURRENT_DOTFILES_CONFIG_FILE_NAME;

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
}
