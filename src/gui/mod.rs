mod element;
mod style;
mod update;

use crate::cli::Opts;
use crate::localization::{localized_string, LANG};
use ajour_core::{
    config::{Config, Language},
    error::ThemeError,
    fs::PersistentData,
    theme::Theme,
};

use iced::{
    button, pick_list, scrollable, slider, text_input, Alignment, Application, Button, Column,
    Command, Container, Element, Length, PickList, Row, Scrollable, Settings, Space, Subscription,
    Text, TextInput,
};

use image::ImageFormat;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use ajour_core::theme::ColorPalette;
use element::DEFAULT_PADDING;

static WINDOW_ICON: &[u8] = include_bytes!("../../resources/windows/ajour.ico");

pub struct Ajour {
    state: HashMap<Mode, State>,
    error: Option<anyhow::Error>,
    mode: Mode,
    config: Config,
    /// Main menu state
    menu_state: element::menu::StateContainer,

    /// Settings screen + sub-screens states
    settings_state: element::settings::StateContainer,
    wallet_settings_state: element::settings::wallet::StateContainer,
    node_settings_state: element::settings::node::StateContainer,
    general_settings_state: element::settings::general::StateContainer,

    /// About screen state
    about_state: element::about::StateContainer,
}

impl<'a> Default for Ajour {
    fn default() -> Self {
        let mut state = HashMap::new();
        state.insert(Mode::Catalog, State::Loading);

        Self {
            state,
            error: None,
            mode: Mode::Catalog,
            config: Config::default(),
            menu_state: Default::default(),
            settings_state: Default::default(),
            wallet_settings_state: Default::default(),
            node_settings_state: Default::default(),
            general_settings_state: Default::default(),
            about_state: Default::default(),
        }
    }
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum Message {
    Error(anyhow::Error),
    Interaction(Interaction),
    GeneralSettingsViewThemeSelected(String),
    GeneralSettingsViewThemeImported(Result<(String, Vec<Theme>), ThemeError>),
    RuntimeEvent(iced_native::Event),
    None(()),
}

impl Application for Ajour {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Config;

    fn new(config: Config) -> (Self, Command<Message>) {
        let mut ajour = Ajour::default();
        apply_config(&mut ajour, config);
        (ajour, Command::batch(vec![]))
    }

    fn title(&self) -> String {
        String::from("Ajour")
    }

    fn scale_factor(&self) -> f64 {
        self.general_settings_state.scale_state.scale
    }

    #[cfg(target_os = "windows")]
    fn should_exit(&self) -> bool {
        use crate::tray::SHOULD_EXIT;
        use std::sync::atomic::Ordering;

        SHOULD_EXIT.load(Ordering::Relaxed)
    }

