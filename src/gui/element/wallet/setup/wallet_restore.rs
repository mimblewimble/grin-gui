use {
	super::super::super::{
		DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING, SMALLER_FONT_SIZE,
	},
	crate::gui::{GrinGui, Interaction, Message},
	crate::localization::localized_string,
	crate::{log_error, Result},
	anyhow::Context,
	grin_gui_core::theme::ColorPalette,
	grin_gui_core::theme::{
		Column, Container, Element, PickList, Row, Scrollable, Text, TextInput,
	},
	grin_gui_core::wallet::WalletInterface,
	grin_gui_core::{
		config::{Config, Wallet},
		fs::PersistentData,
		node::ChainTypes::{self, Mainnet, Testnet},
	},
	iced::widget::{button, pick_list, scrollable, text_input, Button, Checkbox, Space},
	iced::{alignment, Alignment, Command, Length},
	iced_aw::Card,
	std::path::PathBuf,
	std::sync::{Arc, RwLock},
};

/*
TODO:
- search help in the wordlist
- card text input
*/

#[derive(Debug)]
pub struct StateContainer {
	pub password: String,
	pub top_level_directory: PathBuf,
	pub display_name: String,
	pub recovery_phrase_state: SeedState,
	pub copy_button_state: button::State,
	pub next_button_state: button::State,
	pub is_valid: bool, // TODO: ZeroingString this
}

impl Default for StateContainer {
	fn default() -> Self {
		Self {
			password: Default::default(),
			top_level_directory: Default::default(),
			display_name: Default::default(),
			recovery_phrase_state: Default::default(),
			copy_button_state: Default::default(),
			next_button_state: Default::default(),
			is_valid: false,
		}
	}
}
#[derive(Debug)]
pub struct SeedState {
	input_state: String,
	input_value: String,
}

impl Default for SeedState {
	fn default() -> Self {
		SeedState {
			input_state: Default::default(),
			input_value: Default::default(),
		}
	}
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
	SeedInput(String),
	SeedInputEnterPressed,
	Submit,
	WalletMnemonicOk,
	WalletMnemonicError(Arc<RwLock<Option<anyhow::Error>>>),
	WalletCreatedOk,
	WalletCreateError(Arc<RwLock<Option<anyhow::Error>>>),
}

pub fn handle_message(
	grin_gui: &mut GrinGui,
	message: LocalViewInteraction,
) -> Result<Command<Message>> {
	let state = &mut grin_gui.wallet_state.setup_state.setup_wallet_restore_state;
	match message {
		LocalViewInteraction::SeedInput(seed) => {
			state.recovery_phrase_state.input_value = seed.clone();
			let words = seed.split_whitespace();
			let vec_words: Vec<&str> = words.collect();
			let size_seed = vec_words.len();
			if size_seed == 12 || size_seed == 24 {
				let w = grin_gui.wallet_interface.clone();
				let pass = state.recovery_phrase_state.input_value.clone();

				let fut = move || WalletInterface::validate_mnemonic(w, pass);

				return Ok(Command::perform(fut(), |r| {
					match r.context("Failed to Restore Wallet From Seed") {
						Ok(ret) => Message::Interaction(
							Interaction::WalletSetupRestoreWalletViewInteraction(
								LocalViewInteraction::WalletMnemonicOk,
							),
						),
						Err(e) => Message::Interaction(
							Interaction::WalletSetupRestoreWalletViewInteraction(
								LocalViewInteraction::WalletMnemonicError(Arc::new(RwLock::new(
									Some(e),
								))),
							),
						),
					}
				}));
			}
		}

		LocalViewInteraction::WalletMnemonicOk => {
			state.is_valid = true;
		}
		LocalViewInteraction::SeedInputEnterPressed => {
			//state.recovery_phrase_state.input_state.unfocus();
		}
		LocalViewInteraction::Submit => {
			let password = state.password.clone();
			let w = grin_gui.wallet_interface.clone();

			/*let fut = move || {
				WalletInterface::init(
					w,
					password,
					state.top_level_directory.clone(),
					state.display_name.clone(),
					32,
				)
			};*/
		}
		LocalViewInteraction::WalletCreatedOk => {
			grin_gui.wallet_state.setup_state.mode =
				crate::gui::element::wallet::setup::Mode::WalletCreateSuccess;
			let _ = grin_gui.config.save();
		}
		LocalViewInteraction::WalletMnemonicError(err) => {
			state.is_valid = false;
			grin_gui.error = err.write().unwrap().take();
			if let Some(e) = grin_gui.error.as_ref() {
				log_error(e);
			}
		}
		LocalViewInteraction::WalletCreateError(err) => {
			grin_gui.error = err.write().unwrap().take();
			if let Some(e) = grin_gui.error.as_ref() {
				log_error(e);
			}
		}
	}
	Ok(Command::none())
}

