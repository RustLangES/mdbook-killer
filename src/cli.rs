use anyhow::Result;
use clap::Parser;
use inquire::Confirm;

use crate::command::Commands;

// Parse commands and arguments from the CLI
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// If verbose is set, print extra messages while running.
    #[arg(short, long)]
    verbose: Option<bool>,

    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn is_verbose_mode(&self) -> bool {
        match self.verbose {
            Some(verbose)  => verbose,
            None => {
                Confirm::new("Verbosity?")
                    .with_default(true)
                    .with_help_message("If set, extra statements will print while running")
                    .prompt()
                    .unwrap()
            },
        }
    }

    pub fn execute(&self) -> Result<()> {
        let verbose = self.is_verbose_mode();

        self.command.execute(verbose)

    }
}
