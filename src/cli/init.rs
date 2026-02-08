use anyhow::Result;
use clap::Args;
use std::{io::Write, path::PathBuf};

#[derive(Debug, Args, Clone)]
pub struct InitArgs {
    /// Path where to initialize the configuration. Defaults to current path.
    #[arg(short, long)]
    path: Option<PathBuf>,
}

const INDEX_TS: &str = include_str!("../initial_files/index.ts");
const INDEX_D_TS: &str = include_str!("../initial_files/index.d.ts");

impl InitArgs {
    pub fn run(self) -> Result<()> {
        let path = self.path.unwrap_or(std::env::current_dir()?);
        let index_ts_file_path = path.join("index.ts");
        let mut index_ts_file = match std::fs::File::create_new(index_ts_file_path) {
            Ok(file) => file,
            Err(_error) => {
                log::error!("The file 'index.ts' already exits.");
                std::process::exit(1);
            }
        };
        index_ts_file.write_all(INDEX_TS.as_bytes())?;

        let index_d_ts_path = path.join("index.d.ts");
        let mut index_d_ts_file = match std::fs::File::create_new(index_d_ts_path) {
            Ok(file) => file,
            Err(_error) => {
                log::error!("The file 'index.d.ts' already exits.");
                std::process::exit(1);
            }
        };
        index_d_ts_file.write_all(INDEX_D_TS.as_bytes())?;

        Ok(())
    }
}