pub fn data_container<'a>(
	color_palette: ColorPalette,
	state: &'a mut StateContainer,
) -> Container<'a, Message> {
	// Title row
	let title = Text::new(localized_string("setup-grin-wallet-success"))
		.size(DEFAULT_HEADER_FONT_SIZE)
		.horizontal_alignment(alignment::Horizontal::Left);

	let title_container =
		Container::new(title).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

	let title_row = Row::new()
		.push(title_container)
		.align_items(Alignment::Center)
		.spacing(20);

	let description = Text::new(localized_string("setup-grin-wallet-recovery-phrase"))
		.size(DEFAULT_FONT_SIZE)
		.horizontal_alignment(alignment::Horizontal::Center);
	let description_container =
		Container::new(description).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

	let recovery_phrase_column = {
		let recovery_phrase_input = TextInput::new(
			&localized_string("password")[..],
			&state.recovery_phrase_state.input_state,
		)
		.on_input(|s| {
			Interaction::WalletSetupRestoreWalletViewInteraction(LocalViewInteraction::SeedInput(s))
		})
		.on_submit(Interaction::WalletSetupRestoreWalletViewInteraction(
			LocalViewInteraction::SeedInputEnterPressed,
		))
		.size(DEFAULT_FONT_SIZE)
		.padding(6)
		.width(Length::Fixed(400.0))
		.style(grin_gui_core::theme::TextInputStyle::AddonsQuery);

		let recovery_phrase_input: Element<Interaction> = recovery_phrase_input.into();

		let recovery_phrase_input_col = Column::new()
			.push(recovery_phrase_input.map(Message::Interaction))
			.spacing(DEFAULT_PADDING)
			.align_items(Alignment::Center);

		/*let check_seed = || {
			!state.recovery_phrase_state.input_value.is_empty() && state.is_valid
		};

		let mut seed_entry_value = localized_string("");
		if check_seed() {
			seed_entry_value = localized_string("setup-grin-passwords-okay")
		}

		let seed_entry_status = Text::new(seed_entry_value)
				.size(DEFAULT_FONT_SIZE)
				.horizontal_alignment(alignment::Horizontal::Left);

		let seed_entry_status_container = Container::new(seed_entry_status)
			//.width(Length::Fill)
			.style(style::NormalSuccessBackgroundContainer(color_palette));*/

		Column::new().push(recovery_phrase_input_col) //.push(seed_entry_status_container)
	};

	let submit_button_label_container = Container::new(
		Text::new(localized_string("setup-grin-wallet-done")).size(DEFAULT_FONT_SIZE),
	)
	.center_x()
	.align_x(alignment::Horizontal::Center);

	let mut next_button = Button::new(submit_button_label_container)
		.style(grin_gui_core::theme::ButtonStyle::Primary);

	if state.is_valid {
		next_button = next_button.on_press(Interaction::WalletSetupRestoreWalletViewInteraction(
			LocalViewInteraction::Submit,
		));
	}

	let next_button: Element<Interaction> = next_button.into();

	let unit_spacing = 15;

	let colum = Column::new()
		.push(title_row)
		.push(Space::new(
			Length::Fixed(0.0),
			Length::Fixed(unit_spacing as f32 + 5.0),
		))
		.push(description_container)
		.push(Space::new(
			Length::Fixed(0.0),
			Length::Fixed(unit_spacing as f32 + 5.0),
		))
		.push(recovery_phrase_column)
		.push(Space::new(
			Length::Fixed(0.0),
			Length::Fixed(unit_spacing as f32 + 10.0),
		))
		.push(next_button.map(Message::Interaction))
		.align_items(Alignment::Center);

	Container::new(colum)
		.center_y()
		.center_x()
		.width(Length::Fill)
}