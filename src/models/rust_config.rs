use serde::{Deserialize, Serialize};

/// Configuration for the Rust compiler(e.g., for playground)
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct RustConfig {
    /// Rust edition used in playground
    pub edition: Option<RustEdition>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
/// Rust edition to use for the code.
pub enum RustEdition {
    /// The 2021 edition of Rust
    #[serde(rename = "2021")]
    E2021,
    /// The 2018 edition of Rust
    #[serde(rename = "2018")]
    E2018,
    /// The 2015 edition of Rust
    #[serde(rename = "2015")]
    E2015,
}
