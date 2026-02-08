use std::path::PathBuf;

use clap::Args;

use crate::config::Demiurge;

#[derive(Debug, Args, Clone)]
pub struct EvalArgs {
    /// The path to the python file containing the configuration
    #[arg(short, long)]
    file: PathBuf,
}

impl EvalArgs {
    pub fn run(self) {
        let config = Demiurge::from_file(self.file);
        println!("{config:#?}");
    }
}
