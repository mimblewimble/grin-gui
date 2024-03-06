pub mod init;
pub mod wallet_import;
pub mod wallet_import_success;
pub mod wallet_list;
pub mod wallet_setup;
pub mod wallet_success;

use {
	crate::gui::{GrinGui, Message},
	crate::Result,
	grin_gui_core::config::Config,
	grin_gui_core::theme::ColorPalette,
	grin_gui_core::theme::{
		Column, Container, Element, PickList, Row, Scrollable, Text, TextInput,
	},
	iced::widget::Space,
	iced::{Command, Length},
};

pub struct StateContainer {
	pub mode: Mode,
	pub setup_init_state: init::StateContainer,
	pub setup_wallet_state: wallet_setup::StateContainer,
	pub import_wallet_state: wallet_import::StateContainer,
	pub setup_wallet_success_state: wallet_success::StateContainer,
	pub import_wallet_success_state: wallet_import_success::StateContainer,
	pub setup_wallet_list_state: wallet_list::StateContainer,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
	Init,
	CreateWallet(String),
	ListWallets,
	WalletCreateSuccess,
	WalletImportSuccess,
}

impl Default for StateContainer {
	fn default() -> Self {
		Self {
			mode: Mode::Init,
			setup_init_state: Default::default(),
			setup_wallet_state: Default::default(),
			import_wallet_state: Default::default(),
			setup_wallet_success_state: Default::default(),
			import_wallet_success_state: Default::default(),
			setup_wallet_list_state: Default::default(),
		}
	}
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {}

pub fn handle_message(
	grin_gui: &mut GrinGui,
	message: LocalViewInteraction,
) -> Result<Command<Message>> {
	Ok(Command::none())
}

pub fn data_container<'a>(state: &'a StateContainer, config: &Config) -> Container<'a, Message> {
	let content = match &state.mode {
		Mode::Init => init::data_container(),
		Mode::CreateWallet(default_display_name) => {
			wallet_setup::data_container(&state.setup_wallet_state, default_display_name)
		}
		Mode::WalletCreateSuccess => {
			wallet_success::data_container(&state.setup_wallet_success_state)
		}
		Mode::WalletImportSuccess => {
			wallet_import_success::data_container(&state.import_wallet_success_state)
		}
		Mode::ListWallets => wallet_list::data_container(&state.setup_wallet_list_state, config),
	};

	Container::new(content)
		.center_y()
		.center_x()
		.width(Length::Fill)
		.style(grin_gui_core::theme::ContainerStyle::NormalBackground)
}