    #[cfg(target_os = "windows")]
    fn mode(&self) -> iced::window::Mode {
        use crate::tray::GUI_VISIBLE;
        use iced::window::Mode;
        use std::sync::atomic::Ordering;

        if GUI_VISIBLE.load(Ordering::Relaxed) {
            Mode::Windowed
        } else {
            Mode::Hidden
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let runtime_subscription = iced_native::subscription::events().map(Message::RuntimeEvent);
        iced::Subscription::batch(vec![runtime_subscription])
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match update::handle_message(self, message) {
            Ok(x) => x,
            Err(e) => Command::perform(async { e }, Message::Error),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let color_palette = self
            .general_settings_state
            .theme_state
            .themes
            .iter()
            .find(|(name, _)| name == &self.general_settings_state.theme_state.current_theme_name)
            .as_ref()
            .unwrap_or(&&("Dark".to_string(), Theme::dark()))
            .1
            .palette;

        let mut content = Column::new();

        let menu_state = self.menu_state.clone();

        content = Column::new().push(element::menu::data_container(
            &mut self.menu_state,
            color_palette,
            &mut self.error,
        ));

        // Spacer between menu and content.
        //content = content.push(Space::new(Length::Units(0), Length::Units(DEFAULT_PADDING)));
        match menu_state.mode {
            element::menu::Mode::About => {
                let about_container =
                    element::about::data_container(color_palette, &None, &mut self.about_state);
                content = content.push(about_container)
            }
            element::menu::Mode::Settings => {
                content = content.push(element::settings::data_container(
                    &mut self.settings_state,
                    &mut self.config,
                    &mut self.wallet_settings_state,
                    &mut self.node_settings_state,
                    &mut self.general_settings_state,
                    color_palette,
                ))
                /*if let Some(settings_container) = views.get_mut(settings_view_index) {
                    content = content.push(settings_container.view.data_container(color_palette))
                }*/
            }
            _ => {}
        }
        let container: Option<Container<Message>> = match self.mode {
            _ => None,
        };

        if let Some(c) = container {
            content = content.push(c);
        };

        // Finally wraps everything in a container.
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::NormalBackgroundContainer(color_palette))
            .into()
    }
}

/// Starts the GUI.
/// This function does not return.
pub fn run(opts: Opts, config: Config) {
    // Set LANG using config (defaults to "en_US")
    LANG.set(RwLock::new(config.language.language_code()))
        .expect("setting LANG from config");

    log::debug!("config loaded:\n{:#?}", &config);

    let mut settings = Settings::default();
    settings.window.size = config.window_size.unwrap_or((900, 620));

    #[cfg(target_os = "windows")]
    {
        settings.exit_on_close_request = false;
    }

    #[cfg(not(target_os = "linux"))]
    // TODO (casperstorm): Due to an upstream bug, min_size causes the window to become unresizable
    // on Linux.
    // @see: https://github.com/ajour/ajour/issues/427
    {
        settings.window.min_size = Some((600, 300));
    }

    #[cfg(feature = "wgpu")]
    {
        let antialiasing = opts.antialiasing.unwrap_or(true);
        log::debug!("antialiasing: {}", antialiasing);
        settings.antialiasing = antialiasing;
    }

    #[cfg(feature = "opengl")]
    {
        let antialiasing = opts.antialiasing.unwrap_or(false);
        log::debug!("antialiasing: {}", antialiasing);
        settings.antialiasing = antialiasing;
    }

    // Sets the Window icon.
    let image = image::load_from_memory_with_format(WINDOW_ICON, ImageFormat::Ico)
        .expect("loading icon")
        .to_rgba8();
    let (width, height) = image.dimensions();
    let icon = iced::window::Icon::from_rgba(image.into_raw(), width, height);
    settings.window.icon = Some(icon.unwrap());

    settings.flags = config;

    // Runs the GUI.
    Ajour::run(settings).expect("running Ajour gui");
}

#[derive(Debug)]
pub enum State {
    Ready,
    Loading,
    Error(anyhow::Error),
}

impl Default for State {
    fn default() -> Self {
        State::Ready
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SelfUpdateStatus {
    InProgress,
    Failed,
}

impl std::fmt::Display for SelfUpdateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SelfUpdateStatus::InProgress => localized_string("updating"),
            SelfUpdateStatus::Failed => localized_string("failed"),
        };
        write!(f, "{}", s)
    }
}

