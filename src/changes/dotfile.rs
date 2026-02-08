use std::fmt::Display;

use owo_colors::OwoColorize;

use crate::config::Dotfile;

#[derive(Debug, Clone)]
pub struct DotfileChanges {
    create: Vec<Dotfile>,
    remove: Vec<Dotfile>,
}

impl DotfileChanges {
    #[must_use]
    pub fn new(
        new_dotfiles_config: &[Dotfile],
        applied_dotfiles_config: Option<Vec<Dotfile>>,
    ) -> Self {
        let applied_dotfiles = applied_dotfiles_config.unwrap_or_default();
        println!("{applied_dotfiles:#?}");

        let symlinks_to_create = new_dotfiles_config
            .iter()
            .filter(|dot| !applied_dotfiles.contains(dot))
            .map(ToOwned::to_owned)
            .collect();
        let symlinks_to_remove = applied_dotfiles
            .iter()
            .filter(|dot| !new_dotfiles_config.contains(dot))
            .map(ToOwned::to_owned)
            .collect();

        Self {
            create: symlinks_to_create,
            remove: symlinks_to_remove,
        }
    }

    pub fn apply(&self, overwrite: bool) {
        self.create
            .iter()
            .for_each(|dotfile| dotfile.create_symlink(overwrite).unwrap());

        self.remove
            .iter()
            .for_each(|dotfile| dotfile.remove_symlink().unwrap());
    }
}

impl Display for DotfileChanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "Dotfile symlinks".blue().bold().to_string();
        let symlinks_to_create = self
            .create
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
