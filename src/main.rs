use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let cli = demiurge::cli::Cli::parse();
    cli.run()
}