#[derive(Default, Debug)]
pub struct SelfUpdateState {
    status: Option<SelfUpdateStatus>,
    btn_state: button::State,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Catalog,
    Install,
    Settings,
    About,
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Interaction {
    /// String representing view ID and enum message (specific to that view)
    MenuViewInteraction(element::menu::LocalViewInteraction),
    SettingsViewInteraction(element::settings::LocalViewInteraction),
    WalletSettingsViewInteraction(element::settings::wallet::LocalViewInteraction),
    NodeSettingsViewInteraction(element::settings::node::LocalViewInteraction),
    GeneralSettingsViewInteraction(element::settings::general::LocalViewInteraction),
    GeneralSettingsViewLanguageSelected(Language),
    GeneralSettingsViewImportTheme,
    GeneralSettingsViewThemeUrlInput(String),
    ViewInteraction(String, String),
    ModeSelected(Mode),
    ModeSelectedSettings(element::settings::Mode),
    //Expand(ExpandType),
    Ignore(String),
    SelectBackupDirectory(),
    OpenLink(String),
    Unignore(String),
    Update(String),
    ScaleUp,
    ScaleDown,
    Backup,
    ToggleHideIgnoredAddons(bool),
    CatalogQuery(String),
    InstallScmQuery(String),
    InstallScmUrl,
    UpdateAjour,
    AlternatingRowColorToggled(bool),
    KeybindingsToggle(bool),
    #[cfg(target_os = "windows")]
    ToggleCloseToTray(bool),
    #[cfg(target_os = "windows")]
    ToggleAutoStart(bool),
    #[cfg(target_os = "windows")]
    ToggleStartClosedToTray(bool),
}

pub struct ThemeState {
    themes: Vec<(String, Theme)>,
    current_theme_name: String,
    pick_list_state: pick_list::State<String>,
    input_state: text_input::State,
    input_url: String,
    import_button_state: button::State,
    open_builder_button_state: button::State,
}

impl Default for ThemeState {
    fn default() -> Self {
        let themes = Theme::all();

        ThemeState {
            themes,
            current_theme_name: "Dark".to_string(),
            pick_list_state: Default::default(),
            input_state: Default::default(),
            input_url: Default::default(),
            import_button_state: Default::default(),
            open_builder_button_state: Default::default(),
        }
    }
}

fn apply_config(ajour: &mut Ajour, mut config: Config) {
    // Set column widths from the config
    /*match &config.column_config {
        ColumnConfig::V1 {
            local_version_width,
            remote_version_width,
            status_width,
        } => {
            ajour
                .header_state
                .columns
                .get_mut(1)
                .as_mut()
                .unwrap()
                .width = Length::Units(*local_version_width);
            ajour
                .header_state
                .columns
                .get_mut(2)
                .as_mut()
                .unwrap()
                .width = Length::Units(*remote_version_width);
            ajour
                .header_state
                .columns
                .get_mut(3)
                .as_mut()
                .unwrap()
                .width = Length::Units(*status_width);
        }
        ColumnConfig::V2 { columns } => {
            ajour.header_state.columns.iter_mut().for_each(|a| {
                if let Some((idx, column)) = columns
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, column)| {
                        if column.key == a.key.as_string() {
                            Some((idx, column))
                        } else {
                            None
                        }
                    })
                    .next()
                {
                    a.width = column.width.map_or(Length::Fill, Length::Units);
                    a.hidden = column.hidden;
                    a.order = idx;
                }
            });

            ajour.column_settings.columns.iter_mut().for_each(|a| {
                if let Some(idx) = columns
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, column)| {
                        if column.key == a.key.as_string() {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                    .next()
                {
                    a.order = idx;
                }
            });

            // My Addons
            ajour.header_state.columns.sort_by_key(|c| c.order);
            ajour.column_settings.columns.sort_by_key(|c| c.order);
        }
        ColumnConfig::V3 {
            my_addons_columns,
            catalog_columns,
            aura_columns,
        } => {
            ajour.header_state.columns.iter_mut().for_each(|a| {
                if let Some((idx, column)) = my_addons_columns
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, column)| {
                        if column.key == a.key.as_string() {
                            Some((idx, column))
                        } else {
                            None
                        }
                    })
                    .next()
                {
                    // Always force "Title" column as Length::Fill
                    //
                    // Shouldn't be an issue here, as it was for catalog column fix
                    // below, but will cover things in case anyone accidently manually
                    // modifies their config and sets a fixed width on this column.
                    a.width = if a.key == ColumnKey::Title {
                        Length::Fill
                    } else {
                        column.width.map_or(Length::Fill, Length::Units)
                    };

                    a.hidden = column.hidden;
                    a.order = idx;
                }
            });

            ajour.column_settings.columns.iter_mut().for_each(|a| {
                if let Some(idx) = my_addons_columns
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, column)| {
                        if column.key == a.key.as_string() {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                    .next()
                {
                    a.order = idx;
                }
            });

            ajour
                .catalog_column_settings
                .columns
                .iter_mut()
                .for_each(|a| {
                    if let Some(idx) = catalog_columns
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, column)| {
                            if column.key == a.key.as_string() {
                                Some(idx)
                            } else {
                                None
                            }
                        })
                        .next()
                    {
                        a.order = idx;
                    }
                });

            ajour.catalog_header_state.columns.iter_mut().for_each(|a| {
                if let Some((idx, column)) = catalog_columns
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, column)| {
                        if column.key == a.key.as_string() {
                            Some((idx, column))
                        } else {
                            None
                        }
                    })
                    .next()
                {
                    // Always force "Title" column as Length::Fill
                    //
                    // An older version of ajour used a different column as the fill
                    // column and some users have migration issues when updating to
                    // a newer version, causing NO columns to be set as Fill and
                    // making resizing columns work incorrectly
                    a.width = if a.key == CatalogColumnKey::Title {
                        Length::Fill
                    } else {
                        column.width.map_or(Length::Fill, Length::Units)
                    };

                    a.hidden = column.hidden;
                    a.order = idx;
                }
            });

            ajour.aura_header_state.columns.iter_mut().for_each(|a| {
                if let Some((_idx, column)) = aura_columns
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, column)| {
                        if column.key == a.key.as_string() {
                            Some((idx, column))
                        } else {
                            None
                        }
                    })
                    .next()
                {
                    // Always force "Title" column as Length::Fill
                    //
                    // An older version of ajour used a different column as the fill
                    // column and some users have migration issues when updating to
                    // a newer version, causing NO columns to be set as Fill and
                    // making resizing columns work incorrectly
                    a.width = if a.key == AuraColumnKey::Title {
                        Length::Fill
                    } else {
                        column.width.map_or(Length::Fill, Length::Units)
                    };
                }
            });

            // My Addons
            ajour.header_state.columns.sort_by_key(|c| c.order);
            ajour.column_settings.columns.sort_by_key(|c| c.order);

            // Catalog
            ajour.catalog_header_state.columns.sort_by_key(|c| c.order);
            ajour
                .catalog_column_settings
                .columns
                .sort_by_key(|c| c.order);

            // No sorting on Aura columns currently
        }
    }*/

    // Use theme from config. Set to "Dark" if not defined.
    ajour.general_settings_state.theme_state.current_theme_name =
        config.theme.as_deref().unwrap_or("Dark").to_string();

    // Use scale from config. Set to 1.0 if not defined.
    ajour.general_settings_state.scale_state.scale = config.scale.unwrap_or(1.0);

    // Migration for the new TBC client. Link ClassicEra flavor to `_classic_era_` instead of
    // `_classic_`
    /*{
        if let Some(classic_era_dir) = config.wow.directories.get(&Flavor::ClassicEra) {
            if classic_era_dir.ends_with("_classic_") {
                if let Some(parent) = classic_era_dir.parent() {
                    let new_path = parent.join("_classic_era_");

                    config.wow.directories.insert(Flavor::ClassicEra, new_path);
                }
            }
        }
    }*/

    // Set the inital mode flavor
    //ajour.mode = Mode::MyAddons(config.wow.flavor);

    ajour.config = config;

    // @see (casperstorm): Migration from single World of Warcraft directory to multiple directories.
    // This is essentially deprecrating `ajour.config.wow.directory`.
    /*if ajour.config.wow.directory.is_some() {
        for flavor in Flavor::ALL.iter() {
            let path = ajour.config.wow.directory.as_ref().unwrap();
            let flavor_path = ajour.config.get_flavor_directory_for_flavor(flavor, path);
            if flavor_path.exists() {
                ajour.config.wow.directories.insert(*flavor, flavor_path);
            }
        }

        // Removing `directory`, so we don't end up here again.
        ajour.config.wow.directory = None;
    }*/

    let _ = &ajour.config.save();
}
