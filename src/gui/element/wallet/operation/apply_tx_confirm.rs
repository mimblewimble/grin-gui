use super::tx_list::{self, ExpandType};
use crate::log_error;
use async_std::prelude::FutureExt;
use grin_gui_core::{
    config::Config,
    wallet::{Slate, SlateState, Slatepack, TxLogEntry, TxLogEntryType},
};
use grin_gui_widgets::widget::header;
use iced_aw::Card;
use iced_native::Widget;
use std::fs::{self, File};
use std::io::Write;
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
    grin_gui_core::wallet::{StatusMessage, WalletInfo, WalletInterface},
    grin_gui_core::{node::amount_to_hr_string, theme::ColorPalette},
    iced::widget::{button, pick_list, scrollable, text_input, Checkbox, Space},
    iced::{alignment, Alignment, Command, Length},
    serde::{Deserialize, Serialize},
    std::sync::{Arc, RwLock},
};

pub struct StateContainer {
    // pub back_button_state: button::State,
    // pub copy_address_button_state: button::State,
    // pub address_state: text_input::State,
    pub address_value: String,
    // Slatepack read result
    pub slatepack_read_result: String,
    // Actual read slatepack
    pub slatepack_parsed: Option<(Slatepack, Slate)>,
    // In the state of applying slatepack
    pub is_signing: bool,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            // back_button_state: Default::default(),
            // copy_address_button_state: Default::default(),
            // address_state: Default::default(),
            address_value: Default::default(),
            slatepack_read_result: localized_string("tx-slatepack-read-result-default"),
            slatepack_parsed: None,
            is_signing: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    Back,
    Accept,
    TxAcceptSuccess(Slate, Option<String>),
    TxAcceptFailure(Arc<RwLock<Option<anyhow::Error>>>),
}

pub fn handle_message<'a>(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let state = &mut grin_gui
        .wallet_state
        .operation_state
        .apply_tx_state
        .confirm_state;
    match message {
        LocalViewInteraction::Back => {
            log::debug!("Interaction::WalletOperationApplyTxConfirmViewInteraction(Cancel)");
            state.is_signing = false;
            grin_gui.wallet_state.operation_state.mode =
                crate::gui::element::wallet::operation::Mode::Home;
        }
        LocalViewInteraction::Accept => {
            grin_gui.error.take();

            log::debug!("Interaction::WalletOperationApplyTxConfirmViewInteraction(Accept)");
            if state.slatepack_parsed.is_none() {
                log::debug!("you should never see this - dev make sure slatepack is not None");
                return Ok(Command::none());
            }

            let (slatepack, slate) = state.slatepack_parsed.as_ref().unwrap();

            let sp_sending_address = match &slatepack.sender {
                None => "None".to_string(),
                Some(s) => s.to_string(),
            };

            let w = grin_gui.wallet_interface.clone();
            let out_slate = slate.clone();
            match slate.state {
                SlateState::Standard1 => {
                    state.is_signing = true;
                    let fut = move || {
                        WalletInterface::receive_tx_from_s1(w, out_slate, sp_sending_address)
                    };

                    return Ok(Command::perform(fut(), |r| {
                        match r.context("Failed to Progress Transaction") {
                            Ok((slate, enc_slate)) => Message::Interaction(
                                Interaction::WalletOperationApplyTxConfirmViewInteraction(
                                    LocalViewInteraction::TxAcceptSuccess(slate, enc_slate),
                                ),
                            ),
                            Err(e) => Message::Interaction(
                                Interaction::WalletOperationApplyTxConfirmViewInteraction(
                                    LocalViewInteraction::TxAcceptFailure(Arc::new(RwLock::new(
                                        Some(e),
                                    ))),
                                ),
                            ),
                        }
                    }));
                }
                SlateState::Standard2 => {
                    state.is_signing = true;
                    let fut = move || WalletInterface::finalize_from_s2(w, out_slate, true);

                    return Ok(Command::perform(fut(), |r| {
                        match r.context("Failed to Progress Transaction") {
                            Ok((slate, enc_slate)) => Message::Interaction(
                                Interaction::WalletOperationApplyTxConfirmViewInteraction(
                                    LocalViewInteraction::TxAcceptSuccess(slate, enc_slate),
                                ),
                            ),
                            Err(e) => Message::Interaction(
                                Interaction::WalletOperationApplyTxConfirmViewInteraction(
                                    LocalViewInteraction::TxAcceptFailure(Arc::new(RwLock::new(
                                        Some(e),
                                    ))),
                                ),
                            ),
                        }
                    }));
                }
                _ => {
                    log::error!("Slate state not yet supported");
                    return Ok(Command::none());
                }
            }
        }
        LocalViewInteraction::TxAcceptSuccess(slate, encrypted_slate) => {
            // Output the latest slatepack, overriding any previous
            if let Some(ref s) = encrypted_slate {
                if let Some(dir) = grin_gui.config.get_wallet_slatepack_dir() {
                    let out_file_name = format!("{}/{}.slatepack", dir, slate.id);
                    let mut output = File::create(out_file_name.clone())?;
                    output.write_all(&s.as_bytes())?;
                    output.sync_all()?;
                }
            } else {
                // If no encrypted slate, tx was posted so remove file
                if let Some(dir) = grin_gui.config.get_wallet_slatepack_dir() {
                    let out_file_name = format!("{}/{}.slatepack", dir, slate.id);
                    let _ = fs::remove_file(out_file_name);
                }
            }

            grin_gui
                .wallet_state
                .operation_state
                .show_slatepack_state
                .encrypted_slate = encrypted_slate;

            grin_gui
                .wallet_state
                .operation_state
                .show_slatepack_state
                .title_label = localized_string("tx-continue-success-title");

            grin_gui
                .wallet_state
                .operation_state
                .show_slatepack_state
                .desc = localized_string("tx-continue-success-desc");

            state.is_signing = false;
            grin_gui.wallet_state.operation_state.mode =
                crate::gui::element::wallet::operation::Mode::ShowSlatepack;
        }
        LocalViewInteraction::TxAcceptFailure(err) => {
            state.is_signing = false;
            grin_gui.error = err.write().unwrap().take();
            if let Some(e) = grin_gui.error.as_ref() {
                log_error(e);
            }
        }
    }
    Ok(Command::none())
}

