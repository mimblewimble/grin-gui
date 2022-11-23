use super::tx_list::{self, ExpandType};
use crate::log_error;
use async_std::prelude::FutureExt;
use grin_gui_core::{
    config::Config,
    wallet::{TxLogEntry, TxLogEntryType},
};
use grin_gui_widgets::{header};
//use iced::button::StyleSheet;
use iced_aw::Card;
use iced_native::Widget;
use std::path::PathBuf;

use super::tx_list::{HeaderState, TxList};

use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, SMALLER_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    anyhow::Context,
    grin_gui_core::wallet::{StatusMessage, WalletInfo, WalletInterface},
    grin_gui_core::{node::amount_to_hr_string, theme::ColorPalette},
    grin_gui_core::theme::{Container, Button, Element, Column, PickList, Row, Scrollable, Text, TextInput, Header, TableRow},
    iced::{alignment, Alignment, Command, Length},
    iced::widget::{
        button, pick_list, scrollable, text_input,Checkbox, Space,
    },
    serde::{Deserialize, Serialize},
    std::sync::{Arc, RwLock},
};

pub struct StateContainer {
    // pub back_button_state: button::State,
    // pub copy_address_button_state: button::State,
    // pub address_state: text_input::State,
    pub address_value: String,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            // back_button_state: Default::default(),
            // copy_address_button_state: Default::default(),
            // address_state: Default::default(),
            address_value: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    Back,
    Address(String),
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
            log::debug!("Interaction::WalletOperationApplyTxViewInteraction(Back)");
            grin_gui.wallet_state.operation_state.mode =
                crate::gui::element::wallet::operation::Mode::Home;
        }
        LocalViewInteraction::Address(_) => {

        }
    }
    Ok(Command::none())
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    config: &'a Config,
    state: &'a StateContainer,
) -> Container<'a, Message> {
    // Title row
    let title = Text::new(localized_string("apply-tx"))
        .size(DEFAULT_HEADER_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);

    let title_container =
        Container::new(title).style(grin_gui_core::theme::container::Container::NormalBackground(color_palette));

    let back_button_label_container =
        Container::new(Text::new(localized_string("back")).size(DEFAULT_FONT_SIZE))
            .height(Length::Units(20))
            .align_y(alignment::Vertical::Bottom)
            .align_x(alignment::Horizontal::Center);

    let back_button: Element<Interaction> =
        Button::new( back_button_label_container)
            .style(grin_gui_core::theme::button::Button::NormalText(color_palette))
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

    let address_name = Text::new(localized_string("slatepack-address-name"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let address_name_container =
        Container::new(address_name).style(grin_gui_core::theme::container::Container::NormalBackground(color_palette));

    let address_input = TextInput::new( "", &state.address_value, |s| {
        Interaction::WalletOperationApplyTxViewInteraction(LocalViewInteraction::Address(s))
    })
    .size(DEFAULT_FONT_SIZE)
    .padding(6)
    .width(Length::Units(400))
    .style(grin_gui_core::theme::text_input::TextInputStyles::AddonsQuery(color_palette));

    let address_input: Element<Interaction> = address_input.into();

    let copy_address_button = Button::new(
        // &mut state.copy_address_button_state,
        Text::new(localized_string("copy-to-clipboard"))
            .size(SMALLER_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Center),
    )
    .style(grin_gui_core::theme::button::Button::NormalText(color_palette))
    .on_press(Interaction::WriteToClipboard(
        state.address_value.clone(),
    ));

    let copy_address_button: Element<Interaction> = copy_address_button.into();

    let address_row = Row::new()
    .push(address_input)
    .push(copy_address_button)
    .spacing(DEFAULT_PADDING);

    let address_row: Element<Interaction> = address_row.into();

    let address_instruction_container = Text::new(localized_string("address-instruction"))
        .size(SMALLER_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let address_instruction_container =
        Container::new(address_instruction_container).style(grin_gui_core::theme::container::Container::NormalBackground(color_palette));


    let column = Column::new()
        .push(title_row)
        .push(address_name_container)
        .push(address_instruction_container)
        .push(address_row.map(Message::Interaction))
        .spacing(DEFAULT_PADDING)
        .align_items(Alignment::Start);

    Container::new(column)
        .center_y()
        .center_x()
        .width(Length::Fill)
}
