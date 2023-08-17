use super::tx_list::{self, ExpandType};
use crate::log_error;
use async_std::prelude::FutureExt;
use chrono::DurationRound;
use grin_gui_core::{
    config::Config,
    wallet::{TxLogEntry, TxLogEntryType},
};
use grin_gui_widgets::widget::header;
use iced_aw::Card;
use iced_core::Widget;
use std::{borrow::Borrow, path::PathBuf, str::FromStr};

use super::tx_list::{HeaderState, TxList, TxLogEntryWrap};

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
    grin_gui_core::wallet::{
        InitTxArgs, RetrieveTxQueryArgs, RetrieveTxQuerySortOrder, Slate, StatusMessage,
        WalletInfo, WalletInterface,
    },
    grin_gui_core::{
        node::amount_to_hr_string,
        theme::{ButtonStyle, ColorPalette, ContainerStyle},
    },
    iced::widget::{button, pick_list, scrollable, text_input, Checkbox, Space},
    iced::{alignment, Alignment, Command, Length},
    serde::{Deserialize, Serialize},
    std::sync::{Arc, RwLock},
};

pub struct StateContainer {
    // maintains a list of all confirmed transactions sorted by date
    confirmed_txns: Vec<TxLogEntry>,
    wallet_txs: TxList,
    tx_header_state: HeaderState,
    query_args: RetrieveTxQueryArgs,
    pub mode: Mode,

    pub expanded_type: ExpandType,

    // balance history for wallet as (date, grin_balance)
    pub balance_data: Vec<(chrono::DateTime<chrono::Utc>, f64)>,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            wallet_txs: Default::default(),
            tx_header_state: Default::default(),
            expanded_type: ExpandType::None,
            query_args: Default::default(),
            mode: Mode::NotInit,
            balance_data: vec![],
            confirmed_txns: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    NotInit,
    Recent,
    Outstanding,
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    SelectMode(Mode),
    RefreshList,
    TxListUpdateSuccess(bool, Vec<TxLogEntry>),
    TxListUpdateFailure(Arc<RwLock<Option<anyhow::Error>>>),
}

