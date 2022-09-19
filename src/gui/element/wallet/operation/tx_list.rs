use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, Interaction, Message},
    crate::localization::localized_string,
    grin_gui_core::theme::ColorPalette,
    grin_gui_widgets::{header, Header},
    iced::{button, pick_list, scrollable, text_input, Button, Container, Element, Length, Text},
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
    strfmt::strfmt,
};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum ColumnKey {
    Title,
    LocalVersion,
    RemoteVersion,
    Status,
    Channel,
    Author,
    GameVersion,
    DateReleased,
    Source,
    Summary,
    // Only used for sorting, not an actual visible column that can be shown
    FuzzyScore,
}

impl ColumnKey {
    fn title(self) -> String {
        use ColumnKey::*;

        match self {
            Title => localized_string("addon"),
            LocalVersion => localized_string("local"),
            RemoteVersion => localized_string("remote"),
            Status => localized_string("status"),
            Channel => localized_string("channel"),
            Author => localized_string("author"),
            GameVersion => localized_string("game-version"),
            DateReleased => localized_string("latest-release"),
            Source => localized_string("source"),
            Summary => localized_string("summary"),
            FuzzyScore => unreachable!("fuzzy score not used as an actual column"),
        }
    }

    fn as_string(self) -> String {
        use ColumnKey::*;

        let s = match self {
            Title => "title",
            LocalVersion => "local",
            RemoteVersion => "remote",
            Status => "status",
            Channel => "channel",
            Author => "author",
            GameVersion => "game_version",
            DateReleased => "date_released",
            Source => "source",
            Summary => "summary",
            FuzzyScore => unreachable!("fuzzy score not used as an actual column"),
        };

        s.to_string()
    }
}

