use {
    super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, Interaction, Message, MessageHandlingView},
    crate::localization::localized_string,
    ajour_core::{theme::ColorPalette, utility::Release},
    iced::{
        button, scrollable, Alignment, Button, Column, Command, Container, Element, Length, Row,
        Scrollable, Space, Text,
    },
    std::collections::HashMap,
    std::{fmt::Display},
    strfmt::strfmt,
    serde_json,
    serde::{Serialize, Deserialize},
};

#[derive(Debug, Clone)]
pub struct StateContainer {
    pub mode: Mode,
    wallet_btn: button::State,
    node_btn: button::State,
    general_btn: button::State,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            mode: Mode::Wallet,
            wallet_btn: Default::default(),
            node_btn: Default::default(),
            general_btn: Default::default(),
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

#[derive(Debug, Clone)]
pub struct View {
    pub id: String,
    pub state: StateContainer,
}

impl Default for View {
    fn default() -> Self {
        Self {
            id: Default::default(),
            state: Default::default(),
        }
    }
}

impl MessageHandlingView for View {

    fn set_id(&mut self, new_id: &str) {
        self.id = new_id.to_string()
    }

    fn get_id(&self) -> &str {
        &self.id
    }

    fn handle_message(&mut self, message: &Message) -> crate::Result<iced::Command<Message>> {
        if let Message::Interaction(Interaction::ViewInteraction(stringified)) = message {
            let local_interaction: LocalViewInteraction = serde_json::from_str(stringified).unwrap();
            match local_interaction {
                LocalViewInteraction::SelectMode(mode) => {
                    log::debug!("Interaction::ModeSelectedSettings({:?})", mode);
                    // Set Mode
                    self.state.mode = mode
                }
            }
        }
        Ok(Command::none())
    }

    fn data_container<'a>(&'a mut self, color_palette: ColorPalette) -> Container<'a, Message> {
        let mut selection_row = Row::new()
            .height(Length::Units(40))
            .push(Space::new(Length::Units(DEFAULT_PADDING), Length::Units(1)));

        let mut wallet_button: Button<Interaction> = Button::new(
            &mut self.state.wallet_btn,
            Text::new(localized_string("wallet")).size(DEFAULT_FONT_SIZE),
        )
        .on_press(Interaction::ViewInteraction(
            serde_json::to_string(&LocalViewInteraction::SelectMode(Mode::Wallet)).unwrap()
        ));

        let mut node_button: Button<Interaction> = Button::new(
            &mut self.state.node_btn,
            Text::new(localized_string("node")).size(DEFAULT_FONT_SIZE),
        )
        .on_press(Interaction::ViewInteraction(
            serde_json::to_string(&LocalViewInteraction::SelectMode(Mode::Node)).unwrap()
        ));

        let mut general_button: Button<Interaction> = Button::new(
            &mut self.state.general_btn,
            Text::new(localized_string("general")).size(DEFAULT_FONT_SIZE),
        )
        .on_press(Interaction::ViewInteraction(
            serde_json::to_string(&LocalViewInteraction::SelectMode(Mode::General)).unwrap()
        ));

        match self.state.mode {
            Mode::Wallet => {
                wallet_button = wallet_button.style(style::SelectedDefaultButton(color_palette));
                node_button = node_button.style(style::DefaultButton(color_palette));
                general_button = general_button.style(style::DefaultButton(color_palette));
            }
            Mode::Node => {
                wallet_button = wallet_button.style(style::DefaultButton(color_palette));
                node_button = node_button.style(style::SelectedDefaultButton(color_palette));
                general_button = general_button.style(style::DefaultButton(color_palette));
            }
            Mode::General => {
                wallet_button = wallet_button.style(style::DefaultButton(color_palette));
                node_button = node_button.style(style::DefaultButton(color_palette));
                general_button = general_button.style(style::SelectedDefaultButton(color_palette));
            }
        }

        let wallet_button: Element<Interaction> = wallet_button.into();
        let node_button: Element<Interaction> = node_button.into();
        let general_button: Element<Interaction> = general_button.into();

        let segmented_mode_row = Row::new()
            .push(wallet_button.map(Message::Interaction))
            .push(node_button.map(Message::Interaction))
            .push(general_button.map(Message::Interaction))
            .spacing(0);

        let segmented_mode_container = Container::new(segmented_mode_row)
            .padding(1)
            .style(style::SegmentedContainer(color_palette));

        let segmented_mode_control_container = Container::new(segmented_mode_container)
            .padding(1)
            .style(style::SegmentedContainer(color_palette));

        selection_row = selection_row
            .push(Space::new(Length::Fill, Length::Units(0)))
            .push(segmented_mode_control_container)
            .push(Space::new(
                Length::Units(DEFAULT_PADDING + 4),
                Length::Units(0),
            ))
            .align_items(Alignment::Center);
        Container::new(selection_row).style(style::BrightForegroundContainer(color_palette))
    }

}
