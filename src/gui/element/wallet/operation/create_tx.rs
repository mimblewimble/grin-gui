use super::tx_list::{self, ExpandType};
use crate::log_error;
use async_std::prelude::FutureExt;
use grin_gui_core::{
    config::Config,
    error::GrinWalletInterfaceError,
    wallet::{TxLogEntry, TxLogEntryType},
};
use grin_gui_widgets::widget::header;
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
        Button, Column, Container, Element, Header, PickList, Row, Scrollable, TableRow, Text,
        TextInput,
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
    pub recipient_address_value: String,
    // pub amount_input_state: text_input::State,
    pub amount_value: String,
    // whether amount has errored
    amount_error: bool,
    // slatepack address error
    slatepack_address_error: bool,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            recipient_address_value: Default::default(),
            amount_value: Default::default(),
            amount_error: false,
            slatepack_address_error: false,
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

    TxCreatedOk(Slate, String),
    TxCreateError(Arc<RwLock<Option<anyhow::Error>>>),
    SlatepackAddressError,
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
            state.amount_error = false;
            state.slatepack_address_error = false;

            log::debug!("Interaction::WalletOperationCreateTxViewInteraction");

            let w = grin_gui.wallet_interface.clone();

            let amount = match amount_from_hr_string(&state.amount_value) {
                Ok(0) | Err(_) => {
                    state.amount_error = true;
                    return Ok(Command::none());
                }
                Ok(a) => a,
            };

            // Todo: Amount parsing + validation, just testing the flow for now
            let args = InitTxArgs {
                src_acct_name: None,
                amount,
                minimum_confirmations: 2,
                max_outputs: 500,
                num_change_outputs: 1,
                selection_strategy_is_use_all: false,
                late_lock: Some(false),
                ..Default::default()
            };
            let fut =
                move || WalletInterface::create_tx(w, args, state.recipient_address_value.clone());

            return Ok(Command::perform(fut(), |r| match r {
                Ok((enc_slate, unenc_slate)) => {
                    Message::Interaction(Interaction::WalletOperationCreateTxViewInteraction(
                        LocalViewInteraction::TxCreatedOk(enc_slate, unenc_slate),
                    ))
                }
                Err(e) => match e {
                    GrinWalletInterfaceError::InvalidSlatepackAddress => {
                        Message::Interaction(Interaction::WalletOperationCreateTxViewInteraction(
                            LocalViewInteraction::SlatepackAddressError,
                        ))
                    }
                    _ => Message::Interaction(Interaction::WalletOperationCreateTxViewInteraction(
                        LocalViewInteraction::TxCreateError(Arc::new(RwLock::new(Some(
                            anyhow::Error::from(e),
                        )))),
                    )),
                },
            }));
        }
        LocalViewInteraction::TxCreatedOk(unencrypted_slate, encrypted_slate) => {
            log::debug!("{:?}", encrypted_slate);
            grin_gui
                .wallet_state
                .operation_state
                .show_slatepack_state
                .encrypted_slate = Some(encrypted_slate.to_string());

            grin_gui
                .wallet_state
                .operation_state
                .show_slatepack_state
                .title_label = localized_string("tx-create-success-title");

            grin_gui
                .wallet_state
                .operation_state
                .show_slatepack_state
                .desc = localized_string("tx-create-success-desc");

            // create a directory to which files will be output, if it doesn't exist
            if let Some(dir) = grin_gui.config.get_wallet_slatepack_dir() {
                let out_file_name = format!("{}/{}.slatepack", dir, unencrypted_slate.id);
                let mut output = File::create(out_file_name.clone())?;
                output.write_all(&encrypted_slate.as_bytes())?;
                output.sync_all()?;
            }

            grin_gui.wallet_state.operation_state.mode =
                crate::gui::element::wallet::operation::Mode::ShowSlatepack;
        }
        LocalViewInteraction::TxCreateError(err) => {
            grin_gui.error = err.write().unwrap().take();
            if let Some(e) = grin_gui.error.as_ref() {
                log_error(e);
            }
        }
        LocalViewInteraction::SlatepackAddressError => state.slatepack_address_error = true,
    }

    Ok(Command::none())
}

