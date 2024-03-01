use crate::{gui::element, log_error};
use iced_core::Widget;
use native_dialog::FileDialog;
use std::path::PathBuf;

use {
	super::super::super::{
		BUTTON_HEIGHT, BUTTON_WIDTH, DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING,
	},
	crate::gui::{GrinGui, Interaction, Message},
	crate::localization::localized_string,
	crate::Result,
	anyhow::Context,
	grin_gui_core::theme::ColorPalette,
	grin_gui_core::theme::{
		Button, Column, Container, Element, PickList, Row, Scrollable, Text, TextInput,
	},
	grin_gui_core::{
		config::Wallet,
		fs::PersistentData,
		node::ChainTypes::{self, Mainnet, Testnet},
		wallet::create_grin_wallet_path,
		wallet::WalletInterface,
	},
	iced::widget::{button, pick_list, scrollable, text_input, Checkbox, Space},
	iced::{alignment, Alignment, Command, Length},
	std::sync::{Arc, RwLock},
};

pub struct StateContainer {
	pub toml_file: PathBuf,
	pub password_state: PasswordState,
	pub display_name_value: String,
}

impl Default for StateContainer {
	fn default() -> Self {
		Self {
			toml_file: Default::default(),
			password_state: Default::default(),
			display_name_value: Default::default(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct PasswordState {
	pub input_value: String,
}

impl Default for PasswordState {
	fn default() -> Self {
		PasswordState {
			input_value: Default::default(),
		}
	}
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
	Back,
	//TODO: ZeroingString these
	PasswordInput(String),
	PasswordInputEnterPressed,
	DisplayName(String),
	ImportWallet(String, PathBuf),
	WalletImportedOk((String, String, String, ChainTypes)),
	WalletImportError(Arc<RwLock<Option<anyhow::Error>>>),
}

pub fn handle_message<'a>(
	grin_gui: &mut GrinGui,
	message: LocalViewInteraction,
) -> Result<Command<Message>> {
	let state = &mut grin_gui.wallet_state.setup_state.setup_wallet_state;
	match message {
		LocalViewInteraction::Back => {
			// reset user input values
			grin_gui.wallet_state.setup_state.import_wallet_state = Default::default();

			// return user to proper view
			match grin_gui.wallet_state.mode {
				// back to init screen
				element::wallet::Mode::Init => {
					grin_gui.wallet_state.setup_state.mode = super::Mode::Init
				}
				_ => {
					// back to list view
					grin_gui.wallet_state.mode = element::wallet::Mode::Init;
					grin_gui.wallet_state.setup_state.mode = super::Mode::ListWallets;
				}
			};
		}
		LocalViewInteraction::PasswordInput(password) => {
			state.password_state.input_value = password;
		}
		LocalViewInteraction::PasswordInputEnterPressed => {
			// state.password_state.input_state.unfocus();
			// state.password_state.repeat_input_state.focus();
		}
		LocalViewInteraction::DisplayName(display_name_value) => {
			state.advanced_options_state.display_name_value = display_name_value;
		}
		LocalViewInteraction::ImportWallet(display_name, toml_file) => {
			grin_gui.error.take();

			log::debug!(
				"setup::wallet::LocalViewInteraction::ImportWallet {}",
				display_name,
			);

			/*let fut = move || {
				WalletInterface::init(
					w,
					password.clone(),
					top_level_directory,
					display_name,
					chain_type,
					recovery_phrase,
				)
			};*/

			/*return Ok(Command::perform(fut(), |r| {
				match r.context("Failed to Create Wallet") {
					Ok(ret) => Message::Interaction(Interaction::WalletSetupImportWalletViewInteraction(
						LocalViewInteraction::WalletImportedOk(ret),
					)),
					Err(e) => Message::Interaction(Interaction::WalletSetupImportWalletViewInteraction(
						LocalViewInteraction::WalletImportError(Arc::new(RwLock::new(Some(e)))),
					)),
				}
			}));*/
		}
		LocalViewInteraction::WalletImportedOk((tld, mnemonic, display_name, chain_type)) => {
			let tld = Some(PathBuf::from(&tld));
			let saved_wallet = Wallet::new(tld, display_name, chain_type);

			let index = grin_gui.config.add_wallet(saved_wallet);
			grin_gui.config.current_wallet_index = Some(index);
			grin_gui.wallet_state.clear_config_missing();
			grin_gui
				.wallet_state
				.setup_state
				.setup_wallet_success_state
				.recovery_phrase = mnemonic;

			// reset user input values
			grin_gui.wallet_state.setup_state.setup_wallet_state = Default::default();

			let _ = grin_gui.config.save();

			grin_gui.wallet_state.setup_state.mode =
				crate::gui::element::wallet::setup::Mode::WalletCreateSuccess;

			if grin_gui.wallet_state.mode != element::wallet::Mode::Init {
				// set init state
				grin_gui.wallet_state.mode = element::wallet::Mode::Init;
			}
		}
		LocalViewInteraction::WalletImportError(err) => {
			grin_gui.error = err.write().unwrap().take();
			if let Some(e) = grin_gui.error.as_ref() {
				log_error(e);
			}
		}
	}

	Ok(Command::none())
}

pub fn data_container<'a>(state: &'a StateContainer) -> Container<'a, Message> {
	let title = Text::new(localized_string("import-grin-wallet-title"))
		.size(DEFAULT_HEADER_FONT_SIZE)
		.horizontal_alignment(alignment::Horizontal::Center);

	// we need 2 pts of padding here to match the position with other views: i.e. wallet list, settings.
	// otherwise this title container will look like it shifts up slightly when the user toggles
	// between views with buttons on the header row.
	let title_container = Container::new(title)
		.style(grin_gui_core::theme::ContainerStyle::BrightBackground)
		.padding(iced::Padding::from([
			2, // top
			0, // right
			2, // bottom
			5, // left
		]));

	// push more items on to header here: e.g. other buttons, things that belong on the header
	let header_row = Row::new().push(title_container);

	let header_container = Container::new(header_row).padding(iced::Padding::from([
		0,                      // top
		0,                      // right
		DEFAULT_PADDING as u16, // bottom
		0,                      // left
	]));

	let password_column = {
		let password_input = TextInput::new(
			&localized_string("password")[..],
			&state.password_state.input_value,
		)
		.on_input(|s| {
			Interaction::WalletSetupImportWalletViewInteraction(
				LocalViewInteraction::PasswordInput(s),
			)
		})
		.on_submit(Interaction::WalletSetupImportWalletViewInteraction(
			LocalViewInteraction::PasswordInputEnterPressed,
		))
		.size(DEFAULT_FONT_SIZE)
		.padding(6)
		.width(Length::Fixed(200.0))
		.style(grin_gui_core::theme::TextInputStyle::AddonsQuery)
		.password();

		let password_input: Element<Interaction> = password_input.into();

		let mut password_input_col = Column::new()
			.push(password_input.map(Message::Interaction))
			.spacing(DEFAULT_PADDING)
			.align_items(Alignment::Start);

		Column::new().push(password_input_col)
	};

	let description = Text::new(localized_string("setup-grin-wallet-enter-password"))
		.size(DEFAULT_FONT_SIZE)
		.horizontal_alignment(alignment::Horizontal::Center);
	let description_container =
		Container::new(description).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

	let display_name = Text::new(localized_string("display-name"))
		.size(DEFAULT_FONT_SIZE)
		.horizontal_alignment(alignment::Horizontal::Left);

	let display_name_container =
		Container::new(display_name).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

	let display_name_input =
		TextInput::new(state.toml_file.to_str().unwrap(), &state.display_name_value)
			.on_input(|s| {
				Interaction::WalletSetupImportWalletViewInteraction(
					LocalViewInteraction::DisplayName(s),
				)
			})
			.size(DEFAULT_FONT_SIZE)
			.padding(6)
			.width(Length::Fixed(200.0))
			.style(grin_gui_core::theme::TextInputStyle::AddonsQuery);

	let display_name_input: Element<Interaction> = display_name_input.into();

	let button_height = Length::Fixed(BUTTON_HEIGHT);
	let button_width = Length::Fixed(BUTTON_WIDTH);

	let submit_button_label_container = Container::new(
		Text::new(localized_string("setup-grin-create-wallet")).size(DEFAULT_FONT_SIZE),
	)
	.width(button_width)
	.height(button_height)
	.center_x()
	.center_y()
	.align_x(alignment::Horizontal::Center);

	let mut submit_button = Button::new(submit_button_label_container)
		.style(grin_gui_core::theme::ButtonStyle::Primary);
	let submit_button: Element<Interaction> = submit_button.into();

	let cancel_button_label_container =
		Container::new(Text::new(localized_string("cancel")).size(DEFAULT_FONT_SIZE))
			.width(button_width)
			.height(button_height)
			.center_x()
			.center_y()
			.align_x(alignment::Horizontal::Center);

	let cancel_button: Element<Interaction> = Button::new(cancel_button_label_container)
		.style(grin_gui_core::theme::ButtonStyle::Primary)
		.on_press(Interaction::WalletSetupImportWalletViewInteraction(
			LocalViewInteraction::Back,
		))
		.into();

	let submit_container = Container::new(submit_button.map(Message::Interaction)).padding(1);
	let submit_container = Container::new(submit_container)
		.style(grin_gui_core::theme::ContainerStyle::Segmented)
		.padding(1);

	let cancel_container = Container::new(cancel_button.map(Message::Interaction)).padding(1);
	let cancel_container = Container::new(cancel_container)
		.style(grin_gui_core::theme::ContainerStyle::Segmented)
		.padding(1);

	let unit_spacing = 15.0;
	let button_row = Row::new()
		.push(submit_container)
		.push(Space::new(Length::Fixed(unit_spacing), Length::Fixed(0.0)))
		.push(cancel_container);

	let mut column = Column::new()
		.push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
		.push(description_container)
		.push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
		.push(display_name_container)
		.push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
		.push(password_column)
		.push(Space::new(
			Length::Fixed(0.0),
			Length::Fixed(unit_spacing + 10.0),
		));

	column = column.push(button_row).align_items(Alignment::Start);
	let form_container = Container::new(column)
		.width(Length::Fill)
		.padding(iced::Padding::from([
			0, // top
			0, // right
			0, // bottom
			5, // left
		]));

	// form container should be scrollable in tiny windows
	let scrollable = Scrollable::new(form_container)
		.height(Length::Fill)
		.style(grin_gui_core::theme::ScrollableStyle::Primary);

	let content = Container::new(scrollable)
		.width(Length::Fill)
		.height(Length::Shrink)
		.style(grin_gui_core::theme::ContainerStyle::NormalBackground);

	let wrapper_column = Column::new()
		.height(Length::Fill)
		.push(header_container)
		.push(content);

	// Returns the final container.
	Container::new(wrapper_column).padding(iced::Padding::from([
		DEFAULT_PADDING, // top
		DEFAULT_PADDING, // right
		DEFAULT_PADDING, // bottom
		DEFAULT_PADDING, // left
	]))
}
