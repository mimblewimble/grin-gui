use super::tx_list::{self, ExpandType};
use crate::log_error;
use async_std::prelude::FutureExt;
use grin_gui_core::{
	config::Config,
	error::GrinWalletInterfaceError,
	wallet::{InvoiceProof, SlatepackAddress, TxLogEntry, TxLogEntryType},
};
use iced_aw::Card;
use iced_core::Widget;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use super::tx_list::{HeaderState, TxList};

use {
	super::super::super::{
		BUTTON_HEIGHT, BUTTON_WIDTH, DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING,
		SMALLER_FONT_SIZE,
	},
	crate::gui::{GrinGui, Interaction, Message},
	crate::localization::localized_string,
	crate::Result,
	anyhow::Context,
	grin_gui_core::theme::{
		Button, Column, Container, Element, PickList, Row, Scrollable, Text, TextInput,
	},
	grin_gui_core::wallet::{InitTxArgs, Slate, StatusMessage, WalletInfo, WalletInterface},
	grin_gui_core::{
		node::{amount_from_hr_string, amount_to_hr_string},
		theme::{ButtonStyle, ColorPalette, ContainerStyle},
	},
	iced::widget::{button, pick_list, scrollable, text_input, Checkbox, Space},
	iced::{alignment, Alignment, Command, Length},
	serde::{Deserialize, Serialize},
	std::sync::{Arc, RwLock},
};

pub struct StateContainer {
	// Transaction that we're viewing
	pub current_tx: Option<TxLogEntry>,
	pub current_proof: Option<InvoiceProof>,
}

