use super::tx_list::{self, ExpandType};
use crate::log_error;
use async_std::prelude::FutureExt;
use grin_gui_core::{
    config::Config,
    wallet::{TxLogEntry, TxLogEntryType},
};
use grin_gui_widgets::{header, Header, TableRow};
use iced::button::StyleSheet;
use iced_native::Widget;
use std::path::PathBuf;

use super::tx_list::{HeaderState, TxList};

use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    anyhow::Context,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::wallet::{StatusMessage, WalletInfo, WalletInterface},
    iced::{
        alignment, button, scrollable, text_input, Alignment, Button, Checkbox, Column, Command,
        Container, Element, Length, Row, Scrollable, Space, Text, TextInput,
    },
    std::sync::{Arc, RwLock},
};

pub struct StateContainer {
    wallet_info: Option<WalletInfo>,
    wallet_txs: TxList,
    wallet_status: String,
    txs_scrollable_state: scrollable::State,
    pub expanded_type: ExpandType,
    last_summary_update: chrono::DateTime<chrono::Local>,
    tx_header_state: HeaderState,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            wallet_info: Default::default(),
            wallet_txs: Default::default(),
            wallet_status: Default::default(),
            txs_scrollable_state: Default::default(),
            expanded_type: ExpandType::None,
            last_summary_update: Default::default(),
            tx_header_state: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    Submit,
    /// was updated from node, info
    WalletInfoUpdateSuccess(bool, WalletInfo, Vec<TxLogEntry>),
    WalletInfoUpdateFailure(Arc<RwLock<Option<anyhow::Error>>>),
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
    Ok(Command::none())
}

pub fn handle_message<'a>(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let state = &mut grin_gui.wallet_state.operation_state.home_state;
    match message {
        LocalViewInteraction::Submit => {}
        LocalViewInteraction::WalletInfoUpdateSuccess(node_success, wallet_info, txs) => {
            debug!(
                "Update Wallet Info Summary: {}, {:?}",
                node_success, wallet_info
            );
            state.wallet_info = Some(wallet_info);
            debug!("Update Wallet Txs Summary: {:?}", txs);
            state.wallet_txs = TxList { txs };
        }
        LocalViewInteraction::WalletInfoUpdateFailure(err) => {
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

    let title_row = Row::new()
        .push(title_container)
        .align_items(Alignment::Center)
        .padding(6)
        .spacing(20);

    // Basic Info "Box"

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
        .height(Length::FillPortion(1))
        .style(style::Scrollable(color_palette));

    let mut has_txs = false;
    // Loops though the txs.
    for (idx, tx) in state.wallet_txs.txs.iter().enumerate() {
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
            ExpandType::Details(a) => a.id == tx.id,
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
            tx,
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
        tx_list_content = tx_list_content.push(tx_row_titles).push(tx_list_scrollable)
    }

    // Overall Home screen layout column
    let column = Column::new()
        .push(title_row)
        .push(Space::new(Length::Units(0), Length::Fill))
        .push(tx_list_content)
        .push(Space::new(Length::Units(0), Length::Fill))
        .push(status_row)
        .align_items(Alignment::Center);

    Container::new(column)
        .center_y()
        .center_x()
        .width(Length::Fill)
}
