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
    // Slatepack data itself as read, or instructions
    pub slatepack_read_data: String,
    // whether we can continue
    pub can_continue: bool,
    // confirmation state, in separate panel
    pub confirm_state: super::apply_tx_confirm::StateContainer,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            slatepack_read_data: localized_string("tx-slatepack-read-result-default"),
            can_continue: false,
            confirm_state: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    Back,
    Continue,
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
            state.slatepack_read_data = localized_string("tx-slatepack-read-result-default");
            grin_gui.wallet_state.operation_state.mode =
                crate::gui::element::wallet::operation::Mode::Home;
            state.confirm_state.slatepack_parsed = None;
            state.can_continue = false;
        }
        LocalViewInteraction::ReadFromClipboardSuccess(value) => {
            debug!("Read from clipboard: {}", value);
            let w = grin_gui.wallet_interface.clone();
            let decode_res = WalletInterface::decrypt_slatepack(w, value.clone());
            match decode_res {
                Err(e) => {
                    state.slatepack_read_data = localized_string("tx-slatepack-read-failure");
                    state.confirm_state.slatepack_parsed = None;
                    state.can_continue = false;
                }
                Ok(s) => {
                    debug!("{}", s.0);
                    state.slatepack_read_data = value;
                    state.confirm_state.slatepack_parsed = Some(s);
                    state.can_continue = true;
                }
            }
        }
        LocalViewInteraction::Continue => {
            //state.slatepack_read_data = localized_string("tx-slatepack-read-result-default");
            state.can_continue = false;
            let fut = move || async {};
            return Ok(Command::perform(fut(), |_| {
                return Message::Interaction(
                        Interaction::WalletOperationApplyTxConfirmViewInteraction(
                            crate::gui::element::wallet::operation::apply_tx_confirm::LocalViewInteraction::Accept
                        ),
                    );
            }));
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

    let encrypted_slate_card = Card::new(
        Text::new(localized_string("apply-tx")).size(DEFAULT_HEADER_FONT_SIZE),
        Text::new(&state.slatepack_read_data).size(DEFAULT_FONT_SIZE),
    )
    .foot(
        Column::new()
            .spacing(10)
            .padding(5)
            .width(Length::Fill)
            .align_items(Alignment::Center),
    )
    .max_width(400)
    .style(grin_gui_core::theme::CardStyle::Normal);

    let mut slatepack_area = Column::new().push(encrypted_slate_card);
    if state.can_continue {
        // Add parsed slatepack contents area here
        let parsed_slate_content =
            super::apply_tx_confirm::data_container(config, &state.confirm_state);
        slatepack_area = slatepack_area
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(parsed_slate_content);
    }

    let slatepack_area_container = Container::new(slatepack_area);

    let button_height = Length::Units(BUTTON_HEIGHT);
    let button_width = Length::Units(BUTTON_WIDTH);

    let submit_button_label_container =
        Container::new(Text::new(localized_string("tx-paste")).size(DEFAULT_FONT_SIZE))
            .width(button_width)
            .height(button_height)
            .center_x()
            .center_y()
            .align_x(alignment::Horizontal::Center);

    let mut submit_button = Button::new(submit_button_label_container)
        .style(grin_gui_core::theme::ButtonStyle::Primary)
        .on_press(Interaction::ReadSlatepackFromClipboard);

    let submit_button: Element<Interaction> = submit_button.into();

    let continue_button_label_container =
        Container::new(Text::new(localized_string("tx-continue")).size(DEFAULT_FONT_SIZE))
            .width(button_width)
            .height(button_height)
            .center_x()
            .center_y()
            .align_x(alignment::Horizontal::Center);

    let continue_button: Element<Interaction> = Button::new(continue_button_label_container)
        .style(grin_gui_core::theme::ButtonStyle::Primary)
        .on_press(Interaction::WalletOperationApplyTxViewInteraction(
            LocalViewInteraction::Continue,
        ))
        .into();

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

    let continue_container = Container::new(continue_button.map(Message::Interaction)).padding(1);
    let continue_container = Container::new(continue_container)
        .style(grin_gui_core::theme::ContainerStyle::Segmented)
        .padding(1);

    let cancel_container = Container::new(cancel_button.map(Message::Interaction)).padding(1);
    let cancel_container = Container::new(cancel_container)
        .style(grin_gui_core::theme::ContainerStyle::Segmented)
        .padding(1);

    let mut button_row = Row::new()
        .push(submit_container)
        .push(Space::new(Length::Units(unit_spacing), Length::Units(0)));

    if state.can_continue {
        button_row = button_row
            .push(continue_container)
            .push(Space::new(Length::Units(unit_spacing), Length::Units(0)))
    }

    button_row = button_row.push(cancel_container);

    let column = Column::new()
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

    let wrapper_column = Column::new().height(Length::Fill).push(content);

    // Returns the final container.
    Container::new(wrapper_column).padding(iced::Padding::from([
        DEFAULT_PADDING, // top
        DEFAULT_PADDING, // right
        DEFAULT_PADDING, // bottom
        DEFAULT_PADDING, // left
    ]))
}
