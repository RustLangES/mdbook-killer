use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::models::config_book::BookConfig;
use crate::models::lang_config::{Language, LanguageConfig};
use crate::models::Config;

use super::Commands;

pub fn execute(_theme: Option<String>, title: Option<String>, root_path: &PathBuf) -> Result<()> {
    log::debug!("Root Path: {root_path:?}");
    let title = title.clone().unwrap_or(get_text(
        "What is the title for this book?",
        "My awesome book",
    )?);
    let description = get_text(
        "What is the description of the book?",
        "My awesome description",
    )?;
    let lang = get_text("What is the default language of this book?", "en")?;

    let config = Config {
        book: BookConfig {
            title: Some(title.clone()),
            authors: vec![],
            description: Some(description.clone()),
            src: "src".into(),
            language: Some(lang.clone()),
            text_direction: None,
        },
        language: Some(LanguageConfig(HashMap::from_iter([(
            lang.clone(),
            Language {
                name: lang.clone(),
                title: Some(title.clone()),
                authors: None,
                description: Some(description.clone()),
            },
        )]))),
        ..Default::default()
    };
    log::trace!("Config generated: {config:?}");

    // Generate struct src folder
    let lang_path = root_path.join("src").join(lang);
    log::trace!("Book Path with Lang: {lang_path:?}");
    std::fs::create_dir_all(&lang_path)?;
    std::fs::File::create(root_path.join("book.toml"))?
        .write_all(toml::to_string_pretty(&config).unwrap().as_bytes());
    std::fs::File::create(lang_path.join("SUMMARY.md"))?.write_all(
        format!("---\nog_title: {title}\nog_description: {description}\n---",).as_bytes(),
    )?;
    std::fs::File::create(lang_path.join("Readme.md"))?
        .write_all(b"# This is a index Example\nWith description")?;
    Ok(())
}

fn get_text(s: &str, d: &str) -> Result<String> {
    inquire::Text::new(s)
        .with_default(d)
        .with_validator(|s: &str| {
            if s.chars().count() >= 2 {
                Ok(inquire::validator::Validation::Valid)
            } else {
                Ok(inquire::validator::Validation::Invalid(
                    "You're need 2 or more characters.".into(),
                ))
            }
        })
        .prompt()
        .context("Cannot get value from prompt")
}
