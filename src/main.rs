use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    colog::init();
    let cli = demigurge::cli::Cli::parse();
    cli.run()
}
