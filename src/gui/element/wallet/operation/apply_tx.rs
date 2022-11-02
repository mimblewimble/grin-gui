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
    pub back_button_state: button::State,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            back_button_state: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    Back,
}

pub fn handle_message<'a>(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    /*let state = &mut grin_gui
        .wallet_state
        .operation_state
        .home_state
        .action_menu_state;*/
    match message {
        LocalViewInteraction::Back => {
            log::debug!(
                "Interaction::WalletOperationApplyTxViewInteraction(Back)"
            );
            grin_gui.wallet_state.operation_state.mode = crate::gui::element::wallet::operation::Mode::Home;
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
    let title = Text::new(localized_string("apply-tx"))
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
            .on_press(Interaction::WalletOperationApplyTxViewInteraction(
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

    let column = Column::new()
        .push(title_row)
        .padding(10) 
        .align_items(Alignment::Center);

    Container::new(column)
        .center_y()
        .center_x()
        .width(Length::Fill)
}
