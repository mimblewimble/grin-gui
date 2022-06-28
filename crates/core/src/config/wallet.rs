use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Struct for settings related to World of Warcraft.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct Wallet {
    #[serde(default)]
    #[allow(deprecated)]
    pub toml_file_path: Option<PathBuf>,
}

impl Default for Wallet {
    fn default() -> Self {
        Wallet {
            toml_file_path: None,
        }
    }
}

