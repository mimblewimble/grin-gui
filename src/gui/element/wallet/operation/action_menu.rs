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
    std::sync::{Arc, RwLock},
    serde::{Deserialize, Serialize}
};

pub struct StateContainer {
    pub create_tx_button_state: button::State,
    pub action_menu_action: Action,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            action_menu_action: Action::None,
            create_tx_button_state: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    None,
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
    let state = &mut grin_gui.wallet_state.operation_state.home_state.action_menu_state;
    match message {
        LocalViewInteraction::SelectAction(action) => {
            log::debug!("Interaction::WalletOperationHomeActionMenuViewInteraction({:?})", action);
            grin_gui.wallet_state.operation_state.home_state.action_menu_state.action_menu_action = action;
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
    let menu_column = Column::new();
    let menu_container = Container::new(menu_column).width(Length::FillPortion(2));

    let mut create_tx_button: Button<Interaction> = Button::new(
        &mut state.create_tx_button_state,
        Text::new(localized_string("wallet")).size(DEFAULT_FONT_SIZE),
    )
    .on_press(Interaction::WalletOperationHomeActionMenuViewInteraction(
        LocalViewInteraction::SelectAction(Action::CreateTx),
    ));

    // Overall operations menu screen layout column
    let column = Column::new()
        .push(menu_container)
        .align_items(Alignment::Center);

    Container::new(column)
        .center_y()
        .center_x()
        .width(Length::Fill)
}
