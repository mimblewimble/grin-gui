use {
   super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE},
    crate::gui::{style, Interaction, Message},
    crate::localization::localized_string,
    grin_gui_core::theme::ColorPalette,
    iced::{
        alignment, button, Alignment, Button, Column, Container, Element, Length, Row,
        Space, Text,
    },
};

pub struct StateContainer {
    yes_button_state: button::State,
    cancel_button_state: button::State,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            yes_button_state: Default::default(),
            cancel_button_state: Default::default(),
        }
    }
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    state: &'a mut StateContainer,
) -> Container<'a, Message> {

    // Title row
    let title = Text::new(localized_string("exit-confirm-msg"))
        .size(DEFAULT_HEADER_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);

    let title_container = Container::new(title)
        .style(style::BrightBackgroundContainer(color_palette));

    let title_row = Row::new()
        .push(title_container)
        .align_items(Alignment::Center)
        .padding(6)
        .spacing(20);

    let yes_button_label = Container::new(
        Text::new(localized_string("yes")).size(DEFAULT_FONT_SIZE),
    )
    .center_x()
    .align_x(alignment::Horizontal::Center);

    let cancel_button_label =
        Container::new(Text::new(localized_string("cancel")).size(DEFAULT_FONT_SIZE))
            .center_x()
            .align_x(alignment::Horizontal::Center);

    let yes_button: Element<Interaction> = Button::new(
        &mut state.yes_button_state,
        yes_button_label,
    )
    .style(style::DefaultBoxedButton(color_palette))
    .on_press(Interaction::Exit)
    .into();

    let cancel_button: Element<Interaction> = Button::new(
        &mut state.cancel_button_state,
        cancel_button_label,
    )
    .style(style::DefaultBoxedButton(color_palette))
    .on_press(Interaction::ExitCancel)
    .into();

    let unit_spacing = 15;

    let button_row = Row::new()
    .push(yes_button.map(Message::Interaction))
    .push(Space::new(Length::Units(unit_spacing), Length::Units(0)))
    .push(cancel_button.map(Message::Interaction));

    let colum = Column::new()
        .push(title_row)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(button_row)
        .align_items(Alignment::Center);

    Container::new(colum)
        .center_y()
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
}
