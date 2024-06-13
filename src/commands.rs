use std::path::PathBuf;

use anyhow::Result;
use clap::{CommandFactory, Subcommand, ValueHint};
use clap_complete::{generate_to, Shell};

use crate::cli::Cli;

mod clean;
mod init;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Creates the boilerplate structure and files for a new book
    Init {
        /// Copies the default theme into your source folder
        #[clap(long, short)]
        theme: Option<String>,
        /// Sets the book title
        #[clap(long)]
        title: Option<String>,
        /// Directory to create the book in
        #[clap(value_hint = ValueHint::AnyPath)]
        dir: PathBuf,
    },
    /// Builds a book from its markdown files
    Build {
        /// Opens the compiled book in a web browser
        #[clap(long, short)]
        open: bool,
        /// Output directory for the book\n\
        /// Relative paths are interpreted relative to the book's root directory.\n\
        /// If omitted, mdBook uses build.build-dir from book.toml \
        /// or defaults to `./book`.
        #[clap(long, short, value_hint = ValueHint::DirPath)]
        dest_dir: Option<PathBuf>,
        /// Root directory for the book
        #[clap(value_hint = ValueHint::DirPath)]
        dir: PathBuf,
    },
    /// Deletes a built book
    Clean {
        /// Root directory for the book
        #[clap(value_hint = ValueHint::DirPath)]
        dir: PathBuf,
        /// Relative paths are interpreted relative to the book's root directory.\n\
        /// If omitted, mdBook uses build.build-dir from book.toml \
        /// or defaults to `./book`.
        #[clap(long, short, value_hint = ValueHint::DirPath)]
        dest_dir: Option<PathBuf>,
    },
    /// The completions command is used to generate auto-completions for some common shells
    Completions {
        #[clap(value_enum)]
        shell: Shell,
        /// Output directory for the generations
        #[clap(value_hint = ValueHint::DirPath)]
        out_dir: PathBuf,
    },
    /// Watches a book's files and rebuilds it on changes
    Watch {
        /// Opens the compiled book in a web browser
        #[clap(long, short)]
        open: bool,
        /// Output directory for the book\n\
        /// Relative paths are interpreted relative to the book's root directory.\n\
        /// If omitted, mdBook uses build.build-dir from book.toml \
        /// or defaults to `./book`.
        #[clap(long, short, value_hint = ValueHint::DirPath)]
        dest_dir: Option<PathBuf>,
        /// Root directory for the book
        #[clap(value_hint = ValueHint::AnyPath)]
        dir: PathBuf,
    },
    /// Serves a book at http://localhost:3000, and rebuilds it on changes
    Serve {
        #[clap(long, short)]
        open: bool,
        /// Port to use for HTTP connections
        #[clap(long, short, default_value = "3000")]
        port: Option<u16>,
        /// Output directory for the book\n\
        /// Relative paths are interpreted relative to the book's root directory.\n\
        /// If omitted, mdBook uses build.build-dir from book.toml \
        /// or defaults to `./book`.
        #[clap(long, short, value_hint = ValueHint::DirPath)]
        dest_dir: Option<PathBuf>,
        /// Hostname to listen on for HTTP connections
        #[clap(long, short = 'n', default_value = "0.0.0.0", value_hint = ValueHint::Hostname)]
        hostname: Option<String>,
        /// Root directory for the book
        #[clap(value_hint = ValueHint::DirPath)]
        dir: PathBuf,
    },
    /// Tests that a book's Rust code samples compile
    Test {
        #[clap(long, short)]
        open: bool,
        #[clap(long, short)]
        chapter: Option<String>,
        /// A comma-separated list of directories to add to the crate search path when building tests
        #[clap(long, short, value_hint = ValueHint::FilePath, value_delimiter = ',')]
        library_path: Vec<PathBuf>,
        /// Root directory for the book
        #[clap(value_hint = ValueHint::DirPath)]
        dir: PathBuf,
    },
}

impl Commands {
    pub fn execute(&self) -> Result<()> {
        match self {
            Commands::Completions { shell, out_dir } => {
                let mut cmd = Cli::command_for_update();
                let name = cmd.get_name().to_string();
                generate_to(*shell, &mut cmd, name, out_dir).unwrap();
            }
            Commands::Clean { dir, dest_dir } => {
                clean::execute(dir.clone(), dest_dir.clone())?;
            }
            Commands::Init { theme, title, dir } => {
                init::execute(theme.clone(), title.clone(), dir)?
            }
            Commands::Build {
                open,
                dest_dir,
                dir,
            } => {}
            Commands::Watch {
                open,
                dest_dir,
                dir,
            } => {}
            Commands::Serve {
                open,
                port,
                dest_dir,
                hostname,
                dir,
            } => {}
            Commands::Test {
                open,
                chapter,
                library_path,
                dir,
            } => {}
        }

        Ok(())
    }
}
