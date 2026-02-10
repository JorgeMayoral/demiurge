use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::config::Demiurge;

#[derive(Debug, Args, Clone)]
pub struct EvalArgs {
    /// The path to the typescript file containing the configuration
    #[arg(short, long)]
    file: PathBuf,
    /// Prints the evaluated config in json format
    #[arg(long, conflicts_with = "yaml")]
    json: bool,
    /// Prints the evaluated config in yaml format
    #[arg(long)]
    yaml: bool,
}

impl EvalArgs {
    pub fn run(self) -> Result<()> {
        let config = Demiurge::from_file(self.file)?;
        if self.json {
            let json_string = serde_json::to_string_pretty(&config)?;
            println!("{json_string}");
        } else if self.yaml {
            let yaml_string = serde_norway::to_string(&config)?;
            println!("{yaml_string}");
        } else {
            println!("{config:#?}");
        }

        Ok(())
    }
}
