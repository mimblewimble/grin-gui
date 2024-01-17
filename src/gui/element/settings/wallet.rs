use {
	super::DEFAULT_FONT_SIZE,
	crate::gui::{GrinGui, Interaction, Message},
	crate::localization::localized_string,
	grin_gui_core::config::{Config, TxMethod},
	grin_gui_core::fs::PersistentData,
	grin_gui_core::theme::{
		Button, Column, Container, Element, PickList, Row, Scrollable, Text, TextInput,
	},
	iced::widget::{button, pick_list, scrollable, text_input, Checkbox, Space},
	iced::Length,
	iced::{alignment, Alignment},
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone)]
pub struct StateContainer {
	// scrollable_state: scrollable::State,
	mw_mixnet_address_1: String,
	mw_mixnet_address_2: String,
	mw_mixnet_address_3: String,
}

impl Default for StateContainer {
	fn default() -> Self {
		Self {
			mw_mixnet_address_1: "".to_string(),
			mw_mixnet_address_2: "".to_string(),
			mw_mixnet_address_3: "".to_string(),
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LocalViewInteraction {
	TxMethodSelected(TxMethod),
	MwMixnetAddress1Changed(String),
	MwMixnetAddress2Changed(String),
	MwMixnetAddress3Changed(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Mode {
	Wallet,
	Node,
	General,
}

pub fn handle_message(grin_gui: &mut GrinGui, message: LocalViewInteraction) {
	let state = &mut grin_gui.wallet_settings_state;
	let mut check_mixnet_config = || {
		if grin_gui.config.mixnet_keys.is_none() {
			grin_gui.config.mixnet_keys = Some(vec![]);
			grin_gui
				.config
				.mixnet_keys
				.as_mut()
				.unwrap()
				.push(String::new());
			grin_gui
				.config
				.mixnet_keys
				.as_mut()
				.unwrap()
				.push(String::new());
			grin_gui
				.config
				.mixnet_keys
				.as_mut()
				.unwrap()
				.push(String::new());
		}
	};
	match message {
		LocalViewInteraction::TxMethodSelected(method) => {
			log::debug!("Interaction::TxMethodSelectedSettings({:?})", method);
			// Set Mode
			grin_gui.config.tx_method = method;
			let _ = grin_gui.config.save();
		}
		LocalViewInteraction::MwMixnetAddress1Changed(value) => {
			check_mixnet_config();
			grin_gui.config.mixnet_keys.as_mut().unwrap()[0] = value.clone();
			state.mw_mixnet_address_1 = value;
			let _ = grin_gui.config.save();
		}
		LocalViewInteraction::MwMixnetAddress2Changed(value) => {
			check_mixnet_config();
			grin_gui.config.mixnet_keys.as_mut().unwrap()[1] = value.clone();
			state.mw_mixnet_address_2 = value;
			let _ = grin_gui.config.save();
		}
		LocalViewInteraction::MwMixnetAddress3Changed(value) => {
			check_mixnet_config();
			grin_gui.config.mixnet_keys.as_mut().unwrap()[2] = value.clone();
			state.mw_mixnet_address_3 = value;
			let _ = grin_gui.config.save();
		}
	}
}

pub fn data_container<'a>(state: &'a StateContainer, config: &Config) -> Container<'a, Message> {
	let (config_addr_1, config_addr_2, config_addr_3) = if let Some(a) = config.mixnet_keys.as_ref()
	{
		(a[0].clone(), a[1].clone(), a[2].clone())
	} else {
		(String::new(), String::new(), String::new())
	};

	let tx_method_column = {
		let tx_method_container =
			Container::new(Text::new(localized_string("tx-method")).size(DEFAULT_FONT_SIZE))
				.style(grin_gui_core::theme::ContainerStyle::NormalBackground);

		let tx_method_pick_list = PickList::new(&TxMethod::ALL[..], Some(config.tx_method), |t| {
			Message::Interaction(Interaction::WalletSettingsViewInteraction(
				LocalViewInteraction::TxMethodSelected(t),
			))
		})
		.text_size(DEFAULT_FONT_SIZE)
		.width(Length::Fixed(120.0))
		.style(grin_gui_core::theme::PickListStyle::Primary);

		// Data row for theme picker list.
		let tx_method_data_row = Row::new()
			.push(tx_method_pick_list)
			.align_items(Alignment::Center)
			.height(Length::Fixed(26.0));

		Column::new()
			.push(tx_method_container)
			.push(Space::new(Length::Fixed(0.0), Length::Fixed(5.0)))
			.push(tx_method_data_row)
	};

	let mw_mixnet_address_column = {
		let mw_mixnet_address_container = Container::new(
			Text::new(localized_string("mw-mixnet-addresses")).size(DEFAULT_FONT_SIZE),
		)
		.style(grin_gui_core::theme::ContainerStyle::NormalBackground);

		let mw_mixnet_address_1 = Text::new(localized_string("mw-mixnet-address-1"))
			.size(DEFAULT_FONT_SIZE)
			.horizontal_alignment(alignment::Horizontal::Left);

		let mw_mixnet_address_1_container = Container::new(mw_mixnet_address_1)
			.style(grin_gui_core::theme::ContainerStyle::NormalBackground);

		let mw_mixnet_address_1_input = TextInput::new("", &config_addr_1)
			.on_input(|s| {
				Interaction::WalletSettingsViewInteraction(
					LocalViewInteraction::MwMixnetAddress1Changed(s),
				)
			})
			.size(DEFAULT_FONT_SIZE)
			.padding(6)
			.width(Length::Fixed(400.0))
			.style(grin_gui_core::theme::TextInputStyle::AddonsQuery);

		let mw_mixnet_address_1_input: Element<Interaction> = mw_mixnet_address_1_input.into();

		let mw_mixnet_address_2 = Text::new(localized_string("mw-mixnet-address-2"))
			.size(DEFAULT_FONT_SIZE)
			.horizontal_alignment(alignment::Horizontal::Left);

		let mw_mixnet_address_2_container = Container::new(mw_mixnet_address_2)
			.style(grin_gui_core::theme::ContainerStyle::NormalBackground);

		let mw_mixnet_address_2_input = TextInput::new("", &config_addr_2)
			.on_input(|s| {
				Interaction::WalletSettingsViewInteraction(
					LocalViewInteraction::MwMixnetAddress2Changed(s),
				)
			})
			.size(DEFAULT_FONT_SIZE)
			.padding(6)
			.width(Length::Fixed(400.0))
			.style(grin_gui_core::theme::TextInputStyle::AddonsQuery);

		let mw_mixnet_address_2_input: Element<Interaction> = mw_mixnet_address_2_input.into();

		let mw_mixnet_address_3 = Text::new(localized_string("mw-mixnet-address-3"))
			.size(DEFAULT_FONT_SIZE)
			.horizontal_alignment(alignment::Horizontal::Left);

		let mw_mixnet_address_3_container = Container::new(mw_mixnet_address_3)
			.style(grin_gui_core::theme::ContainerStyle::NormalBackground);

		let mw_mixnet_address_3_input = TextInput::new("", &config_addr_3)
			.on_input(|s| {
				Interaction::WalletSettingsViewInteraction(
					LocalViewInteraction::MwMixnetAddress3Changed(s),
				)
			})
			.size(DEFAULT_FONT_SIZE)
			.padding(6)
			.width(Length::Fixed(400.0))
			.style(grin_gui_core::theme::TextInputStyle::AddonsQuery);

		let mw_mixnet_address_3_input: Element<Interaction> = mw_mixnet_address_3_input.into();

		// Data row for theme picker list.
		/*let tx_method_data_row = Row::new()
		.push(tx_method_pick_list)
		.align_items(Alignment::Center)
		.height(Length::Fixed(26.0));*/

		Column::new()
			.push(mw_mixnet_address_container)
			.push(Space::new(Length::Fixed(0.0), Length::Fixed(5.0)))
			.push(mw_mixnet_address_1_container)
			.push(Space::new(Length::Fixed(0.0), Length::Fixed(5.0)))
			.push(mw_mixnet_address_1_input.map(Message::Interaction))
			.push(Space::new(Length::Fixed(0.0), Length::Fixed(5.0)))
			.push(mw_mixnet_address_2_container)
			.push(Space::new(Length::Fixed(0.0), Length::Fixed(5.0)))
			.push(mw_mixnet_address_2_input.map(Message::Interaction))
			.push(Space::new(Length::Fixed(0.0), Length::Fixed(5.0)))
			.push(mw_mixnet_address_3_container)
			.push(Space::new(Length::Fixed(0.0), Length::Fixed(5.0)))
			.push(mw_mixnet_address_3_input.map(Message::Interaction))
	};

	let wrap = {
		Column::new()
			.push(tx_method_column)
			.push(Space::new(Length::Fixed(0.0), Length::Fixed(10.0)))
			.push(mw_mixnet_address_column)
	};

	let scrollable = Scrollable::new(wrap)
		.height(Length::Fill)
		.style(grin_gui_core::theme::ScrollableStyle::Primary);

	let col = Column::new()
		.push(Space::new(Length::Fixed(0.0), Length::Fixed(10.0)))
		.push(scrollable)
		.push(Space::new(Length::Fixed(0.0), Length::Fixed(20.0)));
	let row = Row::new()
		.push(Space::new(Length::Fixed(5.0), Length::Fixed(0.0)))
		.push(col);

	// Returns the final container.
	Container::new(row)
		.width(Length::Fill)
		.height(Length::Shrink)
		.style(grin_gui_core::theme::ContainerStyle::NormalBackground)
}
