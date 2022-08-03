use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Struct for settings related to World of Warcraft.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct Wallet {
    #[serde(default)]
    #[allow(deprecated)]
    /// Top-level directory. Should (but not always) contain grin_wallet.toml file
    pub tld: Option<PathBuf>,
    /// Display name in wallet selection
    pub display_name: String,
    /// If true, override the grin_wallet.toml configured node and use the internal one
    pub use_embedded_node: bool,
}

impl Default for Wallet {
    fn default() -> Self {
        Wallet {
            tld: None,
            display_name: "Default".to_owned(),
            use_embedded_node: true,
        }
    }
}