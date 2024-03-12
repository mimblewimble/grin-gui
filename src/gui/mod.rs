mod element;
//mod style;
mod time;
mod update;

use crate::cli::Opts;
use crate::error_cause_string;
use crate::gui::element::{DEFAULT_FONT_SIZE, SMALLER_FONT_SIZE};
use crate::localization::{localized_string, LANG};
use grin_gui_core::theme::Element;
use grin_gui_core::{
	config::Config,
	fs::PersistentData,
	node::{
		subscriber::{self, UIMessage},
		ChainTypes, NodeInterface,
	},
	theme::{Button, ColorPalette, Column, Container, PickList, Row, Scrollable, Text, Theme},
	wallet::{get_grin_wallet_default_path, global, HTTPNodeClient, WalletInterfaceHttpNodeClient},
};

use iced::widget::{button, pick_list, scrollable, text_input, Checkbox, Space, TextInput};
use iced::{
	alignment, font, window, Alignment, Application, Command, Length, Settings, Subscription,
};

use iced_aw::{modal, Card, Modal};

use iced_futures::futures::channel::mpsc;

use image::ImageFormat;

use std::borrow::BorrowMut;
//use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use element::DEFAULT_HEADER_FONT_SIZE;

static WINDOW_ICON: &[u8] = include_bytes!("../../resources/windows/grin.ico");

pub struct GrinGui {
	/// Wallet Interface
	wallet_interface: Arc<RwLock<WalletInterfaceHttpNodeClient>>,

	/// Node Interface
	node_interface: Arc<RwLock<NodeInterface>>,

	error: Option<anyhow::Error>,
	mode: Mode,
	config: Config,

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

	show_modal: bool,
	modal_type: ModalType,
	exit: bool,
	theme: Theme,
}

impl GrinGui {
	pub fn show_exit(&mut self, show: bool) {
		self.show_modal = show;
		if show {
			self.modal_type = ModalType::Exit;
		} else {
			self.modal_type = ModalType::Error;
		}
	}

	pub fn safe_exit(&mut self) {
		let mut node = self.node_interface.write().unwrap();
		node.shutdown_server(true);
	}
}

