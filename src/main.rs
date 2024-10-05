use anyhow::Result;

use mdbook_killer::cli::get_cli;

#[tokio::main]
pub async fn main() -> Result<()> {
    let cli = get_cli();

    cli.commands.execute().await?;
    Ok(())
}
