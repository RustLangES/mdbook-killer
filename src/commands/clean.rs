use anyhow::{Context, Error};

use crate::models::Config;
use std::path::PathBuf;

pub fn execute(dir: PathBuf, dest_dir: Option<PathBuf>) -> Result<(), Error> {
    let config = Config::from_disk("./book.toml")?;
    let dir_to_remove = match dest_dir {
        Some(dest_dir) => dest_dir.into(),
        None => match config.build.as_ref().map(|b| b.build_dir.clone()) {
            Some(build_dir) => config.book.src.join(&build_dir),
            None => config.book.src.join(&dir),
        },
    };

    if dir_to_remove.exists() {
        std::fs::remove_dir_all(&dir_to_remove)
            .with_context(|| "Unable to remove the build directory")?;
    }
    Ok(())
}
