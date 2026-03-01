use std::fmt::Display;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::config::{Dotfile, Dotfiles};

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

    pub fn apply(&self, overwrite: bool) -> Result<()> {
        for dotfile in self.create.dotfiles() {
            dotfile.create_symlink(overwrite)?;
        }

        for dotfile in self.remove.dotfiles() {
            dotfile.remove_symlink()?;
        }

        Ok(())
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
    use crate::config::Dotfiles;

    fn dots(json: &str) -> Dotfiles {
        serde_json::from_str(json).unwrap()
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
}
