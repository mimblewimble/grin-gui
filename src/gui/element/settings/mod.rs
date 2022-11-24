use iced::alignment::Horizontal;

pub mod general;
pub mod node;
pub mod wallet;

use {
    super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{GrinGui, Interaction, Message},
    crate::localization::localized_string,
    grin_gui_core::theme::{
        Button, Column, Container, Element, PickList, Row, Scrollable, Text, TextInput,
    },
    grin_gui_core::{config::Config, theme::ColorPalette},
    iced::widget::{button, Space},
    iced::{Alignment, Length},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone)]
pub struct StateContainer {
    pub mode: Mode,
    // wallet_btn: button::State,
    // node_btn: button::State,
    // general_btn: button::State,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            mode: Mode::Wallet,
            // wallet_btn: Default::default(),
            // node_btn: Default::default(),
            // general_btn: Default::default(),
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
            grin_gui.settings_state.mode = mode;
        }
    }
}

pub fn data_container<'a>(
    state: &'a StateContainer,
    config: &'a Config,
    wallet_settings_state: &'a wallet::StateContainer,
    node_settings_state: &'a node::StateContainer,
    general_settings_state: &'a general::StateContainer,
    color_palette: ColorPalette,
) -> Container<'a, Message> {
    let title_string = match state.mode {
        Mode::Wallet => localized_string("settings-wallet"),
        Mode::Node => localized_string("settings-node"),
        Mode::General => localized_string("settings-general"),
    };

    // Submenu title to appear of left side of panel
    let general_settings_title = Text::new(title_string).size(DEFAULT_HEADER_FONT_SIZE);
    let general_settings_title_container = Container::new(general_settings_title)
        .style(grin_gui_core::theme::container::Container::BrightBackground)
        .padding(iced::Padding::from([
            0, // top
            0, // right
            0, // bottom
            5, // left
        ]));

    let mut wallet_button: Button<Interaction> = Button::new(
        // &mut state.wallet_btn,
        Text::new(localized_string("wallet")).size(DEFAULT_FONT_SIZE),
    )
    .on_press(Interaction::SettingsViewInteraction(
        LocalViewInteraction::SelectMode(Mode::Wallet),
    ));

    let mut node_button: Button<Interaction> = Button::new(
        // &mut state.node_btn,
        Text::new(localized_string("node")).size(DEFAULT_FONT_SIZE),
    )
    .on_press(Interaction::SettingsViewInteraction(
        LocalViewInteraction::SelectMode(Mode::Node),
    ));

    let mut general_button: Button<Interaction> = Button::new(
        // &mut state.general_btn,
        Text::new(localized_string("general")).size(DEFAULT_FONT_SIZE),
    )
    .on_press(Interaction::SettingsViewInteraction(
        LocalViewInteraction::SelectMode(Mode::General),
    ));

    match state.mode {
        Mode::Wallet => {
            wallet_button = wallet_button.style(grin_gui_core::theme::button::Button::Selected);
            node_button =
                node_button.style(grin_gui_core::theme::button::Button::Primary);
            general_button =
                general_button.style(grin_gui_core::theme::button::Button::Primary);
        }
        Mode::Node => {
            wallet_button =
                wallet_button.style(grin_gui_core::theme::button::Button::Primary);
            node_button = node_button.style(grin_gui_core::theme::button::Button::Selected);
            general_button =
                general_button.style(grin_gui_core::theme::button::Button::Primary);
        }
        Mode::General => {
            wallet_button =
                wallet_button.style(grin_gui_core::theme::button::Button::Primary);
            node_button =
                node_button.style(grin_gui_core::theme::button::Button::Primary);
            general_button = general_button.style(grin_gui_core::theme::button::Button::Selected);
        }
    }

    let wallet_button: Element<Interaction> = wallet_button.into();
    let node_button: Element<Interaction> = node_button.into();
    let general_button: Element<Interaction> = general_button.into();

    let segmented_mode_row = Row::new()
        .push(wallet_button.map(Message::Interaction))
        .push(node_button.map(Message::Interaction))
        .push(general_button.map(Message::Interaction))
        .spacing(1);

    let segmented_mode_container = Container::new(segmented_mode_row).padding(1);

    let segmented_mode_control_container =
        Container::new(segmented_mode_container).padding(1).style(
            grin_gui_core::theme::container::Container::Segmented,
        );

    let header_row = Row::new()
        .push(general_settings_title_container)
        .push(Space::with_width(Length::Fill))
        .push(segmented_mode_control_container)
        .align_items(Alignment::Center);

    let header_container = Container::new(header_row);

    // Wrapper for submenu + actual content
    let mut wrapper_column =
        Column::with_children(vec![header_container.into()]).height(Length::Fill);
    // Submenu Area + actual content
    match state.mode {
        Mode::Wallet => {
            wrapper_column =
                wrapper_column.push(wallet::data_container(wallet_settings_state, color_palette))
        }
        Mode::Node => {
            wrapper_column =
                wrapper_column.push(node::data_container(node_settings_state, color_palette))
        }
        Mode::General => {
            wrapper_column = wrapper_column.push(general::data_container(
                general_settings_state,
                config,
                color_palette,
            ))
        }
    }

    Container::new(wrapper_column)
        .style(grin_gui_core::theme::container::Container::NormalBackground)
        .padding(iced::Padding::from([
            DEFAULT_PADDING, // top
            DEFAULT_PADDING, // right
            DEFAULT_PADDING, // bottom
            DEFAULT_PADDING, // left
        ]))
}
