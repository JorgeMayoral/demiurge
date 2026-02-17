use anyhow::Result;
use clap::Parser;

mod changes;
mod cli;
mod config;
mod utils;

fn main() -> Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let cli = crate::cli::Cli::parse();
    cli.run()
}
