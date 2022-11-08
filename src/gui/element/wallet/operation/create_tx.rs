use super::tx_list::{self, ExpandType};
use crate::log_error;
use async_std::prelude::FutureExt;
use grin_gui_core::{
    config::Config,
    wallet::{TxLogEntry, TxLogEntryType},
};
use grin_gui_widgets::{header, Header, TableRow};
use iced::button::StyleSheet;
use iced_aw::Card;
use iced_native::Widget;
use std::path::PathBuf;

use super::tx_list::{HeaderState, TxList};

use {
    super::super::super::{
        DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING, SMALLER_FONT_SIZE,
    },
    crate::gui::{style, GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    anyhow::Context,
    grin_gui_core::wallet::{StatusMessage, WalletInfo, WalletInterface, InitTxArgs, Slate},
    grin_gui_core::{node::amount_to_hr_string, theme::ColorPalette},
    iced::{
        alignment, button, scrollable, text_input, Alignment, Button, Checkbox, Column, Command,
        Container, Element, Length, Row, Scrollable, Space, Text, TextInput,
    },
    serde::{Deserialize, Serialize},
    std::sync::{Arc, RwLock},
};

pub struct StateContainer {
    pub back_button_state: button::State,
    pub recipient_address_input_state: text_input::State,
    pub submit_button_state: button::State,
    pub recipient_address_value: String,
    pub amount_input_state: text_input::State,
    pub amount_value: String,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            back_button_state: Default::default(),
            recipient_address_input_state: Default::default(),
            submit_button_state: Default::default(),
            recipient_address_value: Default::default(),
            amount_input_state: Default::default(),
            amount_value: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    Back,
    RecipientAddress(String),
    Amount(String),
    CreateTransaction(),
    TxCreatedOk(Slate),
    TxCreateError(Arc<RwLock<Option<anyhow::Error>>>),
}

pub fn handle_message<'a>(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let state = &mut grin_gui.wallet_state.operation_state.create_tx_state;

    match message {
        LocalViewInteraction::Back => {
            log::debug!("Interaction::WalletOperationCreateTxViewInteraction(Back)");
            grin_gui.wallet_state.operation_state.mode =
                crate::gui::element::wallet::operation::Mode::Home;
        }
        LocalViewInteraction::RecipientAddress(s) => {
            state.recipient_address_value = s;
        }
        LocalViewInteraction::Amount(s) => {
            state.amount_value = s;
        }
        LocalViewInteraction::CreateTransaction() => {
            grin_gui.error.take();

            log::debug!("Interaction::WalletOperationCreateTxViewInteraction");

            let w = grin_gui.wallet_interface.clone();

            // Todo: Amount parsing + validation, just testing the flow for now
            let args = InitTxArgs {
                src_acct_name: None,
                amount: 1_000_000_000,
                minimum_confirmations: 2,
                max_outputs: 500,
                num_change_outputs: 1,
                selection_strategy_is_use_all: false,
                late_lock: Some(false),
                ..Default::default()
            };

            let fut = move || WalletInterface::create_tx(w, args);

            return Ok(Command::perform(fut(), |r| {
                match r.context("Failed to Create Transaction") {
                    Ok(ret) => Message::Interaction(Interaction::WalletOperationCreateTxViewInteraction(
                        LocalViewInteraction::TxCreatedOk(ret),
                    )),
                    Err(e) => Message::Interaction(Interaction::WalletOperationCreateTxViewInteraction(
                        LocalViewInteraction::TxCreateError(Arc::new(RwLock::new(Some(e)))),
                    )),
                }
            }));
        },
        LocalViewInteraction::TxCreatedOk(slate) => {
            log::debug!("{:?}", slate);
        }
        LocalViewInteraction::TxCreateError(err) => {
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
    config: &'a Config,
    state: &'a mut StateContainer,
) -> Container<'a, Message> {
    // Title row
    let title = Text::new(localized_string("create-tx"))
        .size(DEFAULT_HEADER_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);

    let title_container =
        Container::new(title).style(style::BrightBackgroundContainer(color_palette));

    let back_button_label_container =
        Container::new(Text::new(localized_string("back")).size(DEFAULT_FONT_SIZE))
            .height(Length::Units(20))
            .align_y(alignment::Vertical::Bottom)
            .align_x(alignment::Horizontal::Center);

    let back_button: Element<Interaction> =
        Button::new(&mut state.back_button_state, back_button_label_container)
            .style(style::NormalTextButton(color_palette))
            .on_press(Interaction::WalletOperationCreateTxViewInteraction(
                LocalViewInteraction::Back,
            ))
            .into();

    let title_row = Row::new()
        .push(title_container)
        .push(back_button.map(Message::Interaction))
        //.push(Space::new(Length::Fill, Length::Units(0)))
        .align_items(Alignment::Center)
        .padding(6)
        .spacing(20);

    let recipient_address = Text::new(localized_string("recipient-address"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let recipient_address_container =
        Container::new(recipient_address).style(style::NormalBackgroundContainer(color_palette));

    let recipient_address_input = TextInput::new(
        &mut state.recipient_address_input_state,
        "",
        &state.recipient_address_value,
        |s| {
            Interaction::WalletOperationCreateTxViewInteraction(
                LocalViewInteraction::RecipientAddress(s),
            )
        },
    )
    .size(DEFAULT_FONT_SIZE)
    .padding(6)
    .width(Length::Units(400))
    .style(style::AddonsQueryInput(color_palette));

    let recipient_address_input: Element<Interaction> = recipient_address_input.into();

    let amount = Text::new(localized_string("create-tx-amount"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let amount_container =
        Container::new(amount).style(style::NormalBackgroundContainer(color_palette));

    let amount_input = TextInput::new(
        &mut state.amount_input_state,
        "",
        &state.amount_value,
        |s| Interaction::WalletOperationCreateTxViewInteraction(LocalViewInteraction::Amount(s)),
    )
    .size(DEFAULT_FONT_SIZE)
    .padding(6)
    .width(Length::Units(100))
    .style(style::AddonsQueryInput(color_palette));

    let amount_input: Element<Interaction> = amount_input.into();

    let address_instruction_container =
        Text::new(localized_string("recipient-address-instruction"))
            .size(SMALLER_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

    let address_instruction_container = Container::new(address_instruction_container)
        .style(style::NormalBackgroundContainer(color_palette));

    let submit_button_label_container =
        Container::new(Text::new(localized_string("create-transaction")).size(DEFAULT_FONT_SIZE))
            .center_x()
            .align_x(alignment::Horizontal::Center);

    let mut submit_button = Button::new(
        &mut state.submit_button_state,
        submit_button_label_container,
    )
    .style(style::DefaultBoxedButton(color_palette));

    submit_button = submit_button.on_press(Interaction::WalletOperationCreateTxViewInteraction(
        LocalViewInteraction::CreateTransaction(),
    ));

    let submit_button: Element<Interaction> = submit_button.into();

    let column = Column::new()
        .push(title_row)
        .push(recipient_address_container)
        .push(address_instruction_container)
        .push(recipient_address_input.map(Message::Interaction))
        .push(amount_container)
        .push(amount_input.map(Message::Interaction))
        .push(submit_button.map(Message::Interaction))
        .spacing(DEFAULT_PADDING)
        .align_items(Alignment::Start);

    Container::new(column)
        .center_y()
        .center_x()
        .width(Length::Fill)
}
