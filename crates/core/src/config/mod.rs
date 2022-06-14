use crate::backup::CompressionFormat;
use crate::error::FilesystemError;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

mod wow;

use crate::fs::PersistentData;

pub use crate::config::wow::Wow;

/// Config struct.
#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub wow: Wow,

    pub theme: Option<String>,

    #[serde(default)]
    pub column_config: ColumnConfig,

    pub window_size: Option<(u32, u32)>,

    pub scale: Option<f64>,

    pub backup_directory: Option<PathBuf>,

    #[serde(default)]
    pub backup_addons: bool,

    #[serde(default)]
    pub backup_wtf: bool,

    #[serde(default)]
    pub backup_config: bool,

    #[serde(default)]
    pub backup_screenshots: bool,

    #[serde(default)]
    pub backup_fonts: bool,

    #[serde(default)]
    pub hide_ignored_addons: bool,

    #[serde(default)]
    pub self_update_channel: SelfUpdateChannel,

    #[serde(default = "default_true")]
    pub alternating_row_colors: bool,

    #[serde(default = "default_true")]
    pub is_keybindings_enabled: bool,

    #[serde(default)]
    pub language: Language,

    #[serde(default)]
    pub auto_update: bool,

    #[serde(default)]
    pub compression_format: CompressionFormat,

    #[serde(default)]
    pub zstd_compression_level: i32,

    #[serde(default)]
    #[cfg(target_os = "windows")]
    pub close_to_tray: bool,

    #[serde(default)]
    #[cfg(target_os = "windows")]
    pub autostart: bool,

    #[serde(default)]
    #[cfg(target_os = "windows")]
    pub start_closed_to_tray: bool,
}

impl PersistentData for Config {
    fn relative_path() -> PathBuf {
        PathBuf::from("ajour.yml")
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum ColumnConfig {
    V1 {
        local_version_width: u16,
        remote_version_width: u16,
        status_width: u16,
    },
    V2 {
        columns: Vec<ColumnConfigV2>,
    },
    V3 {
        my_addons_columns: Vec<ColumnConfigV2>,
        catalog_columns: Vec<ColumnConfigV2>,
        #[serde(default)]
        aura_columns: Vec<ColumnConfigV2>,
    },
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ColumnConfigV2 {
    pub key: String,
    pub width: Option<u16>,
    pub hidden: bool,
}

impl Default for ColumnConfig {
    fn default() -> Self {
        ColumnConfig::V1 {
            local_version_width: 150,
            remote_version_width: 150,
            status_width: 85,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelfUpdateChannel {
    Stable,
    Beta,
}

impl SelfUpdateChannel {
    pub const fn all() -> [Self; 2] {
        [SelfUpdateChannel::Stable, SelfUpdateChannel::Beta]
    }
}

impl Default for SelfUpdateChannel {
    fn default() -> Self {
        SelfUpdateChannel::Stable
    }
}

impl Display for SelfUpdateChannel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            SelfUpdateChannel::Stable => "Stable",
            SelfUpdateChannel::Beta => "Beta",
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash, PartialOrd, Ord)]
pub enum Language {
    Czech,
    Norwegian,
    English,
    Danish,
    German,
    French,
    Hungarian,
    Portuguese,
    Polish,
    Russian,
    Slovak,
    Swedish,
    Spanish,
    Turkish,
    Ukrainian,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Czech => "Čeština",
                Language::Danish => "Dansk",
                Language::English => "English",
                Language::French => "Français",
                Language::German => "Deutsch",
                Language::Hungarian => "Magyar",
                Language::Norwegian => "Norsk Bokmål",
                Language::Portuguese => "Português",
                Language::Polish => "Polski",
                Language::Russian => "Pусский",
                Language::Slovak => "Slovenčina",
                Language::Spanish => "Español",
                Language::Swedish => "Svenska",
                Language::Turkish => "Türkçe",
                Language::Ukrainian => "Yкраїнська",
            }
        )
    }
}

impl Language {
    // Alphabetically sorted based on their local name (@see `impl Display`).
    pub const ALL: [Language; 15] = [
        Language::Czech,
        Language::Danish,
        Language::German,
        Language::English,
        Language::Spanish,
        Language::French,
        Language::Hungarian,
        Language::Norwegian,
        Language::Portuguese,
        Language::Polish,
        Language::Russian,
        Language::Slovak,
        Language::Swedish,
        Language::Turkish,
        Language::Ukrainian,
    ];

    pub const fn language_code(self) -> &'static str {
        match self {
            Language::Czech => "cs_CZ",
            Language::English => "en_US",
            Language::Danish => "da_DK",
            Language::German => "de_DE",
            Language::French => "fr_FR",
            Language::Russian => "ru_RU",
            Language::Swedish => "se_SE",
            Language::Spanish => "es_ES",
            Language::Hungarian => "hu_HU",
            Language::Norwegian => "nb_NO",
            Language::Slovak => "sk_SK",
            Language::Turkish => "tr_TR",
            Language::Portuguese => "pt_PT",
            Language::Polish => "pl_PL",
            Language::Ukrainian => "uk_UA",
        }
    }
}

impl Default for Language {
    fn default() -> Language {
        Language::English
    }
}

/// Returns a Config.
///
/// This functions handles the initialization of a Config.
pub async fn load_config() -> Result<Config, FilesystemError> {
    log::debug!("loading config");

    Ok(Config::load_or_default()?)
}

const fn default_true() -> bool {
    true
}

#[cfg(test)]
mod test {

    /// This method will take a relative path and make a case insentitive pattern
    // For some reason the case insensitive pattern doesn't work
    // unless we add an actual pattern symbol, hence the `?`.
    fn get_pattern_format(relative_path: &str) -> String {
        let splitted_string = relative_path.split('/');
        let mut return_string: Vec<String> = vec![];
        for path in splitted_string {
            let mut to_lower_case = path.to_lowercase();
            to_lower_case.replace_range(0..1, "?");
            return_string.push(to_lower_case);
        }
        return_string.join("/")
    }

    #[test]
    fn test_get_format_interface_addons() {
        assert_eq!(
            get_pattern_format("Interface/Addons"),
            String::from("?nterface/?ddons")
        );
        assert_eq!(
            get_pattern_format("iNtErFaCe/aDdoNs"),
            String::from("?nterface/?ddons")
        );
    }

    #[test]
    fn test_get_format_wtf() {
        assert_eq!(get_pattern_format("WTF"), String::from("?tf"));
        assert_eq!(get_pattern_format("wTF"), String::from("?tf"));
        assert_eq!(get_pattern_format("Wtf"), String::from("?tf"));
        assert_eq!(get_pattern_format("wTf"), String::from("?tf"));
    }

    #[test]
    fn test_get_format_screenshots() {
        assert_eq!(
            get_pattern_format("Screenshots"),
            String::from("?creenshots")
        );
        assert_eq!(
            get_pattern_format("sCREENSHOTS"),
            String::from("?creenshots")
        );
        assert_eq!(
            get_pattern_format("ScreeNShots"),
            String::from("?creenshots")
        );
    }
}
