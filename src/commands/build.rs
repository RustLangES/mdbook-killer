mod summary;

use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::{Parser, ValueHint};
use summary::Summary;
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

pub async fn execute(CommandBuild { out_dir, dir, open }: &CommandBuild) -> Result<()> {
    let dir = dir
        .as_ref()
        .unwrap_or(&PathBuf::from_str(".").unwrap())
        .canonicalize()?;

    let config = dir.join("book.toml");
    let config = fs::read_to_string(&config)
        .await
        .map_err(|err| anyhow!("Cannot read {config:?}.\n  Cause: {err}"))?;
    let config: BuildConfig = toml::from_str(&config)?;

    let build_dir = out_dir.as_ref().unwrap_or(&config.build_dir);

    match fs::create_dir(&build_dir).await {
        Ok(_) => {},
        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {},
        Err(err) => return Err(err.into())
    };

    let build_dir = build_dir.canonicalize()?;

    let src_dir = dir.join("src");

    let mut warned_by_files_in_src = false;
    let mut langs = fs::read_dir(&src_dir)
        .await
        .map_err(|err| anyhow!("Cannot read {src_dir:?}.\n  Cause: {err}"))?;

    while let Some(lang) = langs.next_entry().await? {
        let file_name = lang.file_name().to_string_lossy().to_string();

        if !lang.file_type().await?.is_dir() {
            if !warned_by_files_in_src {
                warned_by_files_in_src = true;

                log::warn!("`src` should contain only folders with languages");
            }

            log::warn!("`src/{file_name}` is a file, should be a folder. Ignoring it");
        }

        let lang_path = lang.path();

        let summary = lang_path.join("SUMMARY.md");
        let summary = Summary::from_path(summary).await?;

        println!("{summary:#?}");
    }

    Ok(())
}
