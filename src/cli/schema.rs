use std::{io::Write, path::PathBuf};

use anyhow::{Context, Result};
use clap::Args;

use crate::{config::Demiurge, utils};

#[derive(Debug, Args, Clone)]
pub struct SchemaArgs {
    /// Path where to save the JSON schema
    #[arg(short, long)]
    output: Option<PathBuf>,
}

impl SchemaArgs {
    pub fn run(self) -> Result<()> {
        let schema = schemars::schema_for!(Demiurge);
        let json_string =
            serde_json::to_string_pretty(&schema).context("serialize schema to JSON")?;
        if let Some(path) = self.output {
            let expanded_path = utils::path_tilde_expand(path)
                .context("expand schema output path")?
                .canonicalize()
                .context("canonicalize schema output path")?;
            let full_path = expanded_path.join("schema.json");
            let mut file =
                std::fs::File::create_new(full_path).context("create schema.json output file")?;
            file.write_all(json_string.as_bytes())
                .context("write schema.json")?;
        } else {
            println!("{json_string}");
        }
        Ok(())
    }
}