// Very hacky, but these amount string will require different placements of
// words + amount in different languages
fn parse_info_strings(in_str: &str, amount: &str) -> String {
    let amount_split: Vec<&str> = in_str.split("[AMOUNT]").collect();
    let mut amount_included = format!("{}{}", amount_split[0], amount);
    if amount_split.len() > 1 {
        amount_included = format!("{}{}", amount_included, amount_split[1]);
    }
    amount_included
}

pub fn data_container<'a>(config: &'a Config, state: &'a StateContainer) -> Container<'a, Message> {
    let unit_spacing = 15;

    if state.slatepack_parsed.is_none() {
        return Container::new(Text::new(
            "you should never see this - dev make sure slatepack is not None",
        ));
    }

    // Decode/parse/etc fields for display here
    let (slatepack, slate) = state.slatepack_parsed.as_ref().unwrap();

    let sp_sending_address = match &slatepack.sender {
        None => "None".to_string(),
        Some(s) => s.to_string(),
    };

    let amount = amount_to_hr_string(slate.amount, true);

    // TODO: What's displayed here should change based on the slate state
    let state_text = match slate.state {
        SlateState::Standard1 => parse_info_strings(&localized_string("tx-reception"), &amount),
        SlateState::Standard2 => {
            "You are the payee, and are finalizing the transaction and sending it to the chain for validation - Standard workflow".to_owned()
        }
        SlateState::Standard3 => "This transaction is finalised - Standard workflow".to_owned(),
        _ => "Support still in development".to_owned(),
    };

    // TX State (i.e. Stage)
    let state = Text::new(state_text).size(DEFAULT_FONT_SIZE);

    let state_container =
        Container::new(state).style(grin_gui_core::theme::ContainerStyle::BrightBackground);

    let state_row = Row::new().push(state_container);

    // Sender address
    let sender_address_label = Text::new(format!("{} ", localized_string("tx-sender-name")))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let sender_address_label_container = Container::new(sender_address_label)
        .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let sender_address = Text::new(sp_sending_address).size(DEFAULT_FONT_SIZE);
    //.width(Length::Units(400))
    //.style(grin_gui_core::theme::TextInputStyle::AddonsQuery);

    let sender_address_container = Container::new(sender_address)
        .style(grin_gui_core::theme::ContainerStyle::BrightBackground);

    let sender_address_row = Row::new()
        .push(sender_address_label_container)
        .push(sender_address_container);

    let instruction_label = Text::new(format!("{} ", localized_string("tx-reception-instruction")))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let instruction_label_container = Container::new(instruction_label)
        .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let instruction_label_2 = Text::new(format!(
        "{} ",
        localized_string("tx-reception-instruction-2")
    ))
    .size(DEFAULT_FONT_SIZE)
    .horizontal_alignment(alignment::Horizontal::Left);

    let instruction_label_container_2 = Container::new(instruction_label_2)
        .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let column = Column::new()
        .push(state_row)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(sender_address_row)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(instruction_label_container)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(instruction_label_container_2);

    let wrapper_column = Column::new().height(Length::Fill).push(column);

    // Returns the final container.
    Container::new(wrapper_column)
}
