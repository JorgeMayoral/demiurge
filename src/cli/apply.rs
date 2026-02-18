use std::{io::Read, path::PathBuf};

use anyhow::{Context, Result};
use clap::Args;

use crate::{
    changes::Changes,
    config::{Demiurge, DemiurgeConfig},
};

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Args, Clone)]
pub struct ApplyArgs {
    /// Show the list of changes that would be made without applying them.
    #[arg(short, long)]
    dry_run: bool,
    /// The path to the file containing the configuration
    #[arg(short, long, required_unless_present = "stdin")]
    file: Option<PathBuf>,
    /// Name of the configuration to apply
    #[arg(short, long)]
    name: String,
    /// Allows overwriting already existing dotfile symlinks
    #[arg(long)]
    overwrite_symlink: bool,
    #[clap(flatten)]
    from_group: FromGroup,
    /// Read the given configuration from stdin. Requires JSON or YAML flag.
    #[arg(long, requires = "format", conflicts_with = "file")]
    stdin: bool,
    /// Skip the confirmation prompt and apply the configuration.
    #[arg(long, conflicts_with = "dry_run")]
    no_confirm: bool,
}

#[derive(Debug, Args, Clone, Copy)]
#[group(id = "format", args = ["from_json", "from_yaml"], multiple = false)]
pub struct FromGroup {
    /// Read the configuration from a JSON file
    #[arg(long)]
    from_json: bool,
    /// Read the configuration from a YAML file
    #[arg(long)]
    from_yaml: bool,
}

impl ApplyArgs {
    pub fn run(self) -> Result<()> {
        let configs: Demiurge = if self.from_group.from_json {
            let data = Self::read_static_config(self.stdin, self.file)?;
            serde_json::from_str(&data)?
        } else if self.from_group.from_yaml {
            let data = Self::read_static_config(self.stdin, self.file)?;
            serde_norway::from_str(&data)?
        } else {
            Demiurge::from_file(self.file.expect("Should be required by clap in this case"))?
        };
        let config = configs
            .get(&self.name)
            .context(format!("Configuration \"{}\" not found", self.name))?;
        let applied_config = DemiurgeConfig::read_applied_config();
        let changes = Changes::new(&config, &applied_config);
        if self.dry_run {
            println!("{changes}");
        } else {
            let apply = if self.no_confirm {
                true
            } else {
                inquire::Confirm::new("Do you want to apply the changes?")
                .with_default(false)
                .with_help_message("This will make changes to your system. You can check what changes will be made with the --dry-run flag.")
                .prompt()?
            };
            if apply {
                changes.apply(self.overwrite_symlink)?;
                config.save_applied_config()?;
            } else {
                log::error!("Apply operation canceled. Exiting.");
                std::process::exit(1);
            }
        }

        Ok(())
    }

    fn read_static_config(from_stdin: bool, file: Option<PathBuf>) -> Result<String> {
        if from_stdin {
            let mut buffer = String::new();
            std::io::stdin().lock().read_to_string(&mut buffer)?;
            Ok(buffer)
        } else {
            let path = file.context(
                "A path to the config file should be provided when not reading from stdin.",
            )?;
            let data = std::fs::read_to_string(path)?;
            Ok(data)
        }
    }
}
