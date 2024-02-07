use std::{fs::{self, DirEntry}, io::Read, path::PathBuf};

use anyhow::Result;
use clap::Args;

use crate::models::config_book::ConfigBook;



#[derive(Debug, Args)]
pub struct ServeCommand {
    #[arg(short, long, default_value_t=3000)]
    port: u32,
    #[arg(short, long)]
    language: Option<String>,

    #[arg(skip)]
    custom_config: Option<ConfigBook>
}

impl ServeCommand {
    pub fn execute(&mut self) -> Result<()> {

        let content = fs::read_to_string("Book.toml")?;

        let custom_config = toml::from_str::<ConfigBook>(&content)?;

        self.custom_config = Some(custom_config);
            
        let source_dir = fs::read_dir(format!("./src/{}", self.has_prefer_language().unwrap_or_default()))?;

        Ok(())
    }

    fn has_prefer_language(&self) -> Option<String> {
        self.language.clone()
        .or(
            self.custom_config.clone().and_then(
                |ConfigBook { languages, .. }| 
                languages.first().cloned()
            )
        )
        .clone()
    }
}