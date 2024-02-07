use std::{env, fs, path::PathBuf};
use anyhow::Result;
use clap::Args;
use git2::Repository;
use inquire::{min_length, required, Text};
use crate::{models::config_book::ConfigBook, utils::to_snake_case::ToSnakeCase};

#[derive(Debug, Args)]
pub struct CreateCommand {
    #[arg(short, long)]
    path: Option<PathBuf>,

    #[arg(short, long)]
    title: Option<String>,
}

impl CreateCommand {
    pub fn execute(&self) -> Result<()> {
        let (title, path) = self.get_from_prompt();
        let repository_to_clone = env::var("TEMPLATE_REPOSITORY")?;
        Repository::clone(&repository_to_clone, path.clone())?;
        let config = ConfigBook::new(title);

        let config_path = path.join("Book.toml");

        fs::write(config_path, toml::to_string(&config)?)?;

        Ok(())
    }

    pub fn get_from_prompt(&self) -> (String, PathBuf) {
        let title = match self.title.clone() {
            Some(title) => title,
            None => {
                Text::new("Provide the title of the new book")
                    .with_default("my_new_book")
                    .with_validator(min_length!(4))
                    .with_validator(required!("This is field is required"))
                    .with_help_message("This is the title of the book, later you can modify it.")
                    .prompt()
                    .expect("You must to provide a title")
            }
        };

        let path = match self.path.clone() {
            Some(path) => path,
            None => {
                PathBuf::from(
                    Text::new("Provide the path to start the new book")
                    .with_default(&title.to_snake_case())
                    .with_help_message("The path relative")
                    .with_validator(required!("This is field is required"))
                    .prompt()
                    .expect("The path is required")
                )
            }
        };

        (title, path)
    }
}