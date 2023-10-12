use super::tx_list::{self, ExpandType};
use crate::log_error;
use async_std::prelude::FutureExt;
use grin_gui_core::{
    config::{Config, TxMethod},
    wallet::{
        ContractNewArgsAPI, ContractSetupArgsAPI, Slate, SlateState, Slatepack, TxLogEntry,
        TxLogEntryType,
    },
};
use grin_gui_widgets::widget::header;
use iced_aw::Card;
use iced_core::Widget;
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
    grin_gui_core::wallet::{parse_abs_tx_amount_fee, StatusMessage, WalletInfo, WalletInterface},
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
    pub slatepack_parsed: Option<(Slatepack, Slate, Option<TxLogEntry>)>,
    // In the state of applying slatepack
    pub is_signing: bool,
    // Is a self send
    pub is_self_send: bool,
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
            is_self_send: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    Back,
    Accept,
    TxAcceptSuccess(Slate, Option<String>, bool),
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
            log::debug!("Interaction::Back");
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

            let (slatepack, slate, tx_log_entry) = state.slatepack_parsed.as_ref().unwrap();

            let sp_sending_address = match &slatepack.sender {
                None => "None".to_string(),
                Some(s) => s.to_string(),
            };

            let w = grin_gui.wallet_interface.clone();
            let out_slate = slate.clone();
            if grin_gui.config.tx_method == TxMethod::Legacy {
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
                                        LocalViewInteraction::TxAcceptSuccess(
                                            slate, enc_slate, false,
                                        ),
                                    ),
                                ),
                                Err(e) => Message::Interaction(
                                    Interaction::WalletOperationApplyTxConfirmViewInteraction(
                                        LocalViewInteraction::TxAcceptFailure(Arc::new(
                                            RwLock::new(Some(e)),
                                        )),
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
                                        LocalViewInteraction::TxAcceptSuccess(
                                            slate, enc_slate, true,
                                        ),
                                    ),
                                ),
                                Err(e) => Message::Interaction(
                                    Interaction::WalletOperationApplyTxConfirmViewInteraction(
                                        LocalViewInteraction::TxAcceptFailure(Arc::new(
                                            RwLock::new(Some(e)),
                                        )),
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
            } else {
                let sp_sending_address = match &slatepack.sender {
                    None => "None".to_string(),
                    Some(s) => s.to_string(),
                };

                // Can we just dumbly do opposite here?
                let net_change = match slate.state {
                    SlateState::Standard1 | SlateState::Invoice1 => Some(-(slate.amount as i64)),
                    SlateState::Standard2 | SlateState::Invoice2 => None,
                    _ => {
                        log::error!("Slate state not yet supported");
                        return Ok(Command::none());
                    }
                };

                // Should be a simplified context flow here, where we can be recipient or sender!
                let mut args = ContractSetupArgsAPI {
                    net_change,
                    ..Default::default()
                };
                state.is_signing = true;

                if state.is_self_send {
                    debug!("SLATE STATE SELF_SEND: {}", slate.state);
                    let fut = move || {
                        WalletInterface::contract_sign(w, out_slate, args, sp_sending_address, true)
                    };
                    return Ok(Command::perform(fut(), |r| {
                        match r.context("Failed to Progress Transaction") {
                            Ok((slate, enc_slate)) => {
                                let finished = slate.state == SlateState::Standard3
                                    || slate.state == SlateState::Invoice3;
                                Message::Interaction(
                                    Interaction::WalletOperationApplyTxConfirmViewInteraction(
                                        LocalViewInteraction::TxAcceptSuccess(
                                            slate, enc_slate, true,
                                        ),
                                    ),
                                )
                            }
                            Err(e) => Message::Interaction(
                                Interaction::WalletOperationApplyTxConfirmViewInteraction(
                                    LocalViewInteraction::TxAcceptFailure(Arc::new(RwLock::new(
                                        Some(e),
                                    ))),
                                ),
                            ),
                        }
                    }));
                } else {
                    let fut = move || {
                        WalletInterface::contract_sign(w, out_slate, args, sp_sending_address, true)
                    };

                    return Ok(Command::perform(fut(), |r| {
                        match r.context("Failed to Progress Transaction") {
                            Ok((slate, enc_slate)) => {
                                debug!("SLATE STATE: {}", slate.state);
                                let finished = slate.state == SlateState::Standard3
                                    || slate.state == SlateState::Invoice3;
                                Message::Interaction(
                                    Interaction::WalletOperationApplyTxConfirmViewInteraction(
                                        LocalViewInteraction::TxAcceptSuccess(
                                            slate, enc_slate, finished,
                                        ),
                                    ),
                                )
                            }
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
            }
        }
        LocalViewInteraction::TxAcceptSuccess(slate, encrypted_slate, finished) => {
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

            state.is_signing = false;

            if finished {
                grin_gui.wallet_state.operation_state.mode =
                    crate::gui::element::wallet::operation::Mode::TxDone;
            } else {
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

                grin_gui.wallet_state.operation_state.mode =
                    crate::gui::element::wallet::operation::Mode::ShowSlatepack;
            }
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
    let unit_spacing = 15.0;

    if state.slatepack_parsed.is_none() {
        return Container::new(Text::new(
            "you should never see this - dev make sure slatepack is not None",
        ));
    }

    // Decode/parse/etc fields for display here
    let (slatepack, slate, tx_log_entry) = state.slatepack_parsed.as_ref().unwrap();

    let sp_sending_address = match &slatepack.sender {
        None => "None".to_string(),
        Some(s) => s.to_string(),
    };

    let mut amount = amount_to_hr_string(slate.amount, true);
    let mut other_wallet_label = localized_string("tx-sender-name");
    let mut reception_instruction_1 = localized_string("tx-reception-instruction");
    let mut reception_instruction_2 = localized_string("tx-reception-instruction-2");

    // TODO: What's displayed here should change based on the slate state
    let mut state_text = match slate.state {
        SlateState::Standard1 => parse_info_strings(&localized_string("tx-reception"), &amount),
        SlateState::Standard2 => {
            let mut fee = String::default();
            other_wallet_label = localized_string("tx-recipient-name");
            reception_instruction_2 =
                parse_info_strings(&localized_string("tx-s1-finalization-3"), &fee);
            if let Some(tx) = tx_log_entry {
                (amount, fee) = parse_abs_tx_amount_fee(tx, true);
            }
            reception_instruction_1 =
                parse_info_strings(&localized_string("tx-s1-finalization-2"), &fee);
            let amt_stmt = parse_info_strings(&localized_string("tx-s1-finalization-1"), &amount);
            amt_stmt
        }
        SlateState::Standard3 => "This transaction is finalised - Standard workflow".to_owned(),
        _ => "Support still in development".to_owned(),
    };

    if config.tx_method == TxMethod::Contracts {
        state_text = match slate.state {
            SlateState::Standard1 => {
                other_wallet_label = localized_string("tx-recipient-name");
                parse_info_strings(&localized_string("tx-sending"), &amount)
            }
            SlateState::Standard2 => {
                let mut fee = String::default();
                other_wallet_label = localized_string("tx-sender-name");
                reception_instruction_2 =
                    parse_info_strings(&localized_string("tx-s1-finalization-3"), &fee);
                if let Some(tx) = tx_log_entry {
                    (amount, fee) = parse_abs_tx_amount_fee(tx, true);
                }
                reception_instruction_1 =
                    parse_info_strings(&localized_string("tx-s1-finalization-2"), &fee);
                let amt_stmt = match state.is_self_send {
                    true => parse_info_strings(&localized_string("tx-s1-finalization-self-send"), &amount),
                    false => parse_info_strings(&localized_string("tx-s1-finalization-1"), &amount),
                };

                amt_stmt
            }
            SlateState::Standard3 => "This transaction is finalised - Standard Workflow".to_owned(),
            _ => state_text,
        };

        if state.is_self_send {
            other_wallet_label = localized_string("wallet-self-send-instruction");
        }
    }

    // TX State (i.e. Stage)
    let state = Text::new(state_text).size(DEFAULT_FONT_SIZE);

    let state_container =
        Container::new(state).style(grin_gui_core::theme::ContainerStyle::BrightBackground);

    let state_row = Row::new().push(state_container);

    // Sender address
    let sender_address_label = Text::new(format!("{} ", other_wallet_label))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let sender_address_label_container = Container::new(sender_address_label)
        .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let sender_address = Text::new(sp_sending_address).size(DEFAULT_FONT_SIZE);
    //.width(Length::Fixed(400))
    //.style(grin_gui_core::theme::TextInputStyle::AddonsQuery);

    let sender_address_container = Container::new(sender_address)
        .style(grin_gui_core::theme::ContainerStyle::BrightBackground);

    let sender_address_row = Row::new()
        .push(sender_address_label_container)
        .push(sender_address_container);

    let instruction_label = Text::new(format!("{} ", reception_instruction_1))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let instruction_label_container = Container::new(instruction_label)
        .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let instruction_label_2 = Text::new(format!("{} ", localized_string(&reception_instruction_2)))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let instruction_label_container_2 = Container::new(instruction_label_2)
        .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let column = Column::new()
        .push(state_row)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
        .push(sender_address_row)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
        .push(instruction_label_container)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
        .push(instruction_label_container_2);

    let wrapper_column = Column::new().height(Length::Fill).push(column);

    // Returns the final container.
    Container::new(wrapper_column)
}
