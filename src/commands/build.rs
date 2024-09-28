mod collect_summaries;
use collect_summaries::collect_summaries;
mod summary;
pub(super) use summary::{Summary, SummaryError};

use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::{Parser, ValueHint};
use tokio::fs;

use crate::models::build_config::BuildConfig;

#[derive(Clone, Debug, Parser)]
pub struct CommandBuild {
    /// Opens the compiled book in a web browser
    #[clap(long)]
    open: bool,
    /// Output directory for the book
    /// Relative paths are interpreted relative to the book's root directory.
    /// If omitted, mdBook uses build.build-dir from book.toml
    /// or defaults to `./book`.
    #[clap(long, short, value_hint = ValueHint::DirPath)]
    out_dir: Option<PathBuf>,
    /// Root directory for the book
    #[clap(value_hint = ValueHint::DirPath)]
    dir: Option<PathBuf>,
}

pub async fn execute(
    CommandBuild {
        out_dir,
        dir,
        open: _,
    }: &CommandBuild,
) -> Result<()> {
    let dir = dir
        .as_ref()
        .unwrap_or(&PathBuf::from_str(".").unwrap())
        .canonicalize()?;

    assert!(std::env::set_current_dir(&dir).is_ok());

    let config = BuildConfig::from_path(dir.join("book.toml")).await?;

    let build_dir = out_dir.as_ref().unwrap_or(&config.build_dir);

    match fs::create_dir(&build_dir).await {
        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {}
        err @ Ok(_) | err @ Err(_) => err?,
    };

    let build_dir = build_dir.canonicalize()?;

    let src_dir = dir.join("src");

    let collected_summaries = collect_summaries(src_dir).await?;

    if collected_summaries.is_empty() {
        eprintln!("No \"SUMMARY.md\", there should be at least one");
        return Ok(());
    }

    println!("Langs: {}", collected_summaries.len());

    Ok(())
}
