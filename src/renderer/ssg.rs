use std::path::Path;
use anyhow::anyhow;
use tokio::fs;

use leptos::{provide_context, IntoView};

use crate::renderer::render::render;

pub struct Ssg<'a> {
    out_dir: &'a Path,
}

impl<'a> Ssg<'a> {
    #[must_use]
    pub fn new(out_dir: &'a Path) -> Self {
        Self { out_dir }
    }

    pub async fn gen<F, V>(
        &'a self,
        path: String,
        view: F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce() -> V + 'static,
        V: IntoView,
    {
        // SsgContext will be available to all components in the view
        let ssg_ctx = SsgContext { path: path.clone() };

        // Render the view to a string
        let res = render(move || view().into_view(), move || provide_context(ssg_ctx)).await;

        // Write the string to a file
        let out_file = self.out_dir.join(path);
        if let Err(error) = fs::write(&out_file, res).await {
            println!("y si, fallo, que esperabas?, {}", error);
            Err(error)?;
        };
        println!("wrote {}", out_file.display());

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SsgContext {
    pub path: String,
}
