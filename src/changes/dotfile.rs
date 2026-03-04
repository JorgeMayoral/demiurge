use std::fmt::Display;

use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::{
    config::{Dotfile, Dotfiles},
    utils,
};

#[derive(Debug, Clone)]
pub struct DotfileChanges {
    pub create: Dotfiles,
    pub remove: Dotfiles,
}

impl DotfileChanges {
    pub fn new(new_dotfiles_config: &Dotfiles, applied_dotfiles_config: &Dotfiles) -> Self {
        let symlinks_to_create: Vec<Dotfile> = new_dotfiles_config
            .dotfiles()
            .iter()
            .filter(|dot| !applied_dotfiles_config.dotfiles().contains(dot))
            .map(ToOwned::to_owned)
            .collect();
        let symlinks_to_remove: Vec<Dotfile> = applied_dotfiles_config
            .dotfiles()
            .iter()
            .filter(|dot| !new_dotfiles_config.dotfiles().contains(dot))
            .map(ToOwned::to_owned)
            .collect();

        Self {
            create: Dotfiles::new(symlinks_to_create),
            remove: Dotfiles::new(symlinks_to_remove),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.create.dotfiles().is_empty() && self.remove.dotfiles().is_empty()
    }

    pub fn apply(&self, overwrite: bool) -> Result<()> {
        for dotfile in self.create.dotfiles() {
            Self::create_symlink(&dotfile, overwrite).with_context(|| {
                format!(
                    "create symlink for dotfile \"{}\"",
                    dotfile.source().display()
                )
            })?;
        }

        for dotfile in self.remove.dotfiles() {
            Self::remove_symlink(&dotfile).with_context(|| {
                format!(
                    "remove symlink for dotfile \"{}\"",
                    dotfile.source().display()
                )
            })?;
        }

        Ok(())
    }

    fn create_symlink(dotfile: &Dotfile, overwrite: bool) -> Result<()> {
        let source_path = utils::path_tilde_expand(dotfile.source())
            .context("expand source dotfile path")?
            .canonicalize()
            .context("canonicalize source dotfile path")?;
        let target_path =
            utils::path_tilde_expand(dotfile.target()).context("expand target dotfile path")?;

        if !source_path.exists() {
            let error_msg = format!(
                "The source path \"{}\" doesn't exist.",
                source_path.display()
            );
            log::error!("{error_msg}");
            return Err(anyhow::anyhow!(error_msg));
        }

        if target_path.exists() || target_path.is_symlink() {
            if !overwrite {
                let error_msg = format!(
                    "The target path \"{}\" already exists or is a symlink",
                    target_path.display()
                );
                log::error!("{error_msg}");
                return Err(anyhow::anyhow!(error_msg));
            }
            log::warn!("Removing {}", target_path.display());
            if target_path.is_dir() && !target_path.is_symlink() {
                std::fs::remove_dir_all(&target_path)
                    .context("remove existing directory at symlink destination")?;
            } else {
                std::fs::remove_file(&target_path)
                    .context("remove existing file at symlink destination")?;
            }
        }

        if let Some(parent_dir) = target_path.parent() {
            std::fs::create_dir_all(parent_dir)
                .context("create parent directories for symlink target")?;
        }

        std::os::unix::fs::symlink(&source_path, &target_path).with_context(|| {
            format!(
                "create symlink from \"{}\" to \"{}\"",
                source_path.display(),
                target_path.display()
            )
        })?;
        log::info!(
            "Symlink created from \"{}\" to \"{}\"",
            source_path.display(),
            target_path.display()
        );
        Ok(())
    }

    fn remove_symlink(dotfile: &Dotfile) -> Result<()> {
        let target_path = utils::path_tilde_expand(dotfile.target())
            .context("expand target dotfile path for removal")?;
        if !target_path.exists() && !target_path.is_symlink() {
            return Ok(());
        }
        log::info!("Removing symlink at \"{}\"", target_path.display());
        std::fs::remove_dir_all(&target_path).context(format!(
            "Couldn't remove target path \"{}\"",
            target_path.display()
        ))
    }
}

impl Display for DotfileChanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "Dotfile symlinks".blue().bold().to_string();
        let symlinks_to_create = self
            .create
            .dotfiles()
            .iter()
            .map(|dotfile| {
                let symbol = "+".green().to_string();
                format!(
                    "[{}] {} => {}",
                    symbol,
                    dotfile.source().display(),
                    dotfile.target().display()
                )
            })
            .collect::<Vec<String>>();
        let symlinks_to_create_text = if symlinks_to_create.is_empty() {
            "No symlinks to create".yellow().to_string()
        } else {
            symlinks_to_create.join("\n")
        };

