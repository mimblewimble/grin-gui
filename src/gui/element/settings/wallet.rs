use {
    super::{DEFAULT_FONT_SIZE},
    crate::gui::{GrinGui, Message},
    crate::localization::localized_string,
    grin_gui_core::{theme::ColorPalette},
    grin_gui_core::theme::{Button, Column, Container, PickList, Row, Scrollable, Text, TextInput},
    iced::Length,
    iced::widget::{
        button, pick_list, scrollable, text_input, Checkbox, Space,
    },
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

pub fn handle_message(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) {
    match message {
        LocalViewInteraction::SelectMode(mode) => {
            log::debug!("Interaction::ModeSelectedSettings({:?})", mode);
            // Set Mode
            grin_gui.wallet_settings_state.mode = mode;
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
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(5.0)))
    };

    let scrollable = Scrollable::new(language_container)
    .height(Length::Fill)
    .style(grin_gui_core::theme::ScrollableStyle::Primary);

    let col = Column::new()
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(10.0)))
        .push(scrollable)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(20.0)));
    let row = Row::new()
        .push(Space::new(Length::Fixed(5.0), Length::Fixed(0.0)))
        .push(col);

    // Returns the final container.
    Container::new(row)
        .width(Length::Fill)
        .height(Length::Shrink)
        .style(grin_gui_core::theme::ContainerStyle::NormalBackground)
}
