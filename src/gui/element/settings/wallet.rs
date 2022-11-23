use {
    super::{DEFAULT_FONT_SIZE},
    crate::gui::{style, GrinGui, Message},
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
    color_palette: ColorPalette,
) -> Container<'a, Message> {
   
    let language_container = {
        let title = Container::new(Text::new(localized_string("language")).size(DEFAULT_FONT_SIZE))
            .style(grin_gui_core::theme::container::Container::NormalBackground(color_palette));
        /*let pick_list: Element<_> = PickList::new(
            localization_picklist_state,
            &Language::ALL[..],
            Some(config.language),
            Interaction::PickLocalizationLanguage,
        )
        .text_size(14)
        .width(Length::Units(120))
        .style(style::PickList(color_palette))
        .into();
        let container = Container::new(pick_list.map(Message::Interaction))
            .center_y()
            .width(Length::Units(120))
            .style(grin_gui_core::theme::container::Container::NormalBackground(color_palette));*/

        Column::new()
            .push(title)
            .push(Space::new(Length::Units(0), Length::Units(5)))
            //.push(container)
    };

    let scrollable = Scrollable::new(language_container)
    .height(Length::Fill)
    .style(grin_gui_core::theme::scrollable::ScrollableStyles::Primary(color_palette));

    // scrollable = scrollable
    //     .push(language_container);

    // Colum wrapping all the settings content.
    //scrollable = scrollable.height(Length::Fill);

    let col = Column::new()
        .push(Space::new(Length::Units(0), Length::Units(10)))
        .push(scrollable)
        .push(Space::new(Length::Units(0), Length::Units(20)));
    let row = Row::new()
        .push(Space::new(Length::Units(20), Length::Units(0)))
        .push(col);

    // Returns the final container.
    Container::new(row)
        .center_x()
        .width(Length::Fill)
        .height(Length::Shrink)
        .style(grin_gui_core::theme::container::Container::NormalBackground(color_palette))
}
