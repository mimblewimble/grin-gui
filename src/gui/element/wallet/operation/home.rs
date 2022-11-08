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

use super::action_menu;
use super::tx_list::{HeaderState, TxList, TxLogEntryWrap};

use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    anyhow::Context,
    grin_gui_core::wallet::{StatusMessage, WalletInfo, WalletInterface},
    grin_gui_core::{node::amount_to_hr_string, theme::ColorPalette},
    iced::{
        alignment, button, scrollable, text_input, Alignment, Button, Checkbox, Column, Command,
        Container, Element, Length, Row, Scrollable, Space, Text, TextInput,
    },
    std::sync::{Arc, RwLock},
};

pub struct StateContainer {
    pub action_menu_state: action_menu::StateContainer,
    pub back_button_state: button::State,
    pub cancel_button_states: Vec<button::State>,
    pub expanded_type: ExpandType,

    wallet_info: Option<WalletInfo>,
    wallet_txs: TxList,
    wallet_status: String,
    txs_scrollable_state: scrollable::State,
    last_summary_update: chrono::DateTime<chrono::Local>,
    tx_header_state: HeaderState,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            action_menu_state: Default::default(),
            cancel_button_states: vec![],
            back_button_state: Default::default(),
            expanded_type: ExpandType::None,
            wallet_info: Default::default(),
            wallet_txs: Default::default(),
            wallet_status: Default::default(),
            txs_scrollable_state: Default::default(),
            last_summary_update: Default::default(),
            tx_header_state: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    Back,
    Submit,
    /// was updated from node, info
    WalletInfoUpdateSuccess(bool, WalletInfo, Vec<TxLogEntry>),
    WalletInfoUpdateFailure(Arc<RwLock<Option<anyhow::Error>>>),
    WalletSlatepackAddressUpdateSuccess(String),
    WalletCloseError(Arc<RwLock<Option<anyhow::Error>>>),
    WalletCloseSuccess,
    CancelTx(u32),
    TxCancelledOk(u32),
    TxCancelError(Arc<RwLock<Option<anyhow::Error>>>)
}

// Okay to modify state and access wallet here
pub fn handle_tick<'a>(
    grin_gui: &mut GrinGui,
    time: chrono::DateTime<chrono::Local>,
) -> Result<Command<Message>> {
    {
        let w = grin_gui.wallet_interface.read().unwrap();
        if !w.wallet_is_open() {
            return Ok(Command::none());
        }
    }
    let messages = WalletInterface::get_wallet_updater_status(grin_gui.wallet_interface.clone())?;
    let state = &mut grin_gui.wallet_state.operation_state.home_state;
    let last_message = messages.get(0);
    if let Some(m) = last_message {
        state.wallet_status = match m {
            StatusMessage::UpdatingOutputs(s) => format!("{}", s),
            StatusMessage::UpdatingTransactions(s) => format!("{}", s),
            StatusMessage::FullScanWarn(s) => format!("{}", s),
            StatusMessage::Scanning(s, m) => format!("Scanning - {}% complete", m),
            StatusMessage::ScanningComplete(s) => format!("{}", s),
            StatusMessage::UpdateWarning(s) => format!("{}", s),
        }
    }
    if time - state.last_summary_update
        > chrono::Duration::from_std(std::time::Duration::from_secs(10)).unwrap()
    {
        state.last_summary_update = chrono::Local::now();

        let w = grin_gui.wallet_interface.clone();

        let fut =
            move || WalletInterface::get_wallet_info(w.clone()).join(WalletInterface::get_txs(w));

        return Ok(Command::perform(fut(), |(wallet_info_res, txs_res)| {
            if wallet_info_res.is_err() {
                let e = wallet_info_res
                    .context("Failed to retrieve wallet info status")
                    .unwrap_err();
                return Message::Interaction(Interaction::WalletOperationHomeViewInteraction(
                    LocalViewInteraction::WalletInfoUpdateFailure(Arc::new(RwLock::new(Some(e)))),
                ));
            }
            if txs_res.is_err() {
                let e = txs_res
                    .context("Failed to retrieve wallet tx status")
                    .unwrap_err();
                return Message::Interaction(Interaction::WalletOperationHomeViewInteraction(
                    LocalViewInteraction::WalletInfoUpdateFailure(Arc::new(RwLock::new(Some(e)))),
                ));
            }
            let (node_success, wallet_info) = wallet_info_res.unwrap();
            let (_, txs) = txs_res.unwrap();
            Message::Interaction(Interaction::WalletOperationHomeViewInteraction(
                LocalViewInteraction::WalletInfoUpdateSuccess(node_success, wallet_info, txs),
            ))
        }));
    }
    // If slatepack address is not filled out, go get it
    let apply_tx_state = &mut grin_gui.wallet_state.operation_state.apply_tx_state;
    if apply_tx_state.address_value.is_empty() {
        let w = grin_gui.wallet_interface.clone();

        let fut = move || WalletInterface::get_slatepack_address(w.clone());
        return Ok(Command::perform(fut(), |get_slatepack_address_res| {
            if get_slatepack_address_res.is_err() {
                let e = get_slatepack_address_res
                    .context("Failed to retrieve wallet slatepack address")
                    .unwrap_err();
                return Message::Interaction(Interaction::WalletOperationHomeViewInteraction(
                    LocalViewInteraction::WalletInfoUpdateFailure(Arc::new(RwLock::new(Some(e)))),
                ));
            }
            Message::Interaction(Interaction::WalletOperationHomeViewInteraction(
                LocalViewInteraction::WalletSlatepackAddressUpdateSuccess(
                    get_slatepack_address_res.unwrap(),
                ),
            ))
        }));
    }
    Ok(Command::none())
}

