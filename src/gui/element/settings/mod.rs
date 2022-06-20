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
    strfmt::strfmt,
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

#[derive(Debug, Clone)]
pub enum Mode {
    Wallet,
    Node,
    General,
}

#[derive(Debug, Clone)]
pub struct View {
    pub state: StateContainer,
}

impl Default for View {
    fn default() -> Self {
        Self {
            state: Default::default(),
        }
    }
}

impl View {
    /// Create new with state
    pub fn new(state: StateContainer) -> Self {
        Self { state }
    }
}

impl MessageHandlingView for View {
    fn handle_message(&mut self, message: &Message) -> crate::Result<iced::Command<Message>> {
        match message {
            Message::Interaction(Interaction::ModeSelectedSettings(mode)) => {
                log::debug!("Interaction::ModeSelectedSettings({:?})", mode);
                // Set Mode
                self.state.mode = mode.clone();
            }
            _ => {},
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
        .on_press(Interaction::ModeSelectedSettings(Mode::Wallet));

        let mut node_button: Button<Interaction> = Button::new(
            &mut self.state.node_btn,
            Text::new(localized_string("node")).size(DEFAULT_FONT_SIZE),
        )
        .on_press(Interaction::ModeSelectedSettings(Mode::Node));

        let mut general_button: Button<Interaction> = Button::new(
            &mut self.state.general_btn,
            Text::new(localized_string("general")).size(DEFAULT_FONT_SIZE),
        )
        .on_press(Interaction::ModeSelectedSettings(Mode::General));

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
