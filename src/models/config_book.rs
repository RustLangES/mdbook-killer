use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigBook {
    title: String,
    pub languages: Vec<String>
}

impl ConfigBook {
    pub fn new(title: String) -> Self {
        ConfigBook{
            title,
            languages: vec!["en".to_string(), "es".to_string()]
        }
    }
}