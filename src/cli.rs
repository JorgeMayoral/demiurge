use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use crate::cli::{apply::ApplyArgs, eval::EvalArgs, init::InitArgs, schema::SchemaArgs};

mod apply;
mod eval;
mod init;
mod schema;

#[derive(Debug, Parser, Clone)]
#[command(version, about, author)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
    #[command(flatten)]
    verbosity: Verbosity<InfoLevel>,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    /// Prints resulting Rust struct after evaluating Typescript config
    Eval(EvalArgs),
    /// Applies the configuration
    Apply(ApplyArgs),
    /// Creates the initial configuration files
    Init(InitArgs),
    /// Prints the JSON schema of the Demiurge configuration object
    Schema(SchemaArgs),
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        match self.command.clone() {
            Command::Eval(args) => {
                args.run()?;
            }
            Command::Apply(args) => {
                args.run()?;
            }
            Command::Init(args) => {
                args.run()?;
            }
            Command::Schema(args) => {
                args.run()?;
            }
        }

        Ok(())
    }

    pub fn verbosity(&self) -> Verbosity<InfoLevel> {
        self.verbosity
    }
}

mod tests {

    #[test]
    fn apply_valid_flags() {
        use crate::cli::Cli;
        use clap::Parser;
        let valid_inputs = vec![
            vec!["--", "apply", "-n", "test", "--from-json", "--stdin"],
            vec!["--", "apply", "-n", "test", "--from-yaml", "--stdin"],
            vec!["--", "apply", "-f", "./some/config/path.ts", "-n", "test"],
            vec![
                "--",
                "apply",
                "-f",
                "./some/config/path.ts",
                "-n",
                "test",
                "-d",
            ],
        ];
        for input in valid_inputs {
            let cli = Cli::try_parse_from(&input);
            assert!(cli.is_ok(), "Expected Ok, got Err:\n{}", cli.unwrap_err());
        }
    }

    #[test]
    fn apply_invalid_flags() {
        use crate::cli::Cli;
        use clap::Parser;
        let invalid_inputs = vec![
            vec!["--", "apply", "-n", "test", "--stdin"],
            vec![
                "--",
                "apply",
                "-n",
                "test",
                "--from-yaml",
                "--from-json",
                "--stdin",
            ],
        ];
        for input in invalid_inputs {
            let cli = Cli::try_parse_from(&input);
            assert!(cli.is_err(), "Expected Err, got Ok: {:?}", cli.unwrap());
        }
    }

    #[test]
    fn init_valid_flags() {
        use crate::cli::Cli;
        use clap::Parser;
        let valid_inputs = vec![
            vec!["--", "init"],
            vec!["--", "init", "--overwrite"],
            vec!["--", "init", "--update-types"],
        ];
        for input in valid_inputs {
            let cli = Cli::try_parse_from(&input);
            assert!(cli.is_ok(), "Expected Ok, got Err:\n{}", cli.unwrap_err());
        }
    }

    #[test]
    fn init_overwrite_and_update_types_are_mutually_exclusive() {
        use crate::cli::Cli;
        use clap::Parser;
        let cli = Cli::try_parse_from(["--", "init", "--overwrite", "--update-types"]);
        assert!(
            cli.is_err(),
            "Expected Err for mutually exclusive flags, got Ok: {:?}",
            cli.unwrap()
        );
    }
}