pub fn handle_message<'a>(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let state = &mut grin_gui
        .wallet_state
        .operation_state
        .home_state
        .tx_list_display_state;

    match message {
        LocalViewInteraction::SelectMode(new_mode) => {
            state.query_args = RetrieveTxQueryArgs::default();

            state.query_args.sort_order = Some(RetrieveTxQuerySortOrder::Desc);

            match new_mode {
                Mode::NotInit => {}
                Mode::Recent => {
                    state.query_args.exclude_cancelled = Some(true);
                }
                Mode::Outstanding => {
                    state.query_args.exclude_cancelled = Some(true);
                    state.query_args.include_outstanding_only = Some(true);
                }
            }

            state.mode = new_mode;

            let fut = move || async {};
            return Ok(Command::perform(fut(), |_| {
                return Message::Interaction(
                    Interaction::WalletOperationHomeTxListDisplayInteraction(
                        LocalViewInteraction::RefreshList,
                    ),
                );
            }));
        }

        LocalViewInteraction::RefreshList => {
            let w = grin_gui.wallet_interface.clone();

            let fut = move || WalletInterface::get_txs(w, Some(state.query_args.clone()));
            return Ok(Command::perform(fut(), |tx_list_res| {
                if tx_list_res.is_err() {
                    let e = tx_list_res
                        .context("Failed to retrieve transaction list")
                        .unwrap_err();
                    return Message::Interaction(
                        Interaction::WalletOperationHomeTxListDisplayInteraction(
                            LocalViewInteraction::TxListUpdateFailure(Arc::new(RwLock::new(Some(
                                e,
                            )))),
                        ),
                    );
                }
                let (node_success, txs) = tx_list_res.unwrap();
                Message::Interaction(Interaction::WalletOperationHomeTxListDisplayInteraction(
                    //LocalViewInteraction::WalletInfoUpdateSuccess(node_success, wallet_info, txs),
                    LocalViewInteraction::TxListUpdateSuccess(node_success, txs),
                ))
            }));
        }
        LocalViewInteraction::TxListUpdateSuccess(node_success, txs) => {
            debug!("Update Tx List Summary: {}", node_success);
            debug!("Update Wallet Txs Summary: {:?}", txs);
            let tx_wrap_list = txs
                .iter()
                .map(|tx| TxLogEntryWrap::new(tx.clone()))
                .collect();
            state.wallet_txs = TxList { txs: tx_wrap_list };

            let confirmed_txns: Vec<&TxLogEntry> = txs.iter().filter(|tx| tx.confirmed).collect();

            if !confirmed_txns.is_empty() {
                // added new confirmed transactions to state confirmed set?
                let mut added = false;

                for tx in confirmed_txns.iter() {
                    // if tx is not in state confirmed transactions, add it
                    if state
                        .confirmed_txns
                        .iter()
                        .find(|t| t.id == tx.id)
                        .is_none()
                    {
                        // push to state confirmed transactions
                        state.confirmed_txns.push(tx.clone().to_owned());
                        added = true;
                        debug!("Confirmed Tx: {:?}", tx);
                    }
                }

                if added {
                    // sort state transactions by date
                    state.confirmed_txns.sort_by(|a, b| {
                        a.confirmation_ts.unwrap().cmp(&b.confirmation_ts.unwrap())
                    });

                    let mut datetime_sums = vec![];
                    for tx in state.confirmed_txns.iter() {
                        // trunc transaction date to day
                        //let datetime = tx.confirmation_ts.unwrap().duration_trunc(chrono::Duration::days(1)).unwrap();
                        // this should be the date time above but for dev purposes lets backdate it
                        let datetime = chrono::DateTime::from_str("2019-01-20T00:00:00Z").unwrap();
                        let credits = tx.amount_credited;
                        let debits = tx.amount_debited;

                        datetime_sums.push((datetime, credits as i64 - debits as i64));
                    }

                    let mut sum = 0;
                    let mut dt = datetime_sums.first().unwrap().0;
                    let today = chrono::Utc::now()
                        .duration_trunc(chrono::Duration::days(1))
                        .unwrap();

                    // fill in sum data for days without transactions
                    let mut balance_history = vec![];
                    while dt <= today {
                        // get all transactions for this date
                        let txns = datetime_sums.iter().filter(|(date, _)| *date == dt);

                        // sum up balance amount
                        sum = sum + txns.map(|x| x.1).collect::<Vec<_>>().iter().sum::<i64>();

                        // convert to grin units
                        let grin_sum = (sum as f64 / grin_gui_core::GRIN_BASE as f64) as f64;
                        balance_history.push((dt.to_owned(), grin_sum));

                        dt = dt + chrono::Duration::days(1);
                    }

                    // finally we update state with the newly constructed balance history
                    state.balance_data = balance_history;
                }
            }
        }
        LocalViewInteraction::TxListUpdateFailure(err) => {
            grin_gui.error = err.write().unwrap().take();
            if let Some(e) = grin_gui.error.as_ref() {
                log_error(e);
            }
        }
    }

    Ok(Command::none())
}

