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
use iced_native::Widget;
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
    // Transaction that we're viewing
    pub current_tx: Option<TxLogEntry>,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            current_tx: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    Back,
}

pub fn handle_message<'a>(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let state = &mut grin_gui.wallet_state.operation_state.create_tx_state;

    match message {
        LocalViewInteraction::Back => {
            log::debug!("Interaction::WalletOperationTxDetailViewInteraction(Back)");
            grin_gui.wallet_state.operation_state.mode =
                crate::gui::element::wallet::operation::Mode::Home;
        }
   }

    Ok(Command::none())
}

pub fn data_container<'a>(config: &'a Config, state: &'a StateContainer) -> Container<'a, Message> {
    // Title row
    let title = Text::new(localized_string("tx-details"))
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

    let header_row = Row::new().push(title_container);

    let header_container = Container::new(header_row).padding(iced::Padding::from([
        0,               // top
        0,               // right
        DEFAULT_PADDING, // bottom
        0,               // left
    ]));

    let amount_label = Text::new(localized_string("create-tx-amount"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let amount_label_container =
        Container::new(amount_label).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let amount_value = Text::new(localized_string("create-tx-amount"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let amount_value_container =
        Container::new(amount_value).style(grin_gui_core::theme::ContainerStyle::NormalBackground);
    
    let amount_row = Row::new()
    .push(amount_label_container)
    .push(amount_value_container);


    let unit_spacing = 15;
    /*let button_row = Row::new()
        .push(submit_container)
        .push(Space::new(Length::Units(unit_spacing), Length::Units(0)))
        .push(cancel_container);*/

    let mut column = Column::new()
        .push(amount_row)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)));
        /* .push(address_instruction_container)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(recipient_address_input.map(Message::Interaction))
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)));*/

    /*column = column
        .push(amount_container)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(amount_input.map(Message::Interaction))
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)));*/

    /*if state.amount_error {
        column = column
            .push(amount_error_container)
            .push(Space::new(Length::Units(0), Length::Units(unit_spacing)));
    }*/

    /*column = column
        .push(button_row)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(Space::new(
            Length::Units(0),
            Length::Units(unit_spacing + 10),
        ));*/

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
