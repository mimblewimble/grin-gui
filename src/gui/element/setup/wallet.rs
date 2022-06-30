use iced_native::Widget;

use {
    super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::{config::Config, wallet::WalletInterface},
    iced::{
        alignment, button, text_input, Alignment, Button, Checkbox, Column, Command, Container,
        Element, Length, Row, Space, Text, TextInput,
    },
};

pub struct StateContainer {
    pub password_state: PasswordState,
    pub back_button_state: button::State,
    pub submit_button_state: button::State,
    pub restore_from_seed: bool,
    pub show_advanced_options: bool,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            password_state: Default::default(),
            back_button_state: Default::default(),
            submit_button_state: Default::default(),
            show_advanced_options: false,
            restore_from_seed: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PasswordState {
    pub input_state: text_input::State,
    pub input_value: String,
    pub repeat_input_state: text_input::State,
    pub repeat_input_value: String,
}

impl Default for PasswordState {
    fn default() -> Self {
        PasswordState {
            input_state: Default::default(),
            input_value: Default::default(),
            repeat_input_state: Default::default(),
            repeat_input_value: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    Back,
    //TODO: ZeroingString these
    PasswordInput(String),
    PasswordRepeatInput(String),
    ToggleRestoreFromSeed(bool),
    ToggleAdvancedOptions(bool),
    Submit,
}

fn asterisk(input: &str) -> String {
    input.chars().map(|_| '*').collect()
}

pub fn handle_message(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let state = &mut grin_gui.setup_wallet_state;
    let setup_state = &mut grin_gui.setup_state;
    match message {
        LocalViewInteraction::Back => {
            setup_state.mode = super::Mode::Init;
        }
        LocalViewInteraction::PasswordInput(password) => {
            state.password_state.input_value = password;
        }
        LocalViewInteraction::PasswordRepeatInput(repeat_password) => {
            state.password_state.repeat_input_value = repeat_password;
        }
        LocalViewInteraction::ToggleRestoreFromSeed(_) => {
            state.restore_from_seed = !state.restore_from_seed
        }
        LocalViewInteraction::ToggleAdvancedOptions(_) => {
            state.show_advanced_options = !state.show_advanced_options
        }
        LocalViewInteraction::Submit => {
            //
        }
     }
    Ok(Command::none())
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    state: &'a mut StateContainer,
) -> Container<'a, Message> {
    // Title row and back button
    let back_button_label_container =
        Container::new(Text::new(localized_string("back")).size(DEFAULT_FONT_SIZE))
            .height(Length::Units(20))
            .align_y(alignment::Vertical::Bottom)
            .align_x(alignment::Horizontal::Center);

    let back_button: Element<Interaction> =
        Button::new(&mut state.back_button_state, back_button_label_container)
            .style(style::NormalTextButton(color_palette))
            .on_press(Interaction::SetupWalletViewInteraction(
                LocalViewInteraction::Back,
            ))
            .into();

    let title = Text::new(localized_string("setup-grin-wallet-title"))
        .size(DEFAULT_HEADER_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);
    let title_container =
        Container::new(title).style(style::BrightBackgroundContainer(color_palette));

    let title_row = Row::new()
        .push(title_container)
        .push(Space::new(Length::Units(100), Length::Units(0)))
        .push(back_button.map(Message::Interaction))
        .align_items(Alignment::Center)
        .spacing(20);

    let password_column = {
        let password_input = TextInput::new(
            &mut state.password_state.input_state,
            &localized_string("password")[..],
            &state.password_state.input_value,
            |s| Interaction::SetupWalletViewInteraction(LocalViewInteraction::PasswordInput(s)),
        )
        .size(DEFAULT_FONT_SIZE)
        .padding(6)
        .width(Length::Units(200))
        .style(style::AddonsQueryInput(color_palette))
        .password();

        let password_input: Element<Interaction> = password_input.into();

        let repeat_password_input = TextInput::new(
            &mut state.password_state.repeat_input_state,
            &localized_string("password-repeat")[..],
            &state.password_state.repeat_input_value,
            |s| {
                Interaction::SetupWalletViewInteraction(LocalViewInteraction::PasswordRepeatInput(
                    s,
                ))
            },
        )
        .size(DEFAULT_FONT_SIZE)
        .padding(6)
        .width(Length::Units(200))
        .style(style::AddonsQueryInput(color_palette))
        .password();

        let repeat_password_input: Element<Interaction> = repeat_password_input.into();

        let password_input_col = Column::new()
            .push(password_input.map(Message::Interaction))
            .push(repeat_password_input.map(Message::Interaction))
            .spacing(DEFAULT_PADDING)
            .align_items(Alignment::Center);

        Column::new().push(password_input_col)
    };

    let description = Text::new(localized_string("setup-grin-wallet-enter-password"))
        .size(DEFAULT_FONT_SIZE)
        //.width(Length::Fill)
        .horizontal_alignment(alignment::Horizontal::Center);
    let description_container = Container::new(description)
        //.width(Length::Fill)
        .style(style::NormalBackgroundContainer(color_palette));

    let restore_from_seed_column = {
        let checkbox = Checkbox::new(
            state.restore_from_seed,
            localized_string("restore-from-seed"),
            |b| Interaction::SetupWalletViewInteraction(LocalViewInteraction::ToggleRestoreFromSeed(b)),
        )
        .style(style::DefaultCheckbox(color_palette))
        .text_size(DEFAULT_FONT_SIZE)
        .spacing(10);

        let checkbox: Element<Interaction> = checkbox.into();

        let checkbox_container = Container::new(checkbox.map(Message::Interaction))
            .style(style::NormalBackgroundContainer(color_palette));
        Column::new().push(checkbox_container)
    };

    let show_advanced_options_column = {
        let checkbox = Checkbox::new(
            state.show_advanced_options,
            localized_string("show-advanced-options"),
            |b| Interaction::SetupWalletViewInteraction(LocalViewInteraction::ToggleAdvancedOptions(b)),
        )
        .style(style::DefaultCheckbox(color_palette))
        .text_size(DEFAULT_FONT_SIZE)
        .spacing(10);

        let checkbox: Element<Interaction> = checkbox.into();

        let checkbox_container = Container::new(checkbox.map(Message::Interaction))
            .style(style::NormalBackgroundContainer(color_palette));
        Column::new().push(checkbox_container)
    };

    let submit_button_label_container = Container::new(
        Text::new(localized_string("setup-grin-create-wallet")).size(DEFAULT_FONT_SIZE),
    )
    .center_x()
    .align_x(alignment::Horizontal::Center);

    let submit_button: Element<Interaction> = Button::new(
        &mut state.submit_button_state,
        submit_button_label_container,
    )
    .style(style::DefaultBoxedButton(color_palette))
    .on_press(Interaction::SetupWalletViewInteraction(
        LocalViewInteraction::Submit
    ))
    .into();

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
        .push(restore_from_seed_column)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(show_advanced_options_column)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(submit_button.map(Message::Interaction))
        .align_items(Alignment::Start);

    Container::new(colum)
        .center_y()
        .center_x()
        .width(Length::Fill)
}
