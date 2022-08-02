mod element;
mod style;
mod update;
mod time;

use crate::cli::Opts;
use crate::error_cause_string;
use crate::localization::{localized_string, LANG};
use crate::gui::element::{DEFAULT_FONT_SIZE, SMALLER_FONT_SIZE};
use grin_gui_core::{
    config::{Config, Language},
    error::ThemeError,
    fs::PersistentData,
    theme::{ColorPalette, Theme},
    wallet::{WalletInterfaceHttpNodeClient, HTTPNodeClient},
    node::{NodeInterface, subscriber::{self, ServerStats, UIMessage}, ChainTypes},
};

use iced::{
    button, pick_list, scrollable, slider, text_input, Alignment, Application, Button, Column,
    Command, Container, Element, Length, PickList, Row, Scrollable, Settings, Space, Subscription,
    Text, TextInput,
};

use iced_native::alignment;

use iced_aw::{modal, Card, Modal};

use iced_futures::futures::channel::mpsc;

use image::ImageFormat;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use element::{DEFAULT_PADDING, DEFAULT_HEADER_FONT_SIZE};

static WINDOW_ICON: &[u8] = include_bytes!("../../resources/windows/ajour.ico");

pub struct GrinGui {
    /// Wallet Interface
    wallet_interface: Arc<RwLock<WalletInterfaceHttpNodeClient>>,

    /// Node Interface
    node_interface: Arc<RwLock<NodeInterface>>,

    error: Option<anyhow::Error>,
    mode: Mode,
    config: Config,

    /// Top-level error modal overlay
    error_modal_state: modal::State<ModalState>,

    /// Main menu state
    menu_state: element::menu::StateContainer,

    /// Top-Level Wallet area state
    wallet_state: element::wallet::StateContainer,

    /// Top-Level Node area state
    node_state: element::node::StateContainer,

    /// Settings screen + sub-screens states
    settings_state: element::settings::StateContainer,
    wallet_settings_state: element::settings::wallet::StateContainer,
    node_settings_state: element::settings::node::StateContainer,
    general_settings_state: element::settings::general::StateContainer,

    /// About screen state
    about_state: element::about::StateContainer,
}

