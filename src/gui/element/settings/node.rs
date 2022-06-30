use {
    super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, GrinGui, Message},
    crate::localization::localized_string,
    grin_gui_core::{config::Language, theme::ColorPalette, utility::Release},
    iced::{
        button, pick_list, scrollable, Alignment, Button, Column, Command, Container, Element,
        Length, PickList, Row, Scrollable, Space, Text,
    },
    serde::{Deserialize, Serialize},
    std::sync::{Arc, RwLock},
};

#[derive(Debug, Clone)]
pub struct StateContainer {
    pub mode: Mode,
    scrollable_state: scrollable::State,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            mode: Mode::Wallet,
            scrollable_state: Default::default(),
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
    state: &'a mut StateContainer,
    color_palette: ColorPalette,
) -> Container<'a, Message> {
    let mut scrollable = Scrollable::new(&mut state.scrollable_state)
        .spacing(1)
        .height(Length::FillPortion(1))
        .style(style::Scrollable(color_palette));

    let language_container = {
        let title = Container::new(Text::new(localized_string("language")).size(DEFAULT_FONT_SIZE))
            .style(style::NormalBackgroundContainer(color_palette));
        /*let pick_list: Element<_> = PickList::new(
            &mut state.localization_picklist_state,
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
            .style(style::NormalForegroundContainer(color_palette));*/

        Column::new()
            .push(title)
            .push(Space::new(Length::Units(0), Length::Units(5)))
        //.push(container)
    };

    scrollable = scrollable.push(language_container);

    // Colum wrapping all the settings content.
    scrollable = scrollable.height(Length::Fill).width(Length::Fill);

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
        .style(style::NormalBackgroundContainer(color_palette))
}
