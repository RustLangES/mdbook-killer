use std::collections::{HashMap, HashSet};
use std::io;
use std::path::PathBuf;

use tokio::fs;

use super::{Summary, SummaryError, SummaryParser};

#[derive(Clone, Debug)]
pub struct SummaryCollection<'a> {
    pub parser: SummaryParser<'a>,
    pub summaries: HashMap<PathBuf, Summary>,
    pub all_files: HashSet<PathBuf>,
}

pub async fn collect_summaries<'a>(
    path: &'a PathBuf,
) -> Result<SummaryCollection<'a>, SummaryError> {
    let mut collection = SummaryCollection {
        parser: SummaryParser::new(&path),
        summaries: HashMap::new(),
        all_files: HashSet::new(),
    };

    explore_dir(&path, &mut collection).await?;

    Ok(collection)
}

async fn explore_dir(
    path: &PathBuf,
    collection: &mut SummaryCollection<'_>,
) -> Result<(), SummaryError> {
    let mut read_dir = match fs::read_dir(&path).await {
        Ok(ok) => ok,
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            return Err(SummaryError::NotFound(path.to_path_buf()))
        }
        Err(err) => return Err(SummaryError::HandledIo(path.to_path_buf(), err)),
    };

    match collection.parser.parse_dir(path).await {
        Ok(summary) => {
            collection.summaries.insert(path.to_path_buf(), summary);
        }
        Err(SummaryError::NotFound(_)) => {}
        Err(err) => return Err(err),
    }

    while let Some(entry) = read_dir.next_entry().await? {
        if entry.file_type().await?.is_dir() {
            let entry_path = entry.path();

            // Prevent infinite sized future due to recursion
            match Box::pin(explore_dir(&entry_path, collection)).await {
                Ok(_) | Err(SummaryError::NotFound(_)) => {}
                Err(err) => return Err(err),
            }
        } else if !entry.file_name().to_string_lossy().contains("SUMMARY") {
            collection.all_files.insert(entry.path());
        }
    }

    Ok(())
}
