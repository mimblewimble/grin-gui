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
        DEFAULT_SUB_HEADER_FONT_SIZE, SMALLER_FONT_SIZE,
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
    // Entire slatepack data
    pub slatepack_read_data_full: String,
    // whether we can continue
    pub can_continue: bool,
    // confirmation state, in separate panel
    pub confirm_state: super::apply_tx_confirm::StateContainer,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            slatepack_read_data: localized_string("tx-slatepack-read-result-default"),
            slatepack_read_data_full: Default::default(),
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
    ShowSlate,
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
            state.slatepack_read_data_full = Default::default();
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
                    state.slatepack_read_data_full = Default::default();
                    state.can_continue = false;
                }
                Ok(s) => {
                    debug!("{}", s.1);
                    // Truncate a bit for compact display purposes
                    let mut s1 = value.clone();
                    s1.truncate(27);
                    let s2 = value
                        .clone()
                        .split_off(usize::saturating_sub(value.len(), 23));
                    let short_display = format!("{}...{}", s1, s2);

                    state.slatepack_read_data_full = value.clone();
                    state.slatepack_read_data = short_display;
                    state.confirm_state.slatepack_parsed = Some(s);
                    state.can_continue = true;
                }
            }
        }
        LocalViewInteraction::Continue => {
            state.slatepack_read_data = localized_string("tx-slatepack-read-result-default");
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
        LocalViewInteraction::ShowSlate => {
            // ensure back button on showing slate screen comes back here
            grin_gui
                .wallet_state
                .operation_state
                .show_slatepack_state
                .submit_mode = Some(crate::gui::element::wallet::operation::Mode::ApplyTx);
            grin_gui
                .wallet_state
                .operation_state
                .show_slatepack_state
                .encrypted_slate = Some(state.slatepack_read_data_full.clone());

            grin_gui.wallet_state.operation_state.mode =
                crate::gui::element::wallet::operation::Mode::ShowSlatepack;
        }
        LocalViewInteraction::Address(_) => {}
        LocalViewInteraction::ApplyTransaction(_) => {}
    }
    Ok(Command::none())
}

pub fn data_container<'a>(config: &'a Config, state: &'a StateContainer) -> Container<'a, Message> {
    let unit_spacing = 15.0;
    let mut title_key = localized_string("apply-tx");

    // Just display Signing... and return while signing futures are being called
    if state.confirm_state.is_signing {
        title_key = localized_string("signing-tx");
    }

    // Title row
    let title = Text::new(title_key)
        .size(DEFAULT_HEADER_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);

    let title_container = Container::new(title)
        .style(grin_gui_core::theme::ContainerStyle::BrightBackground)
        .padding(iced::Padding::from([
            2, // top
            0, // right
            2, // bottom
            0, // left
        ]));

    // push more items on to header here: e.g. other buttons, things that belong on the header
    let header_row = Row::new().push(title_container);

    let header_container = Container::new(header_row).padding(iced::Padding::from([
        0,               // top
        0,               // right
        DEFAULT_PADDING, // bottom
        0,               // left
    ]));

    if state.confirm_state.is_signing {
        let column = Column::new().push(header_container);

        let form_container =
            Container::new(column)
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
        return Container::new(wrapper_column).padding(iced::Padding::from([
            DEFAULT_PADDING, // top
            DEFAULT_PADDING, // right
            DEFAULT_PADDING, // bottom
            DEFAULT_PADDING, // left
        ]));
    }

    let paste_instruction = Text::new(state.slatepack_read_data.clone())
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let paste_instruction_container = Container::new(paste_instruction)
        .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let mut instruction_row = Row::new().push(paste_instruction_container);

    if state.can_continue {
        let show_slate_label_container = Container::new(
            Text::new(localized_string("tx-show-full-slatepack")).size(SMALLER_FONT_SIZE),
        )
        .height(Length::Fixed(14))
        .width(Length::Fixed(60))
        .center_y()
        .center_x();

        let show_slate_button: Element<Interaction> = Button::new(show_slate_label_container)
            .style(grin_gui_core::theme::ButtonStyle::Bordered)
            .on_press(Interaction::WalletOperationApplyTxViewInteraction(
                LocalViewInteraction::ShowSlate,
            ))
            .padding(2)
            .into();

        let paste_again_label_container = Container::new(
            Text::new(localized_string("tx-paste-again")).size(SMALLER_FONT_SIZE),
        )
        .height(Length::Fixed(14))
        .width(Length::Fixed(60))
        .center_y()
        .center_x();

        let paste_again_button: Element<Interaction> = Button::new(paste_again_label_container)
            .style(grin_gui_core::theme::ButtonStyle::Bordered)
            .on_press(Interaction::ReadSlatepackFromClipboard)
            .padding(2)
            .into();

        instruction_row = instruction_row
            .push(Space::with_width(Length::Fixed(2)))
            .push(show_slate_button.map(Message::Interaction))
            .push(Space::with_width(Length::Fixed(2)))
            .push(paste_again_button.map(Message::Interaction));
    }

    let mut instruction_col = Column::new();

    if state.can_continue {
        let pasted_tx_label = Text::new(localized_string("pasted-slatepack"))
            .size(DEFAULT_SUB_HEADER_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);
        let pasted_tx_label_container = Container::new(pasted_tx_label)
            .style(grin_gui_core::theme::ContainerStyle::BrightBackground);
        instruction_col = instruction_col
            .push(pasted_tx_label_container)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
    }

    instruction_col = instruction_col.push(instruction_row);

    if state.can_continue {
        let decrypted_tx_label = Text::new(localized_string("pasted-slatepack-details"))
            .size(DEFAULT_SUB_HEADER_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let decrypted_tx_label_container = Container::new(decrypted_tx_label)
            .style(grin_gui_core::theme::ContainerStyle::BrightBackground);
        instruction_col = instruction_col
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
            .push(decrypted_tx_label_container)
    }

    let mut slatepack_area = Column::new();
    if state.can_continue {
        // Add parsed slatepack contents area here
        let parsed_slate_content =
            super::apply_tx_confirm::data_container(config, &state.confirm_state);

        slatepack_area = slatepack_area.push(parsed_slate_content);
    }

    let slatepack_area_container = Container::new(slatepack_area);

    let button_height = Length::Fixed(BUTTON_HEIGHT);
    let button_width = Length::Fixed(BUTTON_WIDTH);

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

    let mut button_row = Row::new();
    if !state.can_continue {
        button_row = button_row.push(submit_container)
        .push(Space::new(Length::Fixed(unit_spacing), Length::Fixed(0.0)));
    } else {
        button_row = button_row
            .push(continue_container)
            .push(Space::new(Length::Fixed(unit_spacing), Length::Fixed(0.0)))
    }

    button_row = button_row.push(cancel_container);

    let column = Column::new()
        .push(header_container)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
        .push(instruction_col)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
        .push(slatepack_area_container)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
        .push(button_row)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
        .push(Space::new(
            Length::Fixed(0.0),
            Length::Fixed(unit_spacing + 10),
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
