use anyhow::Result;

use clap::Parser;

pub mod cli;
pub mod command;
pub mod commands;
pub mod models;
pub mod utils;
use cli::Cli;

pub fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let cli = Cli::parse();

    println!("{cli:?}");

    cli.execute()?;
    Ok(())
}