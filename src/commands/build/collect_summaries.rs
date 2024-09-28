use std::io;
use std::path::PathBuf;

use tokio::fs;

use super::{Summary, SummaryError};

pub async fn collect_summaries(path: PathBuf) -> Result<Vec<Summary>, SummaryError> {
    let mut collected_summaries = Vec::new();

    explore_dir(path, &mut collected_summaries).await?;

    Ok(collected_summaries)
}

async fn explore_dir(
    path: PathBuf,
    collected_summaries: &mut Vec<Summary>,
) -> Result<(), SummaryError> {
    let mut read_dir = match fs::read_dir(&path).await {
        Ok(ok) => ok,
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            return Err(SummaryError::NotFound(path))
        }
        Err(err) => return Err(SummaryError::HandledIo(path, err)),
    };

    match Summary::from_path(path.join("SUMMARY.md")).await {
        Ok(summary) => collected_summaries.push(summary),
        Err(SummaryError::NotFound(_)) => {}
        Err(err) => return Err(err),
    }

    while let Some(entry) = read_dir.next_entry().await? {
        if entry.file_type().await?.is_dir() {
            let entry_path = entry.path();

            // Prevent infinite sized future due to recursion
            match Box::pin(explore_dir(entry_path, collected_summaries)).await {
                Ok(_) | Err(SummaryError::NotFound(_)) => {}
                Err(err) => return Err(err),
            }
        }
    }

    Ok(())
}