pub fn handle_message<'a>(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let state = &mut grin_gui.wallet_state.operation_state.home_state;
    match message {
        LocalViewInteraction::Back => {
            let wallet_interface = grin_gui.wallet_interface.clone();
            let fut = WalletInterface::close_wallet(wallet_interface);

            return Ok(Command::perform(fut, |r| {
                match r.context("Failed to close wallet") {
                    Ok(()) => {
                        Message::Interaction(Interaction::WalletOperationHomeViewInteraction(
                            LocalViewInteraction::WalletCloseSuccess,
                        ))
                    }
                    Err(e) => {
                        Message::Interaction(Interaction::WalletOperationHomeViewInteraction(
                            LocalViewInteraction::WalletCloseError(Arc::new(RwLock::new(Some(e)))),
                        ))
                    }
                }
            }));
        }
        LocalViewInteraction::Submit => {}
        LocalViewInteraction::WalletInfoUpdateSuccess(node_success, wallet_info, txs) => {
            debug!(
                "Update Wallet Info Summary: {}, {:?}",
                node_success, wallet_info
            );
            state.wallet_info = Some(wallet_info);
            debug!("Update Wallet Txs Summary: {:?}", txs);
            let tx_wrap_list = txs
                .iter()
                .map(|tx| TxLogEntryWrap::new(tx.clone()))
                .collect();
            state.wallet_txs = TxList { txs: tx_wrap_list };
        }
        LocalViewInteraction::WalletInfoUpdateFailure(err) => {
            grin_gui.error = err.write().unwrap().take();
            if let Some(e) = grin_gui.error.as_ref() {
                log_error(e);
            }
        }
        LocalViewInteraction::WalletCloseSuccess => {
            grin_gui.wallet_state.operation_state.mode =
                crate::gui::element::wallet::operation::Mode::Open;
        }
        LocalViewInteraction::WalletCloseError(err) => {
            grin_gui.error = err.write().unwrap().take();
            if let Some(e) = grin_gui.error.as_ref() {
                log_error(e);
            }
        }
        LocalViewInteraction::WalletSlatepackAddressUpdateSuccess(address) => {
            grin_gui
                .wallet_state
                .operation_state
                .apply_tx_state
                .address_value = address;
        }
        LocalViewInteraction::CancelTx(id) => {
            debug!("Cancel Tx: {}", id);
            grin_gui.error.take();

            log::debug!("Interaction::WalletOperationHomeViewInteraction::CancelTx");

            let w = grin_gui.wallet_interface.clone();

            let fut = move || WalletInterface::cancel_tx(w, id);

            return Ok(Command::perform(fut(), |r| {
                match r.context("Failed to Cancel Transaction") {
                    Ok(ret) => Message::Interaction(Interaction::WalletOperationHomeViewInteraction(
                        LocalViewInteraction::TxCancelledOk(ret),
                    )),
                    Err(e) => Message::Interaction(Interaction::WalletOperationHomeViewInteraction(
                        LocalViewInteraction::TxCancelError(Arc::new(RwLock::new(Some(e)))),
                    )),
                }
            }));
        },
        LocalViewInteraction::TxCancelledOk(id) => {
            //TODO: Message
            debug!("TX cancelled okay: {}", id);
        },
        LocalViewInteraction::TxCancelError(err) => {
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
    let title = Text::new(localized_string("wallet-home"))
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
            .on_press(Interaction::WalletOperationHomeViewInteraction(
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

    // Buttons to perform operations go here, but empty container for now
    let operations_menu =
        action_menu::data_container(color_palette, config, &mut state.action_menu_state);

    // Basic Info "Box"
    let waiting_string = "---------";
    let (
        total_string,
        amount_spendable_string,
        awaiting_confirmation_string,
        awaiting_finalization_string,
        locked_string,
    ) = match state.wallet_info.as_ref() {
        Some(info) => (
            amount_to_hr_string(info.total, false),
            amount_to_hr_string(info.amount_currently_spendable, false),
            amount_to_hr_string(info.amount_awaiting_confirmation, false),
            amount_to_hr_string(info.amount_awaiting_finalization, false),
            amount_to_hr_string(info.amount_locked, false),
        ),
        None => (
            waiting_string.to_owned(),
            waiting_string.to_owned(),
            waiting_string.to_owned(),
            waiting_string.to_owned(),
            waiting_string.to_owned(),
        ),
    };

    let total_value_label = Text::new(format!("{}:", localized_string("info-confirmed-total")));
    let total_value_label_container =
        Container::new(total_value_label).style(style::BrightBackgroundContainer(color_palette));

    let total_value = Text::new(total_string);
    let total_value_container = Container::new(total_value)
        .style(style::BrightBackgroundContainer(color_palette))
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Right);

    let total_row = Row::new()
        .push(total_value_label_container)
        .push(total_value_container)
        .width(Length::Fill)
        .spacing(5);

    let awaiting_confirmation_label = Text::new(format!(
        "{}:",
        localized_string("info-awaiting-confirmation")
    ));
    let awaiting_confirmation_label_container = Container::new(awaiting_confirmation_label)
        .style(style::BrightBackgroundContainer(color_palette));

    let awaiting_confirmation_value = Text::new(awaiting_confirmation_string);
    let awaiting_confirmation_value_container = Container::new(awaiting_confirmation_value)
        .style(style::BrightBackgroundContainer(color_palette))
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Right);

    let awaiting_confirmation_row = Row::new()
        .push(awaiting_confirmation_label_container)
        .push(awaiting_confirmation_value_container)
        .width(Length::Fill)
        .spacing(5);

    let awaiting_finalization_label = Text::new(format!(
        "{}:",
        localized_string("info-awaiting-finalization")
    ));
    let awaiting_finalization_label_container = Container::new(awaiting_finalization_label)
        .style(style::BrightBackgroundContainer(color_palette));

    let awaiting_finalization_value = Text::new(awaiting_finalization_string);
    let awaiting_finalization_value_container = Container::new(awaiting_finalization_value)
        .style(style::BrightBackgroundContainer(color_palette))
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Right);

    let awaiting_finalization_row = Row::new()
        .push(awaiting_finalization_label_container)
        .push(awaiting_finalization_value_container)
        .width(Length::Fill)
        .spacing(5);

    let locked_label = Text::new(format!("{}:", localized_string("info-locked")));
    let locked_label_container =
        Container::new(locked_label).style(style::BrightBackgroundContainer(color_palette));

    let locked_value = Text::new(locked_string);
    let locked_value_container = Container::new(locked_value)
        .style(style::BrightBackgroundContainer(color_palette))
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Right);

    let locked_row = Row::new()
        .push(locked_label_container)
        .push(locked_value_container)
        .width(Length::Fill)
        .spacing(5);

    let amount_spendable_label =
        Text::new(format!("{}:", localized_string("info-amount-spendable")));
    let amount_spendable_label_container = Container::new(amount_spendable_label)
        .style(style::BrightBackgroundContainer(color_palette));

    let amount_spendable_value = Text::new(amount_spendable_string);
    let amount_spendable_value_container = Container::new(amount_spendable_value)
        .style(style::BrightBackgroundContainer(color_palette))
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Right);

    let amount_spendable_row = Row::new()
        .push(amount_spendable_label_container)
        .push(amount_spendable_value_container)
        .width(Length::Fill)
        .spacing(5);

    let info_column = Column::new()
        .push(total_row)
        .push(awaiting_confirmation_row)
        .push(awaiting_finalization_row)
        .push(locked_row)
        .push(amount_spendable_row)
        .spacing(10);

    let wallet_info_card_title_string = "".to_owned();
    let wallet_info_card = Card::new(
        Text::new(wallet_info_card_title_string).size(DEFAULT_HEADER_FONT_SIZE),
        info_column,
    )
    .style(style::NormalModalCardContainer(color_palette));

    let wallet_info_card_container = Container::new(wallet_info_card).width(Length::FillPortion(2));

    // Home 'row', operation buttons beside info
    let first_row_container = Row::new()
        .push(wallet_info_card_container)
        .push(operations_menu)
        .padding(10);

    // Status container bar at bottom of screen
    let status_container_label_text = Text::new(localized_string("status"))
        .size(DEFAULT_FONT_SIZE)
        .height(Length::Fill)
        .horizontal_alignment(alignment::Horizontal::Right)
        .vertical_alignment(alignment::Vertical::Center);

    let status_container_separator_text = Text::new(": ")
        .size(DEFAULT_FONT_SIZE)
        .height(Length::Fill)
        .horizontal_alignment(alignment::Horizontal::Right)
        .vertical_alignment(alignment::Vertical::Center);

    let status_container_status_text = Text::new(&state.wallet_status)
        .size(DEFAULT_FONT_SIZE)
        .height(Length::Fill)
        .horizontal_alignment(alignment::Horizontal::Right)
        .vertical_alignment(alignment::Vertical::Center);

    let status_container_contents = Row::new()
        .push(Space::new(Length::Fill, Length::Fill))
        .push(status_container_label_text)
        .push(status_container_separator_text)
        .push(status_container_status_text)
        .push(Space::new(Length::Units(DEFAULT_PADDING), Length::Units(0)));

    let status_container = Container::new(status_container_contents)
        .style(style::BrightForegroundContainer(color_palette))
        .height(Length::Fill)
        .width(Length::Fill)
        .style(style::NormalForegroundContainer(color_palette));

    let status_row = Row::new()
        .push(status_container)
        .height(Length::Units(25))
        .align_items(Alignment::Center)
        .spacing(25);

    // Temp Test Data
    use grin_gui_core::node::Identifier;
    /*let tx_list = TxList {
        txs: vec![
            TxLogEntry::new(Identifier::zero(), TxLogEntryType::ConfirmedCoinbase, 0),
            TxLogEntry::new(Identifier::zero(), TxLogEntryType::ConfirmedCoinbase, 1),
            TxLogEntry::new(Identifier::zero(), TxLogEntryType::ConfirmedCoinbase, 2),
            TxLogEntry::new(Identifier::zero(), TxLogEntryType::ConfirmedCoinbase, 3),
            TxLogEntry::new(Identifier::zero(), TxLogEntryType::ConfirmedCoinbase, 4),
            TxLogEntry::new(Identifier::zero(), TxLogEntryType::ConfirmedCoinbase, 5),
            TxLogEntry::new(Identifier::zero(), TxLogEntryType::ConfirmedCoinbase, 6),
        ],
    };*/

    let column_config = state.tx_header_state.column_config();

    // Tx row titles is a row of titles above the tx scrollable.
    // This is to add titles above each section of the tx row, to let
    // the user easily identify what the value is.
    let tx_row_titles = super::tx_list::titles_row_header(
        color_palette,
        &state.wallet_txs,
        &mut state.tx_header_state.state,
        &mut state.tx_header_state.columns,
        state.tx_header_state.previous_column_key,
        state.tx_header_state.previous_sort_direction,
    );

    // A scrollable list containing rows.
    // Each row holds data about a single tx.
    let mut tx_list_scrollable = Scrollable::new(&mut state.txs_scrollable_state)
        .spacing(1)
        //.height(Length::Fill)
        .style(style::Scrollable(color_palette));

    let mut has_txs = false;

    let cancel_states = &mut state.cancel_button_states;
    // Loops though the txs.
    for (idx, tx_wrap) in state.wallet_txs.txs.iter_mut().enumerate() {
        has_txs = true;
        // If hiding ignored addons, we will skip it.
        /*if addon.state == AddonState::Ignored && self.config.hide_ignored_addons {
            continue;
        }*/

        // Skip addon if we are filter from query and addon doesn't have fuzzy score
        /*if query.is_some() && addon.fuzzy_score.is_none() {
            continue;
        }*/

        // Checks if the current tx is expanded.
        let is_tx_expanded = match &state.expanded_type {
            ExpandType::Details(a) => a.tx.id == tx_wrap.tx.id,
            ExpandType::None => false,
        };

        let is_odd = if config.alternating_row_colors {
            Some(idx % 2 != 0)
        } else {
            None
        };

        // A container cell which has all data about the current tx.
        // If the tx is expanded, then this is also included in this container.
        let tx_data_cell = tx_list::data_row_container(
            color_palette,
            tx_wrap,
            is_tx_expanded,
            &state.expanded_type,
            config,
            &column_config,
            is_odd,
            &None,
        );

        // Adds the addon data cell to the scrollable.
        tx_list_scrollable = tx_list_scrollable.push(tx_data_cell);
    }

    // Bottom space below the scrollable.
    let bottom_space = Space::new(Length::FillPortion(1), Length::Units(DEFAULT_PADDING));

    // This column gathers all the tx list elements together.
    let mut tx_list_content = Column::new();

    // Adds the rest of the elements to the content column.
    if has_txs {
        tx_list_content = tx_list_content.push(tx_row_titles).push(tx_list_scrollable);
    }

    // Overall Home screen layout column
    let column = Column::new()
        .push(title_row)
        .push(first_row_container)
        .push(tx_list_content)
        .push(Space::new(Length::Units(0), Length::Fill))
        .push(status_row)
        .padding(10)
        .align_items(Alignment::Center);

    Container::new(column)
        .center_y()
        .center_x()
        .width(Length::Fill)
}
