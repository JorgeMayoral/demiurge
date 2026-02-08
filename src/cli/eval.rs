use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::config::Demiurge;

#[derive(Debug, Args, Clone)]
pub struct EvalArgs {
    /// The path to the python file containing the configuration
    #[arg(short, long)]
    file: PathBuf,
    /// Prints the evaluated config in json format
    #[arg(long)]
    json: bool,
}

impl EvalArgs {
    pub fn run(self) -> Result<()> {
        let config = Demiurge::from_file(self.file)?;
        if self.json {
            let json_value = serde_json::to_value(config)?;
            let json_string = json_value.to_string();
            println!("{json_string}");
        } else {
            println!("{config:#?}");
        }

        Ok(())
    }
}
