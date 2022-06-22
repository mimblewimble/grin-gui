pub mod wallet;

use {
    super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, Interaction, Message},
    crate::localization::localized_string,
    ajour_core::{theme::ColorPalette, utility::Release},
    iced::{
        button, Alignment, Button, Command, Container, Column, Element, Length, Row,
        Space, Text,
    },
    serde::{Deserialize, Serialize},
    serde_json,
    std::sync::{Arc, RwLock},
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

pub fn handle_message(state: &mut StateContainer, message: LocalViewInteraction) -> crate::Result<Command<Message>> {
    match message {
        LocalViewInteraction::SelectMode(mode) => {
            log::debug!("Interaction::ModeSelectedSettings({:?})", mode);
            // Set Mode
            state.mode = mode
        }
    }
    Ok(Command::none())
}

pub fn data_container<'a>(
    state: &'a mut StateContainer,
    wallet_settings_state: &'a mut wallet::StateContainer,
    color_palette: ColorPalette,
) -> Container<'a, Message> {

    let mut selection_row = Row::new()
        .height(Length::Units(40))
        .push(Space::new(Length::Units(DEFAULT_PADDING), Length::Units(1)));

    let mut wallet_button: Button<Interaction> = Button::new(
        &mut state.wallet_btn,
        Text::new(localized_string("wallet")).size(DEFAULT_FONT_SIZE),
    )
    .on_press(Interaction::SettingsViewInteraction(LocalViewInteraction::SelectMode(Mode::Wallet)));

    let mut node_button: Button<Interaction> = Button::new(
        &mut state.node_btn,
        Text::new(localized_string("node")).size(DEFAULT_FONT_SIZE),
    )
    .on_press(Interaction::SettingsViewInteraction(LocalViewInteraction::SelectMode(Mode::Node)));

    let mut general_button: Button<Interaction> = Button::new(
        &mut state.general_btn,
        Text::new(localized_string("general")).size(DEFAULT_FONT_SIZE),
    )
    .on_press(Interaction::SettingsViewInteraction(LocalViewInteraction::SelectMode(Mode::General)));

    match state.mode {
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
            Length::Units(DEFAULT_PADDING + 5),
            Length::Units(0),
        ))
        .align_items(Alignment::Center);

    // Wrapper for submenu + actual content
    let mut wrapper_column = Column::new().height(Length::Fill);
    wrapper_column = wrapper_column.push(selection_row);
    // Submenu Area + actual content
    match state.mode {
        Mode::Wallet => {
            wrapper_column = wrapper_column.push(wallet::data_container(wallet_settings_state, color_palette))
        },
        _ => {}
    }

    Container::new(wrapper_column).style(style::BrightForegroundContainer(color_palette))
}
