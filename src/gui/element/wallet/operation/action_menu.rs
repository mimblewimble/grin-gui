use super::tx_list::{self, ExpandType};
use crate::log_error;
use async_std::prelude::FutureExt;
use grin_gui_core::{
    config::Config,
    wallet::{TxLogEntry, TxLogEntryType},
};
//use grin_gui_widgets::{header};
//use grin_gui_core::widgets::widget::header;
use iced::alignment;
use iced_aw::Card;
use iced_native::Widget;
use std::path::PathBuf;

use super::tx_list::{HeaderState, TxList};

use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING},
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
    iced::{Alignment, Command, Length},
    serde::{Deserialize, Serialize},
    std::sync::{Arc, RwLock},
};

pub struct StateContainer {
    // pub create_tx_button_state: button::State,
    // pub apply_tx_button_state: button::State,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            // create_tx_button_state: Default::default(),
            // apply_tx_button_state: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    CreateTx,
    ApplyTx,
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    SelectAction(Action),
}

pub fn handle_message<'a>(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let state = &mut grin_gui
        .wallet_state
        .operation_state
        .home_state
        .action_menu_state;
    match message {
        LocalViewInteraction::SelectAction(action) => {
            log::debug!(
                "Interaction::WalletOperationHomeActionMenuViewInteraction({:?})",
                action
            );
            match action {
                Action::CreateTx => {
                    grin_gui.wallet_state.operation_state.mode =
                        crate::gui::element::wallet::operation::Mode::CreateTx
                }
                Action::ApplyTx => {
                    grin_gui.wallet_state.operation_state.mode =
                        crate::gui::element::wallet::operation::Mode::ApplyTx
                }
            }
        }
    }
    Ok(Command::none())
}

pub fn data_container<'a>(
    config: &'a Config,
    state: &'a StateContainer,
    home_state: &'a super::home::StateContainer,
) -> Container<'a, Message> {
    let button_width = Length::Fixed(70.0);

    let description = Text::new(localized_string("tx-transact"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center);
    let description_container = Container::new(description).padding(iced::Padding::from([
        7, // top
        5, // right
        5, // bottom
        5, // left
    ]));

    // Buttons to perform wallet operations
    let create_tx_container =
        Container::new(Text::new(localized_string("wallet-create-tx")).size(DEFAULT_FONT_SIZE))
            .width(button_width)
            .align_y(alignment::Vertical::Center)
            .align_x(alignment::Horizontal::Center);

    let mut create_tx_button = Button::new(create_tx_container)
        .width(button_width)
        .style(grin_gui_core::theme::ButtonStyle::Primary);

    if home_state.node_synched {
        create_tx_button = create_tx_button
            .on_press(Interaction::WalletOperationHomeActionMenuViewInteraction(
                LocalViewInteraction::SelectAction(Action::CreateTx),
            ))
    } 

    let create_tx_button: Element<Interaction> = create_tx_button.into();

    let apply_tx_container =
        Container::new(Text::new(localized_string("wallet-apply-tx")).size(DEFAULT_FONT_SIZE))
            .width(button_width)
            .align_y(alignment::Vertical::Center)
            .align_x(alignment::Horizontal::Center);

    let mut apply_tx_button = Button::new(apply_tx_container)
        .width(button_width)
        .style(grin_gui_core::theme::ButtonStyle::Primary);

    if home_state.node_synched {
        apply_tx_button = apply_tx_button
            .on_press(Interaction::WalletOperationHomeActionMenuViewInteraction(
                LocalViewInteraction::SelectAction(Action::ApplyTx),
            ))
    } 

    let apply_tx_button: Element<Interaction> = apply_tx_button.into();

    // TODO refactor since many of the buttons around the UI repeat this theme
    let create_container = Container::new(create_tx_button.map(Message::Interaction)).padding(1);
    let create_container = Container::new(create_container)
        .style(grin_gui_core::theme::ContainerStyle::Segmented)
        .padding(1);

    let apply_container = Container::new(apply_tx_button.map(Message::Interaction)).padding(1);
    let apply_container = Container::new(apply_container)
        .style(grin_gui_core::theme::ContainerStyle::Segmented)
        .padding(1);

    let menu_column = Row::new()
        .push(description_container)
        .push(Space::with_width(Length::Fixed(DEFAULT_PADDING)))
        .push(create_container)
        .push(Space::with_width(Length::Fixed(DEFAULT_PADDING)))
        .push(apply_container);

    Container::new(menu_column).padding(iced::Padding::from([
        5, // top
        5, // right
        5, // bottom
        5, // left
    ]))
}