pub fn data_container<'a>(config: &'a Config, state: &'a StateContainer) -> Container<'a, Message> {
    // Title row
    let title = Text::new(localized_string("create-tx"))
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

    // push more items on to header here: e.g. other buttons, things that belong on the header
    let header_row = Row::new().push(title_container);

    let header_container = Container::new(header_row).padding(iced::Padding::from([
        0,                      // top
        0,                      // right
        DEFAULT_PADDING as u16, // bottom
        0,                      // left
    ]));

    let recipient_address = Text::new(localized_string("recipient-address"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let recipient_address_container = Container::new(recipient_address)
        .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let recipient_address_input = TextInput::new("", &state.recipient_address_value)
        .on_input(|s| {
            Interaction::WalletOperationCreateTxViewInteraction(
                LocalViewInteraction::RecipientAddress(s),
            )
        })
        .size(DEFAULT_FONT_SIZE)
        .padding(6)
        .width(Length::Fixed(400.0))
        .style(grin_gui_core::theme::TextInputStyle::AddonsQuery);

    let recipient_address_input: Element<Interaction> = recipient_address_input.into();

    let address_error = Text::new(localized_string("create-tx-address-error"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left)
        .style(grin_gui_core::theme::text::TextStyle::Warning);

    let address_error_container =
        Container::new(address_error).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let amount = Text::new(localized_string("create-tx-amount"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let amount_container =
        Container::new(amount).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let amount_input = TextInput::new(
        // &mut state.amount_input_state,
        "",
        &state.amount_value,
    )
    .on_input(|s| {
        Interaction::WalletOperationCreateTxViewInteraction(LocalViewInteraction::Amount(s))
    })
    .size(DEFAULT_FONT_SIZE)
    .padding(6)
    .width(Length::Fixed(100.0))
    .style(grin_gui_core::theme::TextInputStyle::AddonsQuery);

    let amount_input: Element<Interaction> = amount_input.into();

    let amount_error = Text::new(localized_string("create-tx-amount-error"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left)
        .style(grin_gui_core::theme::text::TextStyle::Warning);

    let amount_error_container =
        Container::new(amount_error).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let address_instruction_container =
        Text::new(localized_string("recipient-address-instruction"))
            .size(SMALLER_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

    let address_instruction_container =
        Container::new(address_instruction_container).style(ContainerStyle::NormalBackground);

    let button_height = Length::Fixed(BUTTON_HEIGHT);
    let button_width = Length::Fixed(BUTTON_WIDTH);

    let submit_button_label_container =
        Container::new(Text::new(localized_string("tx-create-submit")).size(DEFAULT_FONT_SIZE))
            .width(button_width)
            .height(button_height)
            .center_x()
            .center_y()
            .align_x(alignment::Horizontal::Center);

    let mut submit_button = Button::new(submit_button_label_container)
        .style(grin_gui_core::theme::ButtonStyle::Primary);
    let submit_button =
        submit_button.on_press(Interaction::WalletOperationCreateTxViewInteraction(
            LocalViewInteraction::CreateTransaction(),
        ));
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
        .on_press(Interaction::WalletOperationCreateTxViewInteraction(
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
        .push(recipient_address_container)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
        .push(address_instruction_container)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
        .push(recipient_address_input.map(Message::Interaction))
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)));

    if state.slatepack_address_error {
        column = column
            .push(address_error_container)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)));
    }

    column = column
        .push(amount_container)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
        .push(amount_input.map(Message::Interaction))
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)));

    if state.amount_error {
        column = column
            .push(amount_error_container)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)));
    }

    column = column
        .push(button_row)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
        .push(Space::new(
            Length::Fixed(0.0),
            Length::Fixed(unit_spacing + 10.0),
        ));

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
