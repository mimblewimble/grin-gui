use iced::Renderer;

use super::{BUTTON_HEIGHT, BUTTON_WIDTH};

use {
    super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, SMALLER_FONT_SIZE},
    crate::gui::{Interaction, Message},
    crate::localization::localized_string,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::theme::{Card, Container, Column, Button, Element, Scrollable, Text, PickList, Row},
    iced::{alignment, Alignment, Command, Length},
    iced::widget::{
        button, pick_list, scrollable, text_input, Checkbox, Space, TextInput,
    },
    iced_aw::{modal, Modal},
};

pub struct StateContainer {
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
        }
    }
}

pub fn exit_card() -> Card<'static, Message> {
    let button_height = Length::Units(BUTTON_HEIGHT);
    let button_width = Length::Units(BUTTON_WIDTH);

    let yes_button_label =
        Container::new(Text::new(localized_string("yes")).size(DEFAULT_FONT_SIZE))
            .width(button_width)
            .height(button_height)
            .center_x()
            .center_y()
            .align_x(alignment::Horizontal::Center);

    let cancel_button_label =
        Container::new(Text::new(localized_string("no")).size(DEFAULT_FONT_SIZE))
            .width(button_width)
            .height(button_height)
            .center_x()
            .center_y()
            .align_x(alignment::Horizontal::Center);

    let yes_button: Element<Interaction> = Button::new( yes_button_label)
        .style(grin_gui_core::theme::ButtonStyle::Primary)
        .on_press(Interaction::Exit)
        .into();

    let cancel_button: Element<Interaction> =
        Button::new( cancel_button_label)
            .style(grin_gui_core::theme::ButtonStyle::Primary)
            .on_press(Interaction::ExitCancel)
            .into();

    let unit_spacing = 15;

    // button lipstick
    let yes_container = Container::new(yes_button.map(Message::Interaction)).padding(1);
    let yes_container = Container::new(yes_container)
        .style(grin_gui_core::theme::ContainerStyle::Segmented)
        .padding(1);
    let cancel_container = Container::new(cancel_button.map(Message::Interaction)).padding(1);
    let cancel_container = Container::new(cancel_container)
        .style(grin_gui_core::theme::ContainerStyle::Segmented)
        .padding(1);

    let button_row = Row::new()
        .push(yes_container)
        .push(Space::new(Length::Units(unit_spacing), Length::Units(0)))
        .push(cancel_container);

    Card::new(
        Text::new(localized_string("exit-confirm-title"))
            .size(DEFAULT_HEADER_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Center),
        Text::new(localized_string("exit-confirm-msg")).size(DEFAULT_FONT_SIZE),
    )
    .foot(
        Column::new()
            .spacing(10)
            .padding(5)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(button_row),
    )
    .max_width(500)
    .on_close(Message::Interaction(Interaction::ExitCancel))
    .style(grin_gui_core::theme::CardStyle::Normal)
}

pub fn error_card(
    error_cause: String,
) -> Card<'static, Message> {
    Card::new(
        Text::new(localized_string("error-detail")).size(DEFAULT_HEADER_FONT_SIZE),
        Text::new(error_cause.clone()).size(DEFAULT_FONT_SIZE),
    )
    .foot(
        Column::new()
            .spacing(10)
            .padding(5)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(
                Button::new(
                    Text::new(localized_string("ok-caps"))
                        .size(DEFAULT_FONT_SIZE)
                        .horizontal_alignment(alignment::Horizontal::Center),
                )
                .style(grin_gui_core::theme::ButtonStyle::Primary)
                .on_press(Message::Interaction(Interaction::CloseErrorModal)),
            )
            .push(
                Button::new(
                    Text::new(localized_string("copy-to-clipboard"))
                        .size(SMALLER_FONT_SIZE)
                        .horizontal_alignment(alignment::Horizontal::Center),
                )
                .style(grin_gui_core::theme::ButtonStyle::NormalText)
                .on_press(Message::Interaction(Interaction::WriteToClipboard(
                    error_cause,
                ))),
            ),
    )
    .max_width(500)
    .on_close(Message::Interaction(Interaction::CloseErrorModal))
    .style(grin_gui_core::theme::CardStyle::Normal)
}
