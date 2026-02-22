use anyhow::Result;
use clap::Parser;

use crate::logging::DemiurgeLog;

mod changes;
mod cli;
mod config;
mod logging;
mod utils;

fn main() -> Result<()> {
    let cli = crate::cli::Cli::parse();
    DemiurgeLog::init(cli.verbosity());
    cli.run()
}
