use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Configuration options which are specific to the book and required for
/// loading it from disk.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct BookConfig {
    /// The book's title.
    pub title: Option<String>,
    /// The book's authors.
    pub authors: Vec<String>,
    /// An optional description for the book.
    pub description: Option<String>,
    /// Location of the book source relative to the book's root directory.
    pub src: PathBuf,
    /// The main language of the book.
    pub language: Option<String>,
    pub languages: Option<Vec<String>>,
    /// The direction of text in the book: Left-to-right (LTR) or Right-to-left (RTL).
    /// When not specified, the text direction is derived from [`BookConfig::language`].
    pub text_direction: Option<TextDirection>,
}

impl Default for BookConfig {
    fn default() -> Self {
        Self {
            title: None,
            authors: Vec::new(),
            description: None,
            src: PathBuf::from("src"),
            language: None,
            languages: None,
            text_direction: None,
        }
    }
}

impl BookConfig {
    /// Gets the realized text direction, either from [`BookConfig::text_direction`]
    /// or derived from [`BookConfig::language`], to be used by templating engines.
    pub fn realized_text_direction(&self) -> TextDirection {
        if let Some(direction) = self.text_direction {
            direction
        } else {
            TextDirection::from_lang_code(self.language.as_deref().unwrap_or_default())
        }
    }
}

/// Text direction to use for HTML output
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextDirection {
    /// Left to right.
    #[serde(rename = "ltr")]
    LeftToRight,
    /// Right to left
    #[serde(rename = "rtl")]
    RightToLeft,
}

impl TextDirection {
    /// Gets the text direction from language code
    pub fn from_lang_code(code: &str) -> Self {
        match code {
            // list sourced from here: https://github.com/abarrak/rtl/blob/master/lib/rtl/core.rb#L16
            "ar" | "ara" | "arc" | "ae" | "ave" | "egy" | "he" | "heb" | "nqo" | "pal" | "phn"
            | "sam" | "syc" | "syr" | "fa" | "per" | "fas" | "ku" | "kur" | "ur" | "urd"
            | "pus" | "ps" | "yi" | "yid" => TextDirection::RightToLeft,
            _ => TextDirection::LeftToRight,
        }
    }
}
