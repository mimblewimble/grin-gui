use {
    super::DEFAULT_FONT_SIZE,
    crate::gui::{GrinGui, Message},
    crate::localization::localized_string,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::theme::{Button, Column, Container, PickList, Row, Scrollable, Text, TextInput},
    iced::widget::{button, pick_list, scrollable, text_input, Checkbox, Space},
    iced::Length,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone)]
pub struct StateContainer {
    pub mode: Mode,
    // scrollable_state: scrollable::State,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            mode: Mode::Wallet,
            // scrollable_state: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LocalViewInteraction {
    SelectMode(Mode),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Mode {
    Wallet,
    Node,
    General,
}

pub fn handle_message(grin_gui: &mut GrinGui, message: LocalViewInteraction) {
    match message {
        LocalViewInteraction::SelectMode(mode) => {
            log::debug!("Interaction::ModeSelectedSettings({:?})", mode);
            // Set Mode
            grin_gui.node_settings_state.mode = mode
        }
    }
}

pub fn data_container<'a>(
    state: &'a StateContainer,
) -> Container<'a, Message> {
    let language_container = {
        let title = Container::new(Text::new(localized_string("todo")).size(DEFAULT_FONT_SIZE))
            .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        Column::new()
            .push(title)
            .push(Space::new(Length::Units(0), Length::Units(5)))
    };

    // Colum wrapping all the settings content.
    let scrollable = Scrollable::new(language_container)
        .height(Length::Fill)
        .style(grin_gui_core::theme::ScrollableStyle::Primary);

    let col = Column::new()
        .push(Space::new(Length::Units(0), Length::Units(10)))
        .push(scrollable)
        .push(Space::new(Length::Units(0), Length::Units(20)));
    let row = Row::new()
        .push(Space::new(Length::Units(5), Length::Units(0)))
        .push(col);

    // Returns the final container.
    Container::new(row)
        .width(Length::Fill)
        .height(Length::Shrink)
        .style(grin_gui_core::theme::ContainerStyle::NormalBackground)
}
