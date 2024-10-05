mod collect_summaries;
use collect_summaries::collect_summaries;
mod summary;
pub(super) use summary::{Summary, SummaryError, SummaryParser};
mod to_html;
use to_html::to_html;

use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::{Parser, ValueHint};
use tokio::fs;
use tokio::io::AsyncReadExt;

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

    let _build_dir = build_dir.canonicalize()?;

    let src_dir = dir.join("src");

    let collection = collect_summaries(&src_dir).await?;

    if collection.summaries.is_empty() {
        eprintln!("No \"SUMMARY.md\", there should be at least one");
        return Ok(());
    }

    println!("Langs: {}", collection.summaries.len());

    println!("\nChecking files not linked...\n");

    let used_files = &collection.parser.all_files;
    let all_files = &collection.all_files;

    for unused_file in all_files.difference(used_files) {
        let relative_path = unused_file.strip_prefix(&dir)?;
        log::warn!("- {}", relative_path.display());
    }

    for file in used_files {
        let relative_path = file.strip_prefix(&src_dir)?;
        println!("Compiling /{}...", relative_path.display());

        let Some(summary) = get_summary(file, &collection.summaries) else {
            log::error!("No summary for this file");
            continue;
        };

        println!("Using summary: {}", summary.dir.display());

        let file_parsed = {
            let dir = file.parent().expect("Cannot get dirname");
            let file_name = file.file_stem().expect("Cannot get filename");
            let file_base = dir.join(file_name);

            let file_md = fs::File::open(file_base.with_extension("md"));
            let file_mdx = fs::File::open(file_base.with_extension("mdx"));

            let (file_md, file_mdx) = tokio::join!(file_md, file_mdx);

            let file_md = map_not_found_file(file_md)?;
            let file_mdx = map_not_found_file(file_mdx)?;

            let mut file_content = String::new();

            if let Some(mut file_md) = file_md {
                if file_mdx.is_some() {
                    return Err(anyhow!("The entry {} is duplicated", file_base.display()));
                }

                file_md.read_to_string(&mut file_content).await?;
                markdown::to_mdast(
                    &file_content,
                    &markdown::ParseOptions {
                        constructs: markdown::Constructs {
                            frontmatter: true,
                            ..markdown::Constructs::gfm()
                        },
                        ..markdown::ParseOptions::gfm()
                    },
                )
                .map_err(|err| anyhow!("{err}"))?
            } else if let Some(mut file_mdx) = file_mdx {
                file_mdx.read_to_string(&mut file_content).await?;
                markdown::to_mdast(
                    &file_content,
                    &markdown::ParseOptions {
                        constructs: markdown::Constructs {
                            frontmatter: true,
                            ..markdown::Constructs::mdx()
                        },
                        ..Default::default()
                    },
                )
                .map_err(|err| anyhow!("{err}"))?
            } else {
                return Err(anyhow!("No file for {}", file_base.display()));
            }
        };

        // println!("{file_parsed:#?}");

        let html = to_html(file_parsed);

        // println!("{html}");
    }

    Ok(())
}

fn get_summary<'a>(
    file: &'a PathBuf,
    summaries: &'a HashMap<PathBuf, Summary>,
) -> Option<&'a Summary> {
    let mut best_summary = (usize::MAX, None);

    for (summary_path, summary) in summaries {
        let stripped_len = file
            .strip_prefix(summary_path)
            .ok()
            .map(|f| f.to_string_lossy().len());

        if let Some(stripped_len) = stripped_len {
            if stripped_len < best_summary.0 {
                best_summary = (stripped_len, Some(summary));
            }
        }
    }

    best_summary.1
}

fn map_not_found_file(file: Result<fs::File, io::Error>) -> Result<Option<fs::File>, io::Error> {
    match file {
        Ok(f) => Ok(Some(f)),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err),
    }
}