impl GrinGui {
	fn from_config(config: &Config) -> Self {
		// Instantiate wallet node client
		// TODO: Fill out
		let node_url = "http://localhost:8080";
		let node_client = HTTPNodeClient::new(node_url, None).unwrap();

		// restore theme from config
		let name = config.theme.clone().unwrap_or("Alliance".to_string());
		let theme = Theme::all().iter().find(|t| t.0 == name).unwrap().1.clone();

		Self {
			wallet_interface: Arc::new(RwLock::new(WalletInterfaceHttpNodeClient::new(
				node_client,
			))),
			node_interface: Arc::new(RwLock::new(NodeInterface::new())),
			error: None,
			mode: Mode::Catalog,
			config: Config::default(),
			menu_state: Default::default(),
			wallet_state: Default::default(),
			node_state: Default::default(),
			settings_state: Default::default(),
			wallet_settings_state: Default::default(),
			node_settings_state: Default::default(),
			general_settings_state: Default::default(),
			about_state: Default::default(),
			show_modal: false,
			modal_type: ModalType::Error,
			exit: false,
			theme,
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
	RuntimeEvent(iced_core::Event),
	FontLoaded(Result<(), font::Error>),
	None(()),
}

pub enum ModalType {
	Exit,
	Error,
}

impl Application for GrinGui {
	type Executor = iced::executor::Default;
	type Message = Message;
	type Flags = Config;
	type Theme = Theme;

	fn theme(&self) -> Theme {
		self.theme.clone()
	}

	fn new(config: Config) -> (Self, Command<Message>) {
		let mut grin_gui = GrinGui::from_config(&config);

		// default Mainnet
		global::set_local_chain_type(ChainTypes::Mainnet);

		if let Some(wallet_index) = config.current_wallet_index {
			let wallet = config.wallets[wallet_index].clone();
			global::set_local_chain_type(wallet.chain_type);
		}

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
		let load_font_reg =
			iced::font::load(include_bytes!("../../fonts/notosans-regular.ttf").as_slice())
				.map(Message::FontLoaded);
		let load_font_bold =
			iced::font::load(include_bytes!("../../fonts/notosans-bold.ttf").as_slice())
				.map(Message::FontLoaded);
		(
			grin_gui,
			Command::batch(vec![load_font_reg, load_font_bold]),
		)
	}

	fn title(&self) -> String {
		String::from("Grin")
	}

	fn scale_factor(&self) -> f64 {
		self.general_settings_state.scale_state.scale
	}

	/*#[cfg(target_os = "windows")]
	fn mode(&self) -> iced::window::Mode {
		use crate::tray::GUI_VISIBLE;
		use iced::window::Mode;
		use std::sync::atomic::Ordering;

		if GUI_VISIBLE.load(Ordering::Relaxed) {
			Mode::Windowed
		} else {
			Mode::Hidden
		}
	}*/

	fn subscription(&self) -> Subscription<Message> {
		let runtime_subscription = iced_futures::subscription::events().map(Message::RuntimeEvent);
		let tick_subscription =
			time::every(std::time::Duration::from_millis(1000)).map(Message::Tick);
		let node_subscription = subscriber::subscriber(0).map(|e| Message::SendNodeMessage(e));

		iced::Subscription::batch(vec![
			runtime_subscription,
			tick_subscription,
			node_subscription,
		])
	}

	fn update(&mut self, message: Message) -> Command<Message> {
		match update::handle_message(self, message) {
			Ok(x) => x,
			Err(e) => Command::perform(async { Arc::new(RwLock::new(Some(e))) }, Message::Error),
		}
	}

	fn view(&self) -> Element<Message> {
		let menu_state = self.menu_state.clone();

		let mut content =
			Column::new().push(element::menu::data_container(&self.menu_state, &self.error));

		// Spacer between menu and content.
		//content = content.push(Space::new(Length::Fixed(0.0), Length::Fixed(DEFAULT_PADDING)));
		match menu_state.mode {
			element::menu::Mode::Wallet => {
				let setup_container =
					element::wallet::data_container(&self.wallet_state, &self.config);
				content = content.push(setup_container)
			}
			element::menu::Mode::Node => {
				let chain_type = self
					.node_interface
					.read()
					.unwrap()
					.chain_type
					.unwrap_or_else(|| ChainTypes::Mainnet);
				let node_container = element::node::data_container(&self.node_state, chain_type);
				content = content.push(node_container)
			}
			element::menu::Mode::About => {
				let about_container = element::about::data_container(&None, &self.about_state);
				content = content.push(about_container)
			}
			element::menu::Mode::Settings => {
				content = content.push(element::settings::data_container(
					&self.settings_state,
					&self.config,
					&self.wallet_settings_state,
					&self.node_settings_state,
					&self.general_settings_state,
				))
				/*if let Some(settings_container) = views.get_mut(settings_view_index) {
					content = content.push(settings_container.view.data_container)
				}*/
			}
		}

		let underlay: Element<Message> = 
        // Wraps everything in a container.
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(grin_gui_core::theme::ContainerStyle::NormalBackground)
            .into();

		let content: Element<Message> = match self.modal_type {
			ModalType::Exit => element::modal::exit_card().into(),
			ModalType::Error => {
				let error_cause = self
					.error
					.as_ref()
					.map_or_else(|| "unknown error".to_owned(), |e| error_cause_string(e));

				element::modal::error_card(error_cause.clone()).into()
			}
		};

		Modal::new(self.show_modal, underlay, content)
			.on_esc(Message::Interaction(Interaction::CloseErrorModal))
			.style(grin_gui_core::theme::ModalStyle::Normal)
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

	#[cfg(target_os = "macos")]
	{
		// false needed for Application shutdown
		settings.exit_on_close_request = false;
	}

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
	let icon = iced_core::window::icon::from_rgba(image.into_raw(), width, height);
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
	ReadSlatepackFromClipboard,
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
	WalletSetupRestoreWalletViewInteraction(
		element::wallet::setup::wallet_restore::LocalViewInteraction,
	),
	WalletSetupImportWalletViewInteraction(
		element::wallet::setup::wallet_import::LocalViewInteraction,
	),
	WalletListWalletViewInteraction(element::wallet::setup::wallet_list::LocalViewInteraction),
	WalletSetupWalletSuccessViewInteraction(
		element::wallet::setup::wallet_success::LocalViewInteraction,
	),
	WalletImportWalletSuccessViewInteraction(
		element::wallet::setup::wallet_import_success::LocalViewInteraction,
	),
	WalletOperationOpenViewInteraction(element::wallet::operation::open::LocalViewInteraction),
	WalletOperationHomeViewInteraction(element::wallet::operation::home::LocalViewInteraction),
	WalletOperationTxListInteraction(element::wallet::operation::tx_list::LocalViewInteraction),
	WalletOperationHomeTxListDisplayInteraction(
		element::wallet::operation::tx_list_display::LocalViewInteraction,
	),
	WalletOperationHomeActionMenuViewInteraction(
		element::wallet::operation::action_menu::LocalViewInteraction,
	),
	WalletOperationCreateTxViewInteraction(
		element::wallet::operation::create_tx::LocalViewInteraction,
	),
	WalletOperationApplyTxViewInteraction(
		element::wallet::operation::apply_tx::LocalViewInteraction,
	),
	WalletOperationApplyTxConfirmViewInteraction(
		element::wallet::operation::apply_tx_confirm::LocalViewInteraction,
	),
	WalletOperationShowSlatepackViewInteraction(
		element::wallet::operation::show_slatepack::LocalViewInteraction,
	),
	WalletOperationTxDetailViewInteraction(
		element::wallet::operation::tx_detail::LocalViewInteraction,
	),
	WalletOperationTxProofViewInteraction(
		element::wallet::operation::tx_proof::LocalViewInteraction,
	),
	WalletOperationTxDoneViewInteraction(element::wallet::operation::tx_done::LocalViewInteraction),
	WalletOperationCreateTxContractsViewInteraction(
		element::wallet::operation::create_tx_contracts::LocalViewInteraction,
	),
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
	SortCatalogColumn(element::wallet::operation::tx_list::ColumnKey),
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

	/// Application shutdown
	Exit,
	ExitCancel,
}

pub struct ThemeState {
	themes: Vec<(String, Theme)>,
	current_theme_name: String,
	// pick_list_state: pick_list::State<String>,
	// input_state: text_input::State,
	input_url: String,
}

impl Default for ThemeState {
	fn default() -> Self {
		let themes = Theme::all();

		ThemeState {
			themes,
			current_theme_name: "Dark".to_string(),
			input_url: Default::default(),
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
				.width = Length::Fixed(*local_version_width);
			grin_gui
				.header_state
				.columns
				.get_mut(2)
				.as_mut()
				.unwrap()
				.width = Length::Fixed(*remote_version_width);
			grin_gui
				.header_state
				.columns
				.get_mut(3)
				.as_mut()
				.unwrap()
				.width = Length::Fixed(*status_width);
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
					a.width = column.width.map_or(Length::Fill, Length::Fixed);
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
						column.width.map_or(Length::Fill, Length::Fixed)
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
						column.width.map_or(Length::Fill, Length::Fixed)
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
						column.width.map_or(Length::Fill, Length::Fixed)
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
