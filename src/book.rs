use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::models::Config;

/// A dumb tree structure representing a book.
///
/// For the moment a book is just a collection of [`BookItems`] which are
/// accessible by either iterating (immutably) over the book with [`iter()`], or
/// recursively applying a closure to each section to mutate the chapters, using
/// [`for_each_mut()`].
///
/// [`iter()`]: #method.iter
/// [`for_each_mut()`]: #method.for_each_mut
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Book {
    /// The sections in this book.
    pub sections: Vec<BookItem>,
    /// Chapter title overrides for this book.
    #[serde(default)]
    pub chapter_titles: HashMap<PathBuf, String>,
}

/// Enum representing any type of item which can be added to a book.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BookItem {
    /// A nested chapter.
    Chapter(Chapter),
    /// A section separator.
    Separator,
    /// A part title.
    PartTitle(String),
}

impl From<Chapter> for BookItem {
    fn from(other: Chapter) -> BookItem {
        BookItem::Chapter(other)
    }
}

impl Book {
    pub fn new(config: &Config, lang: &str) -> Self {
    }
}

/// The representation of a "chapter", usually mapping to a single file on
/// disk however it may contain multiple sub-chapters.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Chapter {
    /// The chapter's name.
    pub name: String,
    /// The chapter's contents.
    pub content: String,
    /// The chapter's section number, if it has one.
    pub number: Option<SectionNumber>,
    /// Nested items.
    pub sub_items: Vec<BookItem>,
    /// The chapter's location, relative to the `SUMMARY.md` file.
    pub path: Option<PathBuf>,
    /// The chapter's source file, relative to the `SUMMARY.md` file.
    pub source_path: Option<PathBuf>,
    /// An ordered list of the names of each chapter above this one in the hierarchy.
    pub parent_names: Vec<String>,
}

impl Chapter {
    /// Create a new chapter with the provided content.
    pub fn new<P: Into<PathBuf>>(
        name: &str,
        content: String,
        p: P,
        parent_names: Vec<String>,
    ) -> Chapter {
        let path: PathBuf = p.into();
        Chapter {
            name: name.to_string(),
            content,
            path: Some(path.clone()),
            source_path: Some(path),
            parent_names,
            ..Default::default()
        }
    }

    /// Create a new draft chapter that is not attached to a source markdown file (and this
    /// has no content).
    pub fn new_draft(name: &str, parent_names: Vec<String>) -> Self {
        Chapter {
            name: name.to_string(),
            content: String::new(),
            path: None,
            source_path: None,
            parent_names,
            ..Default::default()
        }
    }

    /// Check if the chapter is a draft chapter, meaning it has no path to a source markdown file.
    pub fn is_draft_chapter(&self) -> bool {
        self.path.is_none()
    }
}
