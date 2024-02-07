use anyhow::Result;
use clap::Subcommand;

use crate::commands::{create::CreateCommand, serve::ServeCommand};


#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a structure (example command)
    Create(CreateCommand),
    Serve(ServeCommand),
    /// Delete the structure (example command)
    Delete { },
}


impl Commands {
    pub fn execute(&self, _verbose: bool) -> Result<()> {
        match self {
            Commands::Create(create) => create.execute()?,
            _ => unimplemented!()
        };

        Ok(())
    }
}
