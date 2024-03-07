use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Configuration for localizations of this book
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LanguageConfig(pub HashMap<String, Language>);

/// Configuration for a single localization
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Language {
    /// Human-readable name of the language.
    pub name: String,
    /// Localized title of the book.
    pub title: Option<String>,
    /// The authors of the translation.
    pub authors: Option<Vec<String>>,
    /// Localized description of the book.
    pub description: Option<String>,
}