        let symlinks_to_remove = self
            .remove
            .dotfiles()
            .iter()
            .map(|dotfile| {
                let symbol = "-".red().to_string();
                format!(
                    "[{}] {} => {}",
                    symbol,
                    dotfile.source().display(),
                    dotfile.target().display()
                )
            })
            .collect::<Vec<String>>();
        let symlinks_to_remove_text = if symlinks_to_remove.is_empty() {
            "No symlinks to remove".yellow().to_string()
        } else {
            symlinks_to_remove.join("\n")
        };

        let text = format!("{title}\n{symlinks_to_create_text}\n\n{symlinks_to_remove_text}");
        write!(f, "{text}")
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::DotfileChanges;
    use crate::config::{Dotfile, Dotfiles};

    fn dots(json: &str) -> Dotfiles {
        serde_json::from_str(json).expect("literal is well-formed JSON")
    }

    #[test]
    fn apply_does_nothing_when_no_changes() {
        let changes = DotfileChanges {
            create: dots(r#"[]"#),
            remove: dots(r#"[]"#),
        };
        assert!(changes.apply(false).is_ok());
    }

    #[test]
    fn apply_create_fails_when_source_does_not_exist() {
        let tgt_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let create = dots(&format!(
            r#"[{{"source": "/demiurge-test-nonexistent-src", "target": "{}"}}]"#,
            tgt_dir.path().display()
        ));
        let changes = DotfileChanges {
            create,
            remove: dots(r#"[]"#),
        };
        let err = changes.apply(false).unwrap_err();
        assert!(format!("{err:?}").contains("create symlink for dotfile"));
    }

    #[test]
    fn apply_create_fails_when_target_exists_and_overwrite_is_false() {
        let src_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let tgt_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        std::fs::write(src_dir.path().join("file.txt"), "src").expect("temp dir is writable");
        std::fs::write(tgt_dir.path().join("file.txt"), "existing").expect("temp dir is writable");

        let create = dots(&format!(
            r#"[{{"source": "{}", "target": "{}"}}]"#,
            src_dir.path().display(),
            tgt_dir.path().display()
        ));
        let changes = DotfileChanges {
            create,
            remove: dots(r#"[]"#),
        };
        assert!(changes.apply(false).is_err());
    }

    #[test]
    fn apply_remove_is_no_op_when_target_absent() {
        let src_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let remove = dots(&format!(
            r#"[{{"source": "{}", "target": "/demiurge-test-nonexistent-tgt"}}]"#,
            src_dir.path().display()
        ));
        let changes = DotfileChanges {
            create: dots(r#"[]"#),
            remove,
        };
        assert!(changes.apply(false).is_ok());
    }

    #[test]
    fn new_dotfile_goes_to_create_set() {
        let new = dots(r#"[{"source": "/dotfiles/nvim", "target": "/home/user/.config/nvim"}]"#);
        let applied = dots(r#"[]"#);
        let changes = DotfileChanges::new(&new, &applied);
        assert_eq!(changes.create.dotfiles().len(), 1);
        assert_eq!(
            changes.create.dotfiles()[0].source(),
            PathBuf::from("/dotfiles/nvim")
        );
        assert!(changes.remove.dotfiles().is_empty());
    }

    #[test]
    fn removed_dotfile_goes_to_remove_set() {
        let new = dots(r#"[]"#);
        let applied =
            dots(r#"[{"source": "/dotfiles/nvim", "target": "/home/user/.config/nvim"}]"#);
        let changes = DotfileChanges::new(&new, &applied);
        assert_eq!(changes.remove.dotfiles().len(), 1);
        assert!(changes.create.dotfiles().is_empty());
    }

    #[test]
    fn unchanged_dotfiles_produce_empty_sets() {
        let dots_config =
            dots(r#"[{"source": "/dotfiles/nvim", "target": "/home/user/.config/nvim"}]"#);
        let changes = DotfileChanges::new(&dots_config, &dots_config);
        assert!(changes.create.dotfiles().is_empty());
        assert!(changes.remove.dotfiles().is_empty());
    }

    fn make_dotfile(src: &std::path::Path, tgt: &std::path::Path) -> Dotfile {
        serde_json::from_str(&format!(
            r#"{{"source": "{}", "target": "{}"}}"#,
            src.display(),
            tgt.display()
        ))
        .expect("paths are valid JSON strings")
    }

    #[test]
    fn create_symlink_links_single_file() {
        let src_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let parent = tempfile::TempDir::new().expect("OS can create a temp directory");
        let src_file = src_dir.path().join("file.txt");
        std::fs::write(&src_file, "hello").expect("temp dir is writable");

        let link = parent.path().join("link");
        let dotfile = make_dotfile(src_dir.path(), &link);
        DotfileChanges::create_symlink(&dotfile, false)
            .expect("source exists and target path is non-existent");

        assert!(
            link.is_symlink(),
            "expected a symlink at {}",
            link.display()
        );
        assert_eq!(
            std::fs::read_to_string(link.join("file.txt"))
                .expect("file is readable through symlink"),
            "hello"
        );
    }

    #[test]
    fn create_symlink_for_directory_gives_access_to_contents() {
        let src_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let parent = tempfile::TempDir::new().expect("OS can create a temp directory");
        std::fs::create_dir(src_dir.path().join("sub")).expect("temp dir is writable");
        std::fs::write(src_dir.path().join("root.txt"), "root").expect("temp dir is writable");
        std::fs::write(src_dir.path().join("sub").join("nested.txt"), "nested")
            .expect("temp dir is writable");

        let link = parent.path().join("link");
        let dotfile = make_dotfile(src_dir.path(), &link);
        DotfileChanges::create_symlink(&dotfile, false)
            .expect("source exists and target path is non-existent");

        assert!(link.is_symlink());
        assert!(link.join("root.txt").exists());
        assert!(link.join("sub").join("nested.txt").exists());
    }

    #[test]
    fn create_symlink_with_overwrite_replaces_existing() {
        let src_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let parent = tempfile::TempDir::new().expect("OS can create a temp directory");
        std::fs::write(src_dir.path().join("file.txt"), "new content")
            .expect("temp dir is writable");
        let link = parent.path().join("link");
        std::fs::write(&link, "old content").expect("temp dir is writable");

        let dotfile = make_dotfile(src_dir.path(), &link);
        DotfileChanges::create_symlink(&dotfile, true)
            .expect("source exists and overwrite is true");

        assert!(link.is_symlink());
        assert_eq!(
            std::fs::read_to_string(link.join("file.txt"))
                .expect("file is readable through symlink"),
            "new content"
        );
    }

    #[test]
    fn remove_symlink_deletes_link_and_leaves_source_intact() {
        let src_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let parent = tempfile::TempDir::new().expect("OS can create a temp directory");
        let src_file = src_dir.path().join("file.txt");
        std::fs::write(&src_file, "data").expect("temp dir is writable");

        let link = parent.path().join("link");
        let dotfile = make_dotfile(src_dir.path(), &link);
        DotfileChanges::create_symlink(&dotfile, false)
            .expect("source exists and target path is non-existent");
        assert!(link.is_symlink());

        DotfileChanges::remove_symlink(&dotfile).expect("target path is valid");

        assert!(!link.exists(), "symlink should be removed");
        assert!(src_file.exists(), "source file must survive removal");
    }

    #[test]
    fn remove_symlink_is_no_op_when_target_is_absent() {
        let src_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let tgt_dir = tempfile::TempDir::new().expect("OS can create a temp directory");
        let dotfile = make_dotfile(src_dir.path(), &tgt_dir.path().join("nonexistent"));
        DotfileChanges::remove_symlink(&dotfile).expect("target path is valid");
    }
}
