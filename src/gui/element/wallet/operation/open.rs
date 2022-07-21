use crate::{gui::element::settings::wallet, log_error};
use iced::button::StyleSheet;
use iced_native::Widget;
use std::path::PathBuf;

use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    anyhow::Context,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::{fs::PersistentData, wallet::WalletInterface},
    iced::{
        alignment, button, text_input, Alignment, Button, Checkbox, Column, Command, Container,
        Element, Length, Row, Space, Text, TextInput,
    },
    std::sync::{Arc, RwLock},
};

pub struct StateContainer {
    pub password_state: PasswordState,
    pub submit_button_state: button::State,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            password_state: Default::default(),
            submit_button_state: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PasswordState {
    pub input_state: text_input::State,
    pub input_value: String,
}

impl Default for PasswordState {
    fn default() -> Self {
        PasswordState {
            input_state: Default::default(),
            input_value: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    //TODO: ZeroingString these
    PasswordInput(String),
    PasswordInputEnterPressed,
    Submit,
}

pub fn handle_message<'a>(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let state = &mut grin_gui.wallet_state.setup_state.setup_wallet_state;
    match message {
        LocalViewInteraction::PasswordInput(password) => {
            state.password_state.input_value = password;
        }
        LocalViewInteraction::PasswordInputEnterPressed => {
            state.password_state.input_state.unfocus();
            state.password_state.repeat_input_state.focus();
        }
        LocalViewInteraction::Submit => {

        }
    }
    Ok(Command::none())
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    state: &'a mut StateContainer,
) -> Container<'a, Message> {
    // Title row
    let title = Text::new(localized_string("setup-grin-first-time"))
        .size(DEFAULT_HEADER_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);

    let title_container = Container::new(title)
        .style(style::BrightBackgroundContainer(color_palette));

    let title_row = Row::new()
        .push(title_container)
        .align_items(Alignment::Center)
        .padding(6)
        .spacing(20);

    let password_column = {
        let password_input = TextInput::new(
            &mut state.password_state.input_state,
            &localized_string("password")[..],
            &state.password_state.input_value,
            |s| Interaction::WalletOperationOpenViewInteraction(LocalViewInteraction::PasswordInput(s)),
        )
        .on_submit(Interaction::WalletOperationOpenViewInteraction(
            LocalViewInteraction::PasswordInputEnterPressed,
        ))
        .size(DEFAULT_FONT_SIZE)
        .padding(6)
        .width(Length::Units(200))
        .style(style::AddonsQueryInput(color_palette))
        .password();

        let password_input: Element<Interaction> = password_input.into();

        let mut password_input_col = Column::new()
            .push(password_input.map(Message::Interaction))
            .spacing(DEFAULT_PADDING)
            .align_items(Alignment::Start);

        Column::new().push(password_input_col)
    };

    let description = Text::new(localized_string("setup-grin-wallet-enter-password"))
        .size(DEFAULT_FONT_SIZE)
        //.width(Length::Fill)
        .horizontal_alignment(alignment::Horizontal::Center);

    let description_container = Container::new(description)
        //.width(Length::Fill)
        .style(style::NormalBackgroundContainer(color_palette));

    let submit_button_label_container = Container::new(
        Text::new(localized_string("setup-grin-create-wallet")).size(DEFAULT_FONT_SIZE),
    )
    .center_x()
    .align_x(alignment::Horizontal::Center);

    let mut submit_button = Button::new(
        &mut state.submit_button_state,
        submit_button_label_container,
    )
    .style(style::DefaultBoxedButton(color_palette));

    submit_button = submit_button.on_press(Interaction::WalletOperationOpenViewInteraction(
        LocalViewInteraction::Submit
    ));

    let submit_button: Element<Interaction> = submit_button.into();

    let unit_spacing = 15;

    let colum = Column::new()
        .push(title_row)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(description_container)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(password_column)
        .push(Space::new(
            Length::Units(0),
            Length::Units(unit_spacing + 10),
        ))
        .push(submit_button.map(Message::Interaction))
        .align_items(Alignment::Start);

    Container::new(colum)
        .center_y()
        .center_x()
        .width(Length::Fill)
}
