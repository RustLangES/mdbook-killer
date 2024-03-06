use clap::Parser;
use clap_verbosity_flag::Verbosity;

use crate::commands::Commands;

// Parse commands and arguments from the CLI
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(flatten)]
    verbose: Verbosity,

    #[clap(subcommand)]
    pub commands: Commands,
}

pub fn get_cli() -> Cli {
    let cli = Cli::parse();
    let log_filter = cli.verbose.log_level_filter();

    // initialize logger
    env_logger::Builder::new()
        .default_format()
        .parse_default_env()
        .filter_level(log_filter)
        .init();

    log::info!("Initialized Logger with Level: {log_filter:?}");

    cli
}