impl<'a> Default for GrinGui {
    fn default() -> Self {

        // Instantiate wallet node client
        // TODO: Fill out 
        let node_url = "http://localhost:8080";
    	let node_client = HTTPNodeClient::new(node_url, None).unwrap();

        Self {
            wallet_interface: Arc::new(RwLock::new(WalletInterfaceHttpNodeClient::new(node_client, ChainTypes::Mainnet))),
            node_interface: Arc::new(RwLock::new(NodeInterface::new(ChainTypes::Mainnet))),
            error: None,
            mode: Mode::Catalog,
            config: Config::default(),
            error_modal_state: Default::default(),
            menu_state: Default::default(),
            wallet_state: Default::default(),
            node_state: Default::default(),
            settings_state: Default::default(),
            wallet_settings_state: Default::default(),
            node_settings_state: Default::default(),
            general_settings_state: Default::default(),
            about_state: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum Message {
    Error(Arc<RwLock<Option<anyhow::Error>>>),
    SendNodeMessage((usize, UIMessage, Option<mpsc::Sender<UIMessage>>)),
    Interaction(Interaction),
    Tick(chrono::DateTime<chrono::Local>),
    RuntimeEvent(iced_native::Event),
    None(()),
}

impl Application for GrinGui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Config;

    fn new(config: Config) -> (Self, Command<Message>) {
        let mut grin_gui = GrinGui::default();
        let wallet_interface = grin_gui.wallet_interface.clone();
        let mut w = wallet_interface.write().unwrap();

        w.set_chain_type();

        // Check initial wallet status
        /*if !config.wallet.toml_file_path.is_some()
            || !w.config_exists(
                config
                    .wallet
                    .toml_file_path
                    .as_ref()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            )
        {
            grin_gui.menu_state.mode = element::menu::Mode::Wallet;
        }*/

        apply_config(&mut grin_gui, config);

        (grin_gui, Command::batch(vec![]))
    }

    fn title(&self) -> String {
        String::from("Grin")
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
        let tick_subscription = time::every(std::time::Duration::from_millis(1000)).map(Message::Tick);
        let node_subscription = subscriber::subscriber(0).map(|e| 
            Message::SendNodeMessage(e)
        );
 
        iced::Subscription::batch(vec![runtime_subscription, tick_subscription, node_subscription])
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match update::handle_message(self, message) {
            Ok(x) => x,
            Err(e) => Command::perform(async { Arc::new(RwLock::new(Some(e))) }, Message::Error),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
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
            element::menu::Mode::Wallet => {
                let setup_container = element::wallet::data_container(
                    color_palette,
                    &mut self.wallet_state,
                );
                content = content.push(setup_container)
            }
            element::menu::Mode::Node => {
                let node_container = element::node::data_container(
                    color_palette,
                    &mut self.node_state,
                );
                content = content.push(node_container)
            }
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

        let error_cause = if let Some(e) = &self.error {
            error_cause_string(&e)
        } else {
            "None".into()
        };
 
        let content: Element<Self::Message> = 
        // Wraps everything in a container.
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::NormalBackgroundContainer(color_palette))
            .into();

        Modal::new(&mut self.error_modal_state, content, move|state| {
            Card::new(
                Text::new(localized_string("error-detail")).size(DEFAULT_HEADER_FONT_SIZE),
                Text::new(&error_cause).size(DEFAULT_FONT_SIZE)
            )
              .foot(
                Column::new()
                    .spacing(10)
                    .padding(5)
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
                    .push(
                        Button::new(
                            &mut state.cancel_state,
                            Text::new(localized_string("ok-caps")).size(DEFAULT_FONT_SIZE).horizontal_alignment(alignment::Horizontal::Center),
                        )
                        .style(style::DefaultButton(color_palette))
                        .on_press(Message::Interaction(Interaction::CloseErrorModal)),
                    )
                    .push(
                        Button::new(
                            &mut state.ok_state,
                            Text::new(localized_string("copy-to-clipboard")).size(SMALLER_FONT_SIZE).horizontal_alignment(alignment::Horizontal::Center),
                        )
                        .style(style::NormalTextButton(color_palette))
                        .on_press(Message::Interaction(Interaction::WriteToClipboard(error_cause.clone()))),
                    )
            )
            .max_width(500)
            .on_close(Message::Interaction(Interaction::CloseErrorModal))
            .style(style::NormalModalCardContainer(color_palette))
            .into()
        })
        .backdrop(Message::Interaction(Interaction::CloseErrorModal))
        .on_esc(Message::Interaction(Interaction::CloseErrorModal))
        .style(style::NormalModalContainer(color_palette))
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
    GrinGui::run(settings).expect("running Grin gui");
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
    /// Error modal
    OpenErrorModal,
    CloseErrorModal,
    /// Clipboard copy
    WriteToClipboard(String),
    ReadFromClipboard(String),
    /// View interactions
    MenuViewInteraction(element::menu::LocalViewInteraction),
    SettingsViewInteraction(element::settings::LocalViewInteraction),
    WalletSettingsViewInteraction(element::settings::wallet::LocalViewInteraction),
    NodeSettingsViewInteraction(element::settings::node::LocalViewInteraction),
    GeneralSettingsViewInteraction(element::settings::general::LocalViewInteraction),
    GeneralSettingsViewImportTheme,
    WalletSetupViewInteraction(element::wallet::setup::LocalViewInteraction),
    WalletSetupInitViewInteraction(element::wallet::setup::init::LocalViewInteraction),
    WalletSetupWalletViewInteraction(element::wallet::setup::wallet_setup::LocalViewInteraction),
    WalletSetupWalletSuccessViewInteraction(element::wallet::setup::wallet_success::LocalViewInteraction),
    WalletOperationOpenViewInteraction(element::wallet::operation::open::LocalViewInteraction),
    WalletOperationHomeViewInteraction(element::wallet::operation::home::LocalViewInteraction),
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
    UpdateGrin,
    AlternatingRowColorToggled(bool),
    KeybindingsToggle(bool),
    #[cfg(target_os = "windows")]
    ToggleCloseToTray(bool),
    #[cfg(target_os = "windows")]
    ToggleAutoStart(bool),
    #[cfg(target_os = "windows")]
    ToggleStartClosedToTray(bool),
}

#[derive(Default)]
struct ModalState {
    cancel_state: button::State,
    ok_state: button::State,
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

fn apply_config(grin_gui: &mut GrinGui, mut config: Config) {
    // Set column widths from the config
    /*match &config.column_config {
        ColumnConfig::V1 {
            local_version_width,
            remote_version_width,
            status_width,
        } => {
            grin_gui
                .header_state
                .columns
                .get_mut(1)
                .as_mut()
                .unwrap()
                .width = Length::Units(*local_version_width);
            grin_gui
                .header_state
                .columns
                .get_mut(2)
                .as_mut()
                .unwrap()
                .width = Length::Units(*remote_version_width);
            grin_gui
                .header_state
                .columns
                .get_mut(3)
                .as_mut()
                .unwrap()
                .width = Length::Units(*status_width);
        }
        ColumnConfig::V2 { columns } => {
            grin_gui.header_state.columns.iter_mut().for_each(|a| {
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

            grin_gui.column_settings.columns.iter_mut().for_each(|a| {
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
            grin_gui.header_state.columns.sort_by_key(|c| c.order);
            grin_gui.column_settings.columns.sort_by_key(|c| c.order);
        }
        ColumnConfig::V3 {
            my_addons_columns,
            catalog_columns,
            aura_columns,
        } => {
            grin_gui.header_state.columns.iter_mut().for_each(|a| {
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

            grin_gui.column_settings.columns.iter_mut().for_each(|a| {
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

            grin_gui
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

            grin_gui.catalog_header_state.columns.iter_mut().for_each(|a| {
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

            grin_gui.aura_header_state.columns.iter_mut().for_each(|a| {
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
            grin_gui.header_state.columns.sort_by_key(|c| c.order);
            grin_gui.column_settings.columns.sort_by_key(|c| c.order);

            // Catalog
            grin_gui.catalog_header_state.columns.sort_by_key(|c| c.order);
            grin_gui
                .catalog_column_settings
                .columns
                .sort_by_key(|c| c.order);

            // No sorting on Aura columns currently
        }
    }*/

    // Use theme from config. Set to "Dark" if not defined.
    grin_gui
        .general_settings_state
        .theme_state
        .current_theme_name = config.theme.as_deref().unwrap_or("Dark").to_string();

    // Use scale from config. Set to 1.0 if not defined.
    grin_gui.general_settings_state.scale_state.scale = config.scale.unwrap_or(1.0);

    grin_gui.config = config;

    let _ = &grin_gui.config.save();
}
