use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Chapter {
    pub title: String,
    pub content: Option<String>,
    pub slug: Option<String>
}
