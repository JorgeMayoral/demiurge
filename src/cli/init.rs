use anyhow::{Context, Result};
use clap::Args;
use std::{io::Write, path::PathBuf};

#[derive(Debug, Args, Clone)]
pub struct InitArgs {
    /// Path where to initialize the configuration. Defaults to current path.
    #[arg(short, long)]
    path: Option<PathBuf>,
    /// Overwrite initial files if they exists. Create them if not.
    #[arg(long, conflicts_with = "update_types")]
    overwrite: bool,
    /// Update the Typescript definition files to the latest version
    #[arg(long)]
    update_types: bool,
}

const INDEX_TS: &str = include_str!("../initial_files/index.ts");
const INDEX_D_TS: &str = include_str!("../initial_files/index.d.ts");

impl InitArgs {
    pub fn run(self) -> Result<()> {
        if !self.update_types {
            self.create_initial_config_file()
                .context("create initial config file")?;
        }
        self.create_config_types_file()
            .context("create config types file")?;
        Ok(())
    }

    fn get_file_path(&self, filename: &str) -> Result<PathBuf> {
        let path = self
            .path
            .clone()
            .unwrap_or(std::env::current_dir().context("get current directory")?);
        let file_path = path.join(filename);
        Ok(file_path)
    }

    fn create_initial_config_file(&self) -> Result<()> {
        let index_ts_file_path = self
            .get_file_path("index.ts")
            .context("resolve index.ts path")?;
        let mut index_ts_file = if self.overwrite {
            std::fs::File::create(index_ts_file_path).context("create index.ts")?
        } else {
            std::fs::File::create_new(index_ts_file_path)
                .context("create index.ts (file must not already exist)")?
        };
        index_ts_file
            .write_all(INDEX_TS.as_bytes())
            .context("write index.ts")?;
        Ok(())
    }

    fn create_config_types_file(&self) -> Result<()> {
        let index_d_ts_path = self
            .get_file_path("index.d.ts")
            .context("resolve index.d.ts path")?;
        let mut index_d_ts_file = if self.overwrite || self.update_types {
            std::fs::File::create(index_d_ts_path).context("create index.d.ts")?
        } else {
            std::fs::File::create_new(index_d_ts_path)
                .context("create index.d.ts (file must not already exist)")?
        };
        index_d_ts_file
            .write_all(INDEX_D_TS.as_bytes())
            .context("write index.d.ts")?;
        Ok(())
    }
}
