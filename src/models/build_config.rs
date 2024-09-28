use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tokio::fs;

/// Configuration for the build procedure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct BuildConfig {
    /// Where to put built artefacts relative to the book's root directory.
    pub build_dir: PathBuf,
    /// Should non-existent markdown files specified in `SUMMARY.md` be created
    /// if they don't exist?
    pub create_missing: bool,
    /// Should the default preprocessors always be used when they are
    /// compatible with the renderer?
    pub use_default_preprocessors: bool,
    /// Extra directories to trigger rebuild when watching/serving
    pub extra_watch_dirs: Vec<PathBuf>,
}

impl Default for BuildConfig {
    fn default() -> BuildConfig {
        BuildConfig {
            build_dir: PathBuf::from("book"),
            create_missing: true,
            use_default_preprocessors: true,
            extra_watch_dirs: Vec::new(),
        }
    }
}

impl BuildConfig {
    pub async fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let config = fs::read_to_string(path)
            .await
            .map_err(|err| anyhow!("Cannot read {path:?}.\n  Cause: {err}"))?;

        Ok(toml::from_str(&config)?)
    }
}