impl From<&str> for ColumnKey {
    fn from(s: &str) -> Self {
        match s {
            "title" => ColumnKey::Title,
            "local" => ColumnKey::LocalVersion,
            "remote" => ColumnKey::RemoteVersion,
            "status" => ColumnKey::Status,
            "channel" => ColumnKey::Channel,
            "author" => ColumnKey::Author,
            "game_version" => ColumnKey::GameVersion,
            "date_released" => ColumnKey::DateReleased,
            "source" => ColumnKey::Source,
            "summary" => ColumnKey::Summary,
            _ => panic!("Unknown ColumnKey for {}", s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    fn toggle(self) -> SortDirection {
        match self {
            SortDirection::Asc => SortDirection::Desc,
            SortDirection::Desc => SortDirection::Asc,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Catalog {
    pub addons: Vec<CatalogAddon>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct Version {
    pub game_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogAddon {
    #[serde(deserialize_with = "null_to_default::deserialize")]
    pub id: i32,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    pub url: String,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    pub name: String,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    pub categories: Vec<String>,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    pub summary: String,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    pub number_of_downloads: u64,
    #[serde(deserialize_with = "skip_element_unknown_variant::deserialize")]
    pub versions: Vec<Version>,
}

mod null_to_default {
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: Default + Deserialize<'de>,
    {
        let opt = Option::deserialize(deserializer)?;
        Ok(opt.unwrap_or_default())
    }
}

mod skip_element_unknown_variant {
    use serde::{
        de::{self, SeqAccess, Visitor},
        Deserialize, Deserializer,
    };
    use std::fmt;
    use std::marker::PhantomData;

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        struct SeqVisitor<V>(PhantomData<V>);

        impl<'de, V> Visitor<'de> for SeqVisitor<V>
        where
            V: Deserialize<'de>,
        {
            type Value = Vec<V>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "an array of values")
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(vec![])
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(vec![])
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut values = vec![];

                loop {
                    let value = seq.next_element::<V>();

                    match value {
                        Ok(Some(v)) => {
                            values.push(v);
                        }
                        Ok(None) => break,
                        Err(e) => {
                            if e.to_string().starts_with("unknown variant") {
                                continue;
                            } else {
                                return Err(e);
                            }
                        }
                    }
                }

                Ok(values)
            }
        }

        deserializer.deserialize_any(SeqVisitor(PhantomData::default()))
    }
}

mod date_parser {
    use chrono::prelude::*;
    use serde::{self, Deserialize, Deserializer};

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        // Curse format
        let date = DateTime::parse_from_rfc3339(&s)
            .map(|d| d.with_timezone(&Utc))
            .ok();

        if date.is_some() {
            return Ok(date);
        }

        // Tukui format
        let date = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %T")
            .map(|d| Utc.from_utc_datetime(&d))
            .ok();

        if date.is_some() {
            return Ok(date);
        }

        // Handles Elvui and Tukui addons which runs in a format without HH:mm:ss.
        let s_modified = format!("{} 00:00:00", &s);
        let date = NaiveDateTime::parse_from_str(&s_modified, "%Y-%m-%d %T")
            .map(|d| Utc.from_utc_datetime(&d))
            .ok();

        if date.is_some() {
            return Ok(date);
        }

        // Handles WowI.
        if let Ok(ts) = &s.parse::<i64>() {
            let date = Utc.timestamp(ts / 1000, 0);
            return Ok(Some(date));
        }

        Ok(None)
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CatalogResultSize {
    _25,
    _50,
    _100,
    _500,
}

impl Default for CatalogResultSize {
    fn default() -> Self {
        CatalogResultSize::_25
    }
}

impl CatalogResultSize {
    pub fn all() -> Vec<CatalogResultSize> {
        vec![
            CatalogResultSize::_25,
            CatalogResultSize::_50,
            CatalogResultSize::_100,
            CatalogResultSize::_500,
        ]
    }

    pub fn as_usize(self) -> usize {
        match self {
            CatalogResultSize::_25 => 25,
            CatalogResultSize::_50 => 50,
            CatalogResultSize::_100 => 100,
            CatalogResultSize::_500 => 500,
        }
    }
}

impl std::fmt::Display for CatalogResultSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vars = HashMap::new();
        vars.insert("number".to_string(), self.as_usize());
        let fmt = localized_string("catalog-results");

        write!(f, "{}", strfmt(&fmt, &vars).unwrap())
    }
}

pub struct HeaderState {
    state: header::State,
    previous_column_key: Option<ColumnKey>,
    previous_sort_direction: Option<SortDirection>,
    columns: Vec<ColumnState>,
}

impl HeaderState {
    fn column_config(&self) -> Vec<(ColumnKey, Length, bool)> {
        self.columns
            .iter()
            .map(|c| (c.key, c.width, c.hidden))
            .collect()
    }
}

impl Default for HeaderState {
    fn default() -> Self {
        Self {
            state: Default::default(),
            previous_column_key: None,
            previous_sort_direction: None,
            columns: vec![
                ColumnState {
                    key: ColumnKey::Title,
                    btn_state: Default::default(),
                    width: Length::Fill,
                    hidden: false,
                    order: 0,
                },
                ColumnState {
                    key: ColumnKey::LocalVersion,
                    btn_state: Default::default(),
                    width: Length::Units(150),
                    hidden: false,
                    order: 1,
                },
                ColumnState {
                    key: ColumnKey::RemoteVersion,
                    btn_state: Default::default(),
                    width: Length::Units(150),
                    hidden: false,
                    order: 2,
                },
                ColumnState {
                    key: ColumnKey::Status,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: false,
                    order: 3,
                },
                ColumnState {
                    key: ColumnKey::Channel,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: true,
                    order: 4,
                },
                ColumnState {
                    key: ColumnKey::Author,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: true,
                    order: 5,
                },
                ColumnState {
                    key: ColumnKey::GameVersion,
                    btn_state: Default::default(),
                    width: Length::Units(110),
                    hidden: true,
                    order: 6,
                },
                ColumnState {
                    key: ColumnKey::DateReleased,
                    btn_state: Default::default(),
                    width: Length::Units(110),
                    hidden: true,
                    order: 7,
                },
                ColumnState {
                    key: ColumnKey::Source,
                    btn_state: Default::default(),
                    width: Length::Units(110),
                    hidden: true,
                    order: 8,
                },
                ColumnState {
                    key: ColumnKey::Summary,
                    btn_state: Default::default(),
                    width: Length::Units(110),
                    hidden: true,
                    order: 9,
                },
            ],
        }
    }
}

pub struct ColumnState {
    key: ColumnKey,
    btn_state: button::State,
    width: Length,
    hidden: bool,
    order: usize,
}

pub struct ColumnSettings {
    pub scrollable_state: scrollable::State,
    pub columns: Vec<ColumnSettingState>,
}

impl Default for ColumnSettings {
    fn default() -> Self {
        ColumnSettings {
            scrollable_state: Default::default(),
            columns: vec![
                ColumnSettingState {
                    key: ColumnKey::Title,
                    order: 0,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::LocalVersion,
                    order: 1,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::RemoteVersion,
                    order: 2,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::Status,
                    order: 3,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::Channel,
                    order: 4,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::Author,
                    order: 5,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::GameVersion,
                    order: 6,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::DateReleased,
                    order: 7,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::Source,
                    order: 8,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::Summary,
                    order: 9,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
            ],
        }
    }
}

pub struct ColumnSettingState {
    pub key: ColumnKey,
    pub order: usize,
    pub up_btn_state: button::State,
    pub down_btn_state: button::State,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum CatalogColumnKey {
    Title,
    Description,
    Source,
    NumDownloads,
    GameVersion,
    DateReleased,
    Install,
    Categories,
}

impl CatalogColumnKey {
    fn title(self) -> String {
        use CatalogColumnKey::*;

        match self {
            Title => localized_string("addon"),
            Description => localized_string("description"),
            Source => localized_string("source"),
            NumDownloads => localized_string("num-downloads"),
            GameVersion => localized_string("game-version"),
            DateReleased => localized_string("latest-release"),
            Categories => localized_string("categories"),
            CatalogColumnKey::Install => localized_string("status"),
        }
    }

    fn as_string(self) -> String {
        use CatalogColumnKey::*;

        let s = match self {
            Title => "addon",
            Description => "description",
            Source => "source",
            NumDownloads => "num_downloads",
            GameVersion => "game_version",
            DateReleased => "date_released",
            Categories => "categories",
            CatalogColumnKey::Install => "install",
        };

        s.to_string()
    }
}

impl From<&str> for CatalogColumnKey {
    fn from(s: &str) -> Self {
        match s {
            "addon" => CatalogColumnKey::Title,
            "description" => CatalogColumnKey::Description,
            "source" => CatalogColumnKey::Source,
            "num_downloads" => CatalogColumnKey::NumDownloads,
            "install" => CatalogColumnKey::Install,
            "game_version" => CatalogColumnKey::GameVersion,
            "date_released" => CatalogColumnKey::DateReleased,
            "categories" => CatalogColumnKey::Categories,
            _ => panic!("Unknown CatalogColumnKey for {}", s),
        }
    }
}

pub struct CatalogHeaderState {
    state: header::State,
    previous_column_key: Option<CatalogColumnKey>,
    previous_sort_direction: Option<SortDirection>,
    columns: Vec<CatalogColumnState>,
}

impl CatalogHeaderState {
    fn column_config(&self) -> Vec<(CatalogColumnKey, Length, bool)> {
        self.columns
            .iter()
            .map(|c| (c.key, c.width, c.hidden))
            .collect()
    }
}

impl Default for CatalogHeaderState {
    fn default() -> Self {
        Self {
            state: Default::default(),
            previous_column_key: None,
            previous_sort_direction: None,
            columns: vec![
                CatalogColumnState {
                    key: CatalogColumnKey::Title,
                    btn_state: Default::default(),
                    width: Length::Fill,
                    hidden: false,
                    order: 0,
                },
                CatalogColumnState {
                    key: CatalogColumnKey::Description,
                    btn_state: Default::default(),
                    width: Length::Units(150),
                    hidden: false,
                    order: 1,
                },
                CatalogColumnState {
                    key: CatalogColumnKey::Source,
                    btn_state: Default::default(),
                    width: Length::Units(110),
                    hidden: false,
                    order: 2,
                },
                CatalogColumnState {
                    key: CatalogColumnKey::NumDownloads,
                    btn_state: Default::default(),
                    width: Length::Units(105),
                    hidden: true,
                    order: 3,
                },
                CatalogColumnState {
                    key: CatalogColumnKey::GameVersion,
                    btn_state: Default::default(),
                    width: Length::Units(105),
                    hidden: true,
                    order: 4,
                },
                CatalogColumnState {
                    key: CatalogColumnKey::DateReleased,
                    btn_state: Default::default(),
                    width: Length::Units(105),
                    hidden: false,
                    order: 5,
                },
                CatalogColumnState {
                    key: CatalogColumnKey::Install,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: false,
                    order: 6,
                },
                CatalogColumnState {
                    key: CatalogColumnKey::Categories,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: true,
                    order: 7,
                },
            ],
        }
    }
}

pub struct CatalogColumnState {
    key: CatalogColumnKey,
    btn_state: button::State,
    width: Length,
    hidden: bool,
    order: usize,
}

pub struct CatalogColumnSettings {
    pub scrollable_state: scrollable::State,
    pub columns: Vec<CatalogColumnSettingState>,
}

impl Default for CatalogColumnSettings {
    fn default() -> Self {
        CatalogColumnSettings {
            scrollable_state: Default::default(),
            columns: vec![
                CatalogColumnSettingState {
                    key: CatalogColumnKey::Title,
                    order: 0,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                CatalogColumnSettingState {
                    key: CatalogColumnKey::Description,
                    order: 1,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                CatalogColumnSettingState {
                    key: CatalogColumnKey::Source,
                    order: 2,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                CatalogColumnSettingState {
                    key: CatalogColumnKey::NumDownloads,
                    order: 3,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                CatalogColumnSettingState {
                    key: CatalogColumnKey::GameVersion,
                    order: 4,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                CatalogColumnSettingState {
                    key: CatalogColumnKey::DateReleased,
                    order: 5,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                CatalogColumnSettingState {
                    key: CatalogColumnKey::Install,
                    order: 6,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                CatalogColumnSettingState {
                    key: CatalogColumnKey::Categories,
                    order: 7,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
            ],
        }
    }
}

pub struct CatalogColumnSettingState {
    pub key: CatalogColumnKey,
    pub order: usize,
    pub up_btn_state: button::State,
    pub down_btn_state: button::State,
}

pub struct CatalogSearchState {
    pub catalog_rows: Vec<CatalogRow>,
    pub scrollable_state: scrollable::State,
    pub query: Option<String>,
    pub query_state: text_input::State,
    pub result_size: CatalogResultSize,
    pub result_sizes: Vec<CatalogResultSize>,
    pub result_sizes_state: pick_list::State<CatalogResultSize>,
}

impl Default for CatalogSearchState {
    fn default() -> Self {
        CatalogSearchState {
            catalog_rows: Default::default(),
            scrollable_state: Default::default(),
            query: None,
            query_state: Default::default(),
            result_size: Default::default(),
            result_sizes: CatalogResultSize::all(),
            result_sizes_state: Default::default(),
        }
    }
}

pub struct CatalogRow {
    install_button_state: button::State,
}

fn row_title<T: PartialEq>(
    column_key: T,
    previous_column_key: Option<T>,
    previous_sort_direction: Option<SortDirection>,
    title: &str,
) -> String {
    if Some(column_key) == previous_column_key {
        match previous_sort_direction {
            Some(SortDirection::Asc) => format!("{} ▲", title),
            Some(SortDirection::Desc) => format!("{} ▼", title),
            _ => title.to_string(),
        }
    } else {
        title.to_string()
    }
}

pub fn titles_row_header<'a>(
    color_palette: ColorPalette,
    catalog: &Catalog,
    header_state: &'a mut header::State,
    column_state: &'a mut [CatalogColumnState],
    previous_column_key: Option<CatalogColumnKey>,
    previous_sort_direction: Option<SortDirection>,
) -> Header<'a, Message> {
    // A row containing titles above the addon rows.
    let mut row_titles = vec![];

    for column in column_state.iter_mut().filter(|c| !c.hidden) {
        let column_key = column.key;

        let row_title = row_title(
            column_key,
            previous_column_key,
            previous_sort_direction,
            &column.key.title(),
        );

        let mut row_header = Button::new(
            &mut column.btn_state,
            Text::new(row_title)
                .size(DEFAULT_FONT_SIZE)
                .width(Length::Fill),
        )
        .width(Length::Fill);

        if column_key != CatalogColumnKey::Install {
            //TODO
            //row_header = row_header.on_press(Interaction::SortCatalogColumn(column_key));
        }

        if previous_column_key == Some(column_key) {
            row_header = row_header.style(style::SelectedColumnHeaderButton(color_palette));
        } else if column_key == CatalogColumnKey::Install {
            row_header = row_header.style(style::UnclickableColumnHeaderButton(color_palette));
        } else {
            row_header = row_header.style(style::ColumnHeaderButton(color_palette));
        }

        let row_header: Element<Interaction> = row_header.into();

        let row_container = Container::new(row_header.map(Message::Interaction))
            .width(column.width)
            .style(style::NormalBackgroundContainer(color_palette));

        // Only shows row titles if we have any catalog results.
        if !catalog.addons.is_empty() {
            row_titles.push((column.key.as_string(), row_container));
        }
    }

    Header::new(
        header_state,
        row_titles,
        Some(Length::Units(DEFAULT_PADDING)),
        Some(Length::Units(DEFAULT_PADDING + 5)),
    )
    .spacing(1)
    .height(Length::Units(25))
    /* .on_resize(3, |event| {
        //TODO
        //Message::Interaction(Interaction::ResizeColumn(Mode::Catalog, event))
    })*/
}
