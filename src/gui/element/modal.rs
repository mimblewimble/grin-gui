use iced::Renderer;

use {
    super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, SMALLER_FONT_SIZE},
    crate::gui::{style, Interaction, Message},
    crate::localization::localized_string,
    grin_gui_core::theme::ColorPalette,
    iced::{
        alignment, button, Alignment, Button, Column, Container, Element, Length, Row, Space, Text,
    },
    iced_aw::{modal, native::card::Card, Modal},
};

pub struct StateContainer {
    ok_state: button::State,
    cancel_state: button::State,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            ok_state: Default::default(),
            cancel_state: Default::default(),
        }
    }
}

pub fn exit_card<'a>(
    color_palette: ColorPalette,
    state: &'a mut StateContainer,
) -> Card<Message, Renderer> {
    let yes_button_label =
        Container::new(Text::new(localized_string("yes")).size(DEFAULT_FONT_SIZE))
            .center_x()
            .align_x(alignment::Horizontal::Center);

    let cancel_button_label =
        Container::new(Text::new(localized_string("no")).size(DEFAULT_FONT_SIZE))
            .center_x()
            .align_x(alignment::Horizontal::Center);

    let yes_button: Element<Interaction> =
        Button::new(&mut state.ok_state, yes_button_label)
            .style(style::DefaultBoxedButton(color_palette))
            .on_press(Interaction::Exit)
            .into();

    let cancel_button: Element<Interaction> =
        Button::new(&mut state.cancel_state, cancel_button_label)
            .style(style::DefaultBoxedButton(color_palette))
            .on_press(Interaction::ExitCancel)
            .into();

    let unit_spacing = 15;

    let button_row = Row::new()
        .push(yes_button.map(Message::Interaction))
        .push(Space::new(Length::Units(unit_spacing), Length::Units(0)))
        .push(cancel_button.map(Message::Interaction));

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
    .on_close(Message::Interaction(Interaction::CloseErrorModal))
    .style(style::NormalModalCardContainer(color_palette))
}

pub fn error_card<'a>(
    color_palette: ColorPalette,
    state: &'a mut StateContainer,
    error_cause: String,
) -> Card<Message, Renderer> {
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
                    &mut state.cancel_state,
                    Text::new(localized_string("ok-caps"))
                        .size(DEFAULT_FONT_SIZE)
                        .horizontal_alignment(alignment::Horizontal::Center),
                )
                .style(style::DefaultButton(color_palette))
                .on_press(Message::Interaction(Interaction::CloseErrorModal)),
            )
            .push(
                Button::new(
                    &mut state.ok_state,
                    Text::new(localized_string("copy-to-clipboard"))
                        .size(SMALLER_FONT_SIZE)
                        .horizontal_alignment(alignment::Horizontal::Center),
                )
                .style(style::NormalTextButton(color_palette))
                .on_press(Message::Interaction(Interaction::WriteToClipboard(
                    error_cause,
                ))),
            ),
    )
    .max_width(500)
    .on_close(Message::Interaction(Interaction::CloseErrorModal))
    .style(style::NormalModalCardContainer(color_palette))
}
