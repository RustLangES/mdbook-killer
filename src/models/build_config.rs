use std::path::PathBuf;

use serde::{Deserialize, Serialize};

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
