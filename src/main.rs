use clap::Parser;

fn main() {
    let cli = demigurge::cli::Cli::parse();
    println!("{cli:?}");
}
