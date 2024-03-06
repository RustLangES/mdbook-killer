use anyhow::Result;

use mdbook_killer::cli::get_cli;

pub fn main() -> Result<()> {
    let cli = get_cli();

    println!("{cli:?}");

    // cli.execute()?;
    Ok(())
}