impl Default for StateContainer {
	fn default() -> Self {
		Self {
			current_tx: Default::default(),
			current_proof: Default::default(),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
	Back,
}

pub fn handle_message<'a>(
	grin_gui: &mut GrinGui,
	message: LocalViewInteraction,
) -> Result<Command<Message>> {
	let state = &mut grin_gui.wallet_state.operation_state.create_tx_state;

	match message {
		LocalViewInteraction::Back => {
			log::debug!("Interaction::WalletOperationTxProofViewInteraction(Back)");
			grin_gui.wallet_state.operation_state.mode =
				crate::gui::element::wallet::operation::Mode::Home;
		}
	}

	Ok(Command::none())
}

pub fn data_container<'a>(config: &'a Config, state: &'a StateContainer) -> Container<'a, Message> {
	// Title row
	let title = Text::new(localized_string("tx-proof-title"))
		.size(DEFAULT_HEADER_FONT_SIZE)
		.horizontal_alignment(alignment::Horizontal::Center);

	let title_container = Container::new(title)
		.style(grin_gui_core::theme::ContainerStyle::BrightBackground)
		.padding(iced::Padding::from([
			2, // top
			0, // right
			2, // bottom
			5, // left
		]));

	let header_row = Row::new().push(title_container);

	let header_container = Container::new(header_row).padding(iced::Padding::from([
		0,                      // top
		0,                      // right
		DEFAULT_PADDING as u16, // bottom
		0,                      // left
	]));

	let unit_spacing = 15.0;
	let row_spacing = 5.0;

	let button_height = Length::Fixed(BUTTON_HEIGHT);
	let button_width = Length::Fixed(BUTTON_WIDTH);

	let mut column = Column::new();

	if let Some(ref proof) = state.current_proof {
		// Amount
		let pr_amount_label = Text::new(format!("{}:  ", localized_string("pr-amount")))
			.size(DEFAULT_FONT_SIZE)
			.horizontal_alignment(alignment::Horizontal::Left);

		let pr_amount_label_container = Container::new(pr_amount_label)
			.style(grin_gui_core::theme::ContainerStyle::NormalBackground);

		let pr_amount_value = Text::new(format!("{}", amount_to_hr_string(proof.amount, true)))
			.size(DEFAULT_FONT_SIZE)
			.horizontal_alignment(alignment::Horizontal::Left);

		let pr_amount_value_container = Container::new(pr_amount_value)
			.style(grin_gui_core::theme::ContainerStyle::NormalBackground);

		let pr_amount_row = Row::new()
			.push(pr_amount_label_container)
			.push(pr_amount_value_container);
		column = column
			.push(pr_amount_row)
			.push(Space::new(Length::Fixed(0.0), Length::Fixed(row_spacing)));

		// Timestamp
		let pr_timestamp_label = Text::new(format!("{}:  ", localized_string("pr-timestamp")))
			.size(DEFAULT_FONT_SIZE)
			.horizontal_alignment(alignment::Horizontal::Left);

		let pr_timestamp_label_container = Container::new(pr_timestamp_label)
			.style(grin_gui_core::theme::ContainerStyle::NormalBackground);

		// convert i64 timestamp to utc time
		let ts_display = chrono::NaiveDateTime::from_timestamp(proof.timestamp, 0)
			.format("%Y-%m-%d %H:%M:%S")
			.to_string();

		let pr_timestamp_value = Text::new(format!("{} UTC", ts_display))
			.size(DEFAULT_FONT_SIZE)
			.horizontal_alignment(alignment::Horizontal::Left);

		let pr_timestamp_value_container = Container::new(pr_timestamp_value)
			.style(grin_gui_core::theme::ContainerStyle::NormalBackground);

		let pr_timestamp_row = Row::new()
			.push(pr_timestamp_label_container)
			.push(pr_timestamp_value_container);
		column = column
			.push(pr_timestamp_row)
			.push(Space::new(Length::Fixed(0.0), Length::Fixed(row_spacing)));

		// sender address
		let pr_sender_address_label =
			Text::new(format!("{}:  ", localized_string("pr-sender-address")))
				.size(DEFAULT_FONT_SIZE)
				.horizontal_alignment(alignment::Horizontal::Left);

		let pr_sender_address_label_container = Container::new(pr_sender_address_label)
			.style(grin_gui_core::theme::ContainerStyle::NormalBackground);

		let pr_sender_address_value =
			Text::new(format!("{}", SlatepackAddress::new(&proof.sender_address)))
				.size(DEFAULT_FONT_SIZE)
				.horizontal_alignment(alignment::Horizontal::Left);

		let pr_sender_address_value_container = Container::new(pr_sender_address_value)
			.style(grin_gui_core::theme::ContainerStyle::NormalBackground);

		let pr_sender_address_row = Row::new()
			.push(pr_sender_address_label_container)
			.push(pr_sender_address_value_container);
		column = column
			.push(pr_sender_address_row)
			.push(Space::new(Length::Fixed(0.0), Length::Fixed(row_spacing)))
			.push(Space::new(Length::Fixed(0.0), Length::Fixed(row_spacing)));

		let card_contents = format!("{}", serde_json::to_string_pretty(&proof).unwrap());

		let json_proof_card = Card::new(
			Text::new(localized_string("pr-json-proof")).size(DEFAULT_HEADER_FONT_SIZE),
			Text::new(card_contents.clone()).size(DEFAULT_FONT_SIZE),
		)
		.foot(
			Column::new()
				.spacing(10)
				.padding(5)
				.width(Length::Fill)
				.align_items(Alignment::Center)
				.push(
					Button::new(
						Text::new(localized_string("copy-to-clipboard"))
							.size(SMALLER_FONT_SIZE)
							.horizontal_alignment(alignment::Horizontal::Center),
					)
					.style(grin_gui_core::theme::ButtonStyle::NormalText)
					.on_press(Message::Interaction(Interaction::WriteToClipboard(
						card_contents.clone(),
					))),
				),
		)
		.max_width(400.0)
		.style(grin_gui_core::theme::CardStyle::Normal);

		let json_proof_row = Row::new().push(json_proof_card);

		column = column
			.push(json_proof_row)
			.push(Space::new(Length::Fixed(0.0), Length::Fixed(row_spacing)));
	}

	let cancel_button_label_container =
		Container::new(Text::new(localized_string("back")).size(DEFAULT_FONT_SIZE))
			.width(button_width)
			.height(button_height)
			.center_x()
			.center_y()
			.align_x(alignment::Horizontal::Center);

	let cancel_button: Element<Interaction> = Button::new(cancel_button_label_container)
		.style(grin_gui_core::theme::ButtonStyle::Primary)
		.on_press(Interaction::WalletOperationTxProofViewInteraction(
			LocalViewInteraction::Back,
		))
		.into();

	let cancel_container = Container::new(cancel_button.map(Message::Interaction)).padding(1);
	let cancel_container = Container::new(cancel_container)
		.style(grin_gui_core::theme::ContainerStyle::Segmented)
		.padding(1);

	let button_row = Row::new()
		.push(cancel_container)
		.push(Space::new(Length::Fixed(unit_spacing), Length::Fixed(0.0)));

	column = column.push(button_row);

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
