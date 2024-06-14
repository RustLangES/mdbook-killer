use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use log::info;
use serde::{Deserialize, Serialize};

use self::build_config::BuildConfig;
use self::config_book::BookConfig;
use self::lang_config::LanguageConfig;
use self::preprocessors_config::{HtmlPreprocessor, PreprocessorsConfig};
use self::rust_config::RustConfig;

pub mod build_config;
mod chapter;
pub mod config_book;
pub mod lang_config;
pub mod preprocessors_config;
pub mod rust_config;

pub use chapter::Chapter;

/// The overall configuration object for MDBook, essentially an in-memory
/// representation of `book.toml`.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// Metadata about the book.
    pub book: BookConfig,
    /// Information about the build environment.
    pub build: Option<BuildConfig>,
    /// Information about Rust language support.
    pub rust: RustConfig,
    /// Information about localizations of this book.
    pub language: Option<LanguageConfig>,
    pub output: Option<PreprocessorsConfig>,
}

impl Config {
    /// Load the configuration file from disk.
    pub fn from_disk<P: AsRef<Path>>(config_file: P) -> Result<Config> {
        let mut buffer = String::new();
        File::open(config_file)
            .with_context(|| "Unable to open the configuration file")?
            .read_to_string(&mut buffer)
            .with_context(|| "Couldn't read the file")?;

        toml::from_str(&buffer).with_context(|| "Invalid configuration file")
    }

    /// Convenience method for getting the html renderer's configuration.
    ///
    /// # Note
    ///
    /// This is for compatibility only. It will be removed completely once the
    /// HTML renderer is refactored to be less coupled to `mdbook` internals.
    #[doc(hidden)]
    pub fn html_config(&self) -> HtmlPreprocessor {
        self.output.clone().unwrap_or_default().html
    }

    /// Gets the language configured for a book.
    pub fn get_language<I: AsRef<str>>(&self, index: Option<I>) -> Result<Option<String>> {
        match self.default_language() {
            // Languages have been specified, assume directory structure with
            // language subfolders.
            Some(ref default) => match index {
                // Make sure that the language we passed was actually declared
                // in the config, and return an `Err` if not.
                Some(lang_ident) => match self
                    .language
                    .clone()
                    .unwrap_or_default()
                    .0
                    .get(lang_ident.as_ref())
                {
                    Some(_) => Ok(Some(lang_ident.as_ref().into())),
                    None => Err(anyhow!(
                        "Expected [language.{}] to be declared in book.toml",
                        lang_ident.as_ref()
                    )),
                },
                // Use the default specified in book.toml.
                None => Ok(Some(default.to_string())),
            },

            // No [language] table was declared in book.toml.
            None => match index {
                // We passed in a language from the frontend, but the config
                // offers no languages.
                Some(lang_ident) => Err(anyhow!(
                    "No [language] table in book.toml, expected [language.{}] to be declared",
                    lang_ident.as_ref()
                )),
                // Default to previous non-localized behavior.
                None => Ok(None),
            },
        }
    }

    /// Get the source directory of a localized book corresponding to language ident `index`.
    pub fn get_localized_src_path<I: AsRef<str>>(&self, index: Option<I>) -> Result<PathBuf> {
        let language = self.get_language(index)?;

        match language {
            Some(lang_ident) => {
                let mut buf = PathBuf::new();
                buf.push(self.book.src.clone());
                buf.push(lang_ident);
                Ok(buf)
            }

            // No [language] table was declared in book.toml. Preserve backwards
            // compatibility by just returning `src`.
            None => Ok(self.book.src.clone()),
        }
    }

    /// Gets the localized title of the book.
    pub fn get_localized_title<I: AsRef<str>>(&self, index: Option<I>) -> Option<String> {
        let language = self.get_language(index).unwrap();

        match language {
            Some(lang_ident) => self
                .language
                .clone()
                .unwrap_or_default()
                .0
                .get(&lang_ident)
                .unwrap()
                .title
                .clone()
                .or(self.book.title.clone()),
            None => self.book.title.clone(),
        }
    }

    /// Gets the localized description of the book.
    pub fn get_localized_description<I: AsRef<str>>(&self, index: Option<I>) -> Option<String> {
        let language = self.get_language(index).unwrap();

        match language {
            Some(lang_ident) => self
                .language
                .clone()
                .unwrap_or_default()
                .0
                .get(&lang_ident)
                .unwrap()
                .description
                .clone()
                .or(self.book.description.clone()),
            None => self.book.description.clone(),
        }
    }

    /// Get the fallback source directory of a book. If chapters/sections are
    /// missing in a localization, any links to them will gracefully degrade to
    /// the files that exist in this directory.
    pub fn get_fallback_src_path(&self) -> PathBuf {
        match self.default_language() {
            // Languages have been specified, assume directory structure with
            // language subfolders.
            Some(default) => {
                let mut buf = PathBuf::new();
                buf.push(self.book.src.clone());
                buf.push(default);
                buf
            }

            // No default language was configured in book.toml. Preserve
            // backwards compatibility by just returning `src`.
            None => self.book.src.clone(),
        }
    }

    /// If true, mdBook should assume there are subdirectories under src/
    /// corresponding to the localizations in the config. If false, src/ is a
    /// single directory containing the summary file and the rest.
    pub fn has_localized_dir_structure(&self) -> bool {
        if self.language.is_some() {
            !self.language.clone().unwrap_or_default().0.is_empty()
        } else {
            self.book.languages.is_some()
        }
    }

    /// Obtains the default language for this config.
    pub fn default_language(&self) -> Option<String> {
        if self.has_localized_dir_structure() {
            let Some(language_ident) = self.book.language.clone() else {
                info!("Config has [language] table, but `book.language` not was declared");
                return None;
            };

            let _ = self
                .language
                .clone()
                .unwrap_or_default()
                .0
                .get(&language_ident)
                .with_context(|| {
                    format!(
                        "Expected [language.{}] to be declared in book.toml",
                        language_ident
                    )
                });
            Some(language_ident)
        } else if self.book.languages.is_some() {
            let languages = self.book.languages.clone();
            let languages = languages.unwrap();
            languages.first().cloned()
        } else {
            None
        }
    }
}
