use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    sudo::escalate_if_needed().unwrap();
    let cli = demigurge::cli::Cli::parse();
    cli.run()
}
