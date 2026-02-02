use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let cli = demigurge::cli::Cli::parse();
    cli.run()
}
