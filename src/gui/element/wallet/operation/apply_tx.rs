use super::tx_list::{self, ExpandType};
use crate::log_error;
use async_std::prelude::FutureExt;
use grin_gui_core::{
    config::Config,
    wallet::{TxLogEntry, TxLogEntryType},
};
use grin_gui_widgets::widget::header;
use iced_aw::Card;
use iced_native::Widget;
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
    grin_gui_core::wallet::{StatusMessage, WalletInfo, WalletInterface},
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
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            // back_button_state: Default::default(),
            // copy_address_button_state: Default::default(),
            // address_state: Default::default(),
            address_value: Default::default(),
            slatepack_read_result: localized_string("tx-slatepack-read-result-default"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    Back,
    Address(String),
    ApplyTransaction(String),
    ReadFromClipboardSuccess(String),
    ReadFromClipboardFailure,
}

pub fn handle_message<'a>(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let state = &mut grin_gui.wallet_state.operation_state.apply_tx_state;
    match message {
        LocalViewInteraction::Back => {
            log::debug!("Interaction::WalletOperationApplyTxViewInteraction(Back)");
            grin_gui.wallet_state.operation_state.mode =
                crate::gui::element::wallet::operation::Mode::Home;
        }
        LocalViewInteraction::ReadFromClipboardSuccess(value) => {
            debug!("Read from clipboard: {}", value);
            let w = grin_gui.wallet_interface.clone();
            let decode_res = WalletInterface::decrypt_slatepack(w, value);
            match decode_res {
                Err(e) => {
                    state.slatepack_read_result = localized_string("tx-slatepack-read-failure")
                }
                Ok(s) => {
                    debug!("{}", s.0);
                    grin_gui.wallet_state.operation_state.apply_tx_confirm_state.slatepack_parsed = Some(s);
                    grin_gui.wallet_state.operation_state.mode =
                        crate::gui::element::wallet::operation::Mode::ApplyTxConfirm;
                }
            }
        }
        LocalViewInteraction::ReadFromClipboardFailure => {
            error!("Failed to read from clipboard");
        }
        LocalViewInteraction::Address(_) => {}
        LocalViewInteraction::ApplyTransaction(_) => {}
    }
    Ok(Command::none())
}

pub fn data_container<'a>(config: &'a Config, state: &'a StateContainer) -> Container<'a, Message> {
    let unit_spacing = 15;

    // Title row
    let title = Text::new(localized_string("apply-tx"))
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

    // push more items on to header here: e.g. other buttons, things that belong on the header
    let header_row = Row::new().push(title_container);

    let header_container = Container::new(header_row).padding(iced::Padding::from([
        0,               // top
        0,               // right
        DEFAULT_PADDING, // bottom
        0,               // left
    ]));

    let address_name = Text::new(localized_string("slatepack-address-name"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let address_name_container =
        Container::new(address_name).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let address_input = TextInput::new("", &state.address_value, |s| {
        Interaction::WalletOperationApplyTxViewInteraction(LocalViewInteraction::Address(s))
    })
    .size(DEFAULT_FONT_SIZE)
    .padding(6)
    .width(Length::Units(400))
    .style(grin_gui_core::theme::TextInputStyle::AddonsQuery);

    let address_input: Element<Interaction> = address_input.into();

    let copy_address_button = Button::new(
        // &mut state.copy_address_button_state,
        Text::new(localized_string("copy-to-clipboard"))
            .size(SMALLER_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Center),
    )
    .style(grin_gui_core::theme::ButtonStyle::NormalText)
    .on_press(Interaction::WriteToClipboard(state.address_value.clone()));

    let copy_address_button: Element<Interaction> = copy_address_button.into();

    let address_row = Row::new()
        .push(address_input)
        .push(copy_address_button)
        .spacing(DEFAULT_PADDING);

    let address_row: Element<Interaction> = address_row.into();

    let address_instruction_container = Text::new(localized_string("address-instruction"))
        .size(SMALLER_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let address_instruction_container = Container::new(address_instruction_container)
        .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let slatepack_paste_name = Text::new(localized_string("tx-slatepack-paste-transaction-here"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let slatepack_paste_name_container = Container::new(slatepack_paste_name)
        .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let slatepack_text_area = Text::new(state.slatepack_read_result.clone())
        .size(DEFAULT_FONT_SIZE)
        .width(Length::Units(400));

    /*let paste_slatepack_button = Button::new(
        // &mut state.copy_address_button_state,
        Text::new(localized_string("tx-slatepack-paste-from-clipboard"))
            .size(SMALLER_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Center),
    )
    .style(grin_gui_core::theme::ButtonStyle::NormalText)
    .on_press(Interaction::ReadSlatepackFromClipboard);

    let paste_slatepack_button: Element<Interaction> = paste_slatepack_button.into();*/

    let paste_slatepack_row = Row::new()
        .push(slatepack_text_area)
        //.push(paste_slatepack_button.map(Message::Interaction))
        .spacing(DEFAULT_PADDING);

    let slatepack_area = Column::new()
        .push(slatepack_paste_name_container)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(paste_slatepack_row);

    let slatepack_area_container = Container::new(slatepack_area);

    let button_height = Length::Units(BUTTON_HEIGHT);
    let button_width = Length::Units(BUTTON_WIDTH);

    let submit_button_label_container =
        Container::new(Text::new(localized_string("tx-continue")).size(DEFAULT_FONT_SIZE))
            .width(button_width)
            .height(button_height)
            .center_x()
            .center_y()
            .align_x(alignment::Horizontal::Center);

    let mut submit_button = Button::new(submit_button_label_container)
        .style(grin_gui_core::theme::ButtonStyle::Primary)
        .on_press(Interaction::ReadSlatepackFromClipboard);
    /*let submit_button = submit_button.on_press(Interaction::WalletOperationApplyTxViewInteraction(
        LocalViewInteraction::ApplyTransaction("_".into()),
    ));*/

    let submit_button: Element<Interaction> = submit_button.into();

    let cancel_button_label_container =
        Container::new(Text::new(localized_string("cancel")).size(DEFAULT_FONT_SIZE))
            .width(button_width)
            .height(button_height)
            .center_x()
            .center_y()
            .align_x(alignment::Horizontal::Center);

    let cancel_button: Element<Interaction> = Button::new(cancel_button_label_container)
        .style(grin_gui_core::theme::ButtonStyle::Primary)
        .on_press(Interaction::WalletOperationApplyTxViewInteraction(
            LocalViewInteraction::Back,
        ))
        .into();

    let submit_container = Container::new(submit_button.map(Message::Interaction)).padding(1);
    let submit_container = Container::new(submit_container)
        .style(grin_gui_core::theme::ContainerStyle::Segmented)
        .padding(1);

    let cancel_container = Container::new(cancel_button.map(Message::Interaction)).padding(1);
    let cancel_container = Container::new(cancel_container)
        .style(grin_gui_core::theme::ContainerStyle::Segmented)
        .padding(1);

    let button_row = Row::new()
        .push(submit_container)
        .push(Space::new(Length::Units(unit_spacing), Length::Units(0)))
        .push(cancel_container);

    let column = Column::new()
        .push(address_name_container)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(address_instruction_container)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(address_row.map(Message::Interaction))
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(slatepack_area_container)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(button_row)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(Space::new(
            Length::Units(0),
            Length::Units(unit_spacing + 10),
        ));

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
