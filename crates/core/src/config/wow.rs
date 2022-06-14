use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Struct for settings related to World of Warcraft.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct Wow {
    #[serde(default)]
    #[allow(deprecated)]
    pub directory: Option<PathBuf>,
}

impl Default for Wow {
    fn default() -> Self {
        Wow {
            directory: None,
        }
    }
}