pub fn data_container<'a>(config: &'a Config, home_state: &'a super::home::StateContainer, state: &'a StateContainer) -> Container<'a, Message> {
    let button_height = Length::Fixed(BUTTON_HEIGHT);
    let button_width = Length::Fixed(BUTTON_WIDTH);

    let title = Text::new(localized_string("tx-list")).size(DEFAULT_HEADER_FONT_SIZE);
    let title_container = Container::new(title)
        .style(grin_gui_core::theme::ContainerStyle::BrightBackground)
        .padding(iced::Padding::from([
            0, // top
            0, // right
            0, // bottom
            5, // left
        ]));

    let latest_container =
        Container::new(Text::new(localized_string("tx-recent")).size(DEFAULT_FONT_SIZE))
            .width(button_width)
            .align_y(alignment::Vertical::Center)
            .align_x(alignment::Horizontal::Center);

    let latest_button = Button::new(latest_container).width(button_width).on_press(
        Interaction::WalletOperationHomeTxListDisplayInteraction(LocalViewInteraction::SelectMode(
            Mode::Recent,
        )),
    );

    let latest_button = if state.mode == Mode::Recent {
        latest_button.style(grin_gui_core::theme::ButtonStyle::Selected)
    } else {
        latest_button.style(grin_gui_core::theme::ButtonStyle::Primary)
    };

    let latest_button: Element<Interaction> = latest_button.into();

    // add a nice double border around our buttons
    // TODO refactor since many of the buttons around the UI repeat this theme
    let latest_container_wrap = Container::new(latest_button.map(Message::Interaction)).padding(1);
    let latest_container_wrap = Container::new(latest_container_wrap)
        .style(grin_gui_core::theme::ContainerStyle::Segmented)
        .padding(1);

    let outstanding_container =
        Container::new(Text::new(localized_string("tx-outstanding")).size(DEFAULT_FONT_SIZE))
            .width(button_width)
            .align_y(alignment::Vertical::Center)
            .align_x(alignment::Horizontal::Center);

    let outstanding_button = Button::new(outstanding_container)
        .width(button_width)
        .on_press(Interaction::WalletOperationHomeTxListDisplayInteraction(
            LocalViewInteraction::SelectMode(Mode::Outstanding),
        ));

    let outstanding_button = if state.mode == Mode::Outstanding {
        outstanding_button.style(grin_gui_core::theme::ButtonStyle::Selected)
    } else {
        outstanding_button.style(grin_gui_core::theme::ButtonStyle::Primary)
    };

    let outstanding_button: Element<Interaction> = outstanding_button.into();

    let outstanding_container_wrap =
        Container::new(outstanding_button.map(Message::Interaction)).padding(1);
    let outstanding_container_wrap = Container::new(outstanding_container_wrap)
        .style(grin_gui_core::theme::ContainerStyle::Segmented)
        .padding(1);

    // add additional buttons here
    let button_row = Row::new()
        .push(latest_container_wrap)
        .push(Space::with_width(Length::Fixed(DEFAULT_PADDING)))
        .push(outstanding_container_wrap);

    /*let segmented_mode_container = Container::new(button_row).padding(1);
    let segmented_mode_control_container = Container::new(segmented_mode_container)
        .style(grin_gui_core::theme::ContainerStyle::Segmented)
        .padding(1);*/

    let header_row = Row::new()
        .push(title_container)
        .push(Space::with_width(Length::Fill))
        .push(button_row)
        .align_items(Alignment::Center);

    let header_container = Container::new(header_row).padding(iced::Padding::from([
        0,               // top
        0,               // right
        DEFAULT_PADDING as u16, // bottom
        0,               // left
    ]));

    // TRANSACTION HEADER
    let column_config = state.tx_header_state.column_config();

    // Tx row titles is a row of titles above the tx scrollable.
    // This is to add titles above each section of the tx row, to let
    // the user easily identify what the value is.
    let table_header_row = super::tx_list::titles_row_header(
        &state.wallet_txs,
        &state.tx_header_state.state,
        &state.tx_header_state.columns,
        state.tx_header_state.previous_column_key,
        state.tx_header_state.previous_sort_direction,
    );

    let table_header_container = Container::new(table_header_row).padding(iced::Padding::from([
        0,                   // top
        DEFAULT_PADDING as u16 * 3, // right - should roughly match width of content scroll bar to align table headers
        0,                   // bottom
        0,                   // left
    ]));
    //.style(grin_gui_core::theme::ContainerStyle::PanelForeground);

    // A scrollable list containing rows.
    // Each row holds data about a single tx.
    let mut content = Column::new().spacing(1);
    //.height(Length::Fill)
    //.style(grin_gui_core::theme::ScrollableStyles::Primary);

    let mut has_txs = false;

    // Loops though the txs.
    for (idx, tx_wrap) in state.wallet_txs.txs.iter().enumerate() {
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
            tx_wrap,
            is_tx_expanded,
            &state.expanded_type,
            config,
            &column_config,
            is_odd,
            &None,
            home_state.node_synched,
        );

        // Adds the addon data cell to the scrollable.
        content = content.push(tx_data_cell);
    }

    let mut tx_list_scrollable =
        Scrollable::new(content).style(grin_gui_core::theme::ScrollableStyle::Primary);

    // This column gathers all the tx list elements together.
    let mut tx_list_content = Column::new();

    // Adds the rest of the elements to the content column.
    if has_txs {
        tx_list_content = tx_list_content.push(tx_list_scrollable);
    } else {
        let no_txs_label = if state.mode == Mode::NotInit {
            Text::new(localized_string("txs-list-loading")).size(DEFAULT_FONT_SIZE)
        } else {
            Text::new(localized_string("no-txs-list")).size(DEFAULT_FONT_SIZE)
        };
        let no_txs_container = Container::new(no_txs_label)
            .style(grin_gui_core::theme::ContainerStyle::BrightBackground)
            .padding(iced::Padding::from([
                0, // top
                0, // right
                0, // bottom
                5, // left
            ]));
        tx_list_content = tx_list_content.push(no_txs_container);
    }

    // TRANSACTION LISTING

    let column = Column::new()
        .push(header_container)
        .push(table_header_container)
        .push(tx_list_content);

    // Returns the final container.
    Container::new(column)
        .padding(iced::Padding::from([
            DEFAULT_PADDING, // top
            DEFAULT_PADDING, // right
            DEFAULT_PADDING, // bottom
            DEFAULT_PADDING, // left
        ]))
        .style(grin_gui_core::theme::ContainerStyle::PanelBordered)
}
