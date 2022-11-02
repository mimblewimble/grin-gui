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
    serde::{Deserialize, Serialize},
    std::sync::{Arc, RwLock},
};

pub struct StateContainer {
    pub create_tx_button_state: button::State,
    pub apply_tx_button_state: button::State,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            create_tx_button_state: Default::default(),
            apply_tx_button_state: Default::default(),
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
                Action::CreateTx => grin_gui.wallet_state.operation_state.mode = crate::gui::element::wallet::operation::Mode::CreateTx,
                Action::ApplyTx => grin_gui.wallet_state.operation_state.mode = crate::gui::element::wallet::operation::Mode::ApplyTx,
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
    // Buttons to perform wallet operations
    let mut create_tx_button: Button<Interaction> = Button::new(
        &mut state.create_tx_button_state,
        Text::new(localized_string("wallet-create-tx")).size(DEFAULT_FONT_SIZE),
    )
    .on_press(Interaction::WalletOperationHomeActionMenuViewInteraction(
        LocalViewInteraction::SelectAction(Action::CreateTx),
    ));

    let mut apply_tx_button: Button<Interaction> = Button::new(
        &mut state.apply_tx_button_state,
        Text::new(localized_string("wallet-apply-tx")).size(DEFAULT_FONT_SIZE),
    )
    .on_press(Interaction::WalletOperationHomeActionMenuViewInteraction(
        LocalViewInteraction::SelectAction(Action::ApplyTx),
    ));

    create_tx_button = create_tx_button.style(style::BrightTextButton(color_palette));
    apply_tx_button = apply_tx_button.style(style::BrightTextButton(color_palette));

    let create_tx_button: Element<Interaction> = create_tx_button.into();
    let apply_tx_button: Element<Interaction> = apply_tx_button.into();

    let menu_column = Column::new()
        .push(create_tx_button.map(Message::Interaction))
        .push(apply_tx_button.map(Message::Interaction));

    let menu_container = Container::new(menu_column).padding(2);

    Container::new(menu_container)
        .center_y()
        .center_x()
        .width(Length::Fill)
}
