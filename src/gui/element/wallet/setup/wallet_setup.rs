use crate::log_error;
//use futures::future::OrElse;
//use iced::button::StyleSheet;
//use iced_native::Widget;
use native_dialog::FileDialog;
use std::path::PathBuf;

use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    anyhow::Context,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::{
        config::Wallet,
        fs::PersistentData,
        node::ChainTypes::{self, Mainnet, Testnet},
        wallet::create_grin_wallet_path,
        wallet::WalletInterface,
    },
    iced::{
        alignment, button, text_input, Alignment, Button, Checkbox, Column, Command, Container,
        Element, Length, Row, Space, Text, TextInput,
    },
    std::sync::{Arc, RwLock},
};

pub struct StateContainer {
    pub password_state: PasswordState,
    pub back_button_state: button::State,
    pub submit_button_state: button::State,
    pub restore_from_seed: bool,
    pub show_advanced_options: bool,
    pub is_testnet: bool,
    pub advanced_options_state: AdvancedOptionsState,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            password_state: Default::default(),
            back_button_state: Default::default(),
            submit_button_state: Default::default(),
            show_advanced_options: false,
            is_testnet: false,
            restore_from_seed: false,
            advanced_options_state: Default::default(),
        }
    }
}

pub struct AdvancedOptionsState {
    display_name_input_state: text_input::State,
    folder_select_button_state: button::State,
    pub display_name_value: String,
    pub top_level_directory: PathBuf,
}

impl Default for AdvancedOptionsState {
    fn default() -> Self {
        Self {
            display_name_input_state: Default::default(),
            display_name_value: Default::default(),
            folder_select_button_state: Default::default(),
            top_level_directory: Default::default(),
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
    PasswordInputEnterPressed,
    PasswordRepeatInput(String),
    PasswordRepeatInputEnterPressed,
    ToggleRestoreFromSeed(bool),
    ToggleAdvancedOptions(bool),
    ToggleIsTestnet(bool),
    DisplayName(String),
    CreateWallet,
    WalletCreatedOk((String, String, String, ChainTypes)),
    WalletCreateError(Arc<RwLock<Option<anyhow::Error>>>),
    ShowFolderPicker,
}

pub fn handle_message<'a>(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let state = &mut grin_gui.wallet_state.setup_state.setup_wallet_state;
    match message {
        LocalViewInteraction::Back => {
            // reset user input values
            grin_gui.wallet_state.setup_state.setup_wallet_state = Default::default();
            grin_gui.wallet_state.setup_state.mode = super::Mode::Init;
        }
        LocalViewInteraction::PasswordInput(password) => {
            state.password_state.input_value = password;
        }
        LocalViewInteraction::PasswordInputEnterPressed => {
            state.password_state.input_state.unfocus();
            state.password_state.repeat_input_state.focus();
        }
        LocalViewInteraction::PasswordRepeatInput(repeat_password) => {
            state.password_state.repeat_input_value = repeat_password;
        }
        LocalViewInteraction::PasswordRepeatInputEnterPressed => {
            state.password_state.repeat_input_state.unfocus();
        }
        LocalViewInteraction::ToggleRestoreFromSeed(_) => {
            state.restore_from_seed = !state.restore_from_seed
        }
        LocalViewInteraction::ToggleAdvancedOptions(_) => {
            state.show_advanced_options = !state.show_advanced_options
        }
        LocalViewInteraction::ToggleIsTestnet(_) => {
            state.is_testnet = !state.is_testnet;
            let current_tld = state.advanced_options_state.top_level_directory.clone();
            let directory = current_tld.file_name().unwrap().to_str().unwrap();

            if state.is_testnet {
                let default_path = create_grin_wallet_path(&Mainnet, directory);
                // Only change if nobody's modified the default path
                if default_path == current_tld {
                    state.advanced_options_state.top_level_directory =
                        create_grin_wallet_path(&Testnet, directory);
                }
            } else {
                let default_path = create_grin_wallet_path(&Testnet, directory);
                // Only change if nobody's modified the default path
                if default_path == current_tld {
                    state.advanced_options_state.top_level_directory =
                        create_grin_wallet_path(&Mainnet, directory);
                }
            }
        }
        LocalViewInteraction::DisplayName(display_name_value) => {
            state.advanced_options_state.display_name_value = display_name_value;
        }
        LocalViewInteraction::ShowFolderPicker => {
            match FileDialog::new().show_open_single_dir() {
                Ok(path) => match path {
                    Some(d) => {
                        state.advanced_options_state.top_level_directory = d;
                    }
                    None => {}
                },
                Err(e) => {
                    log::debug!(
                        "wallet_setup.rs::LocalViewInteraction::ShowFolderPicker {}",
                        e
                    );
                }
            };
        }
        LocalViewInteraction::CreateWallet => {
            grin_gui.error.take();

            log::debug!(
                "setup::wallet::LocalViewInteraction::CreateWallet {}",
                state.advanced_options_state.display_name_value
            );

            let password = state.password_state.input_value.clone();
            let w = grin_gui.wallet_interface.clone();
            let chain_type = if state.is_testnet { Testnet } else { Mainnet };

            let fut = move || {
                WalletInterface::init(
                    w,
                    password.clone(),
                    state.advanced_options_state.top_level_directory.clone(),
                    state.advanced_options_state.display_name_value.clone(),
                    chain_type,
                )
            };

            return Ok(Command::perform(fut(), |r| {
                match r.context("Failed to Create Wallet") {
                    Ok(ret) => Message::Interaction(Interaction::WalletSetupWalletViewInteraction(
                        LocalViewInteraction::WalletCreatedOk(ret),
                    )),
                    Err(e) => Message::Interaction(Interaction::WalletSetupWalletViewInteraction(
                        LocalViewInteraction::WalletCreateError(Arc::new(RwLock::new(Some(e)))),
                    )),
                }
            }));
        }
        LocalViewInteraction::WalletCreatedOk((tld, mnemonic, display_name, chain_type)) => {
            let tld = Some(PathBuf::from(&tld));
            let saved_wallet = Wallet::new(tld, display_name, chain_type);

            let index = grin_gui.config.add_wallet(saved_wallet);
            grin_gui.config.current_wallet_index = Some(index);
            grin_gui.wallet_state.clear_config_missing();
            grin_gui
                .wallet_state
                .setup_state
                .setup_wallet_success_state
                .recovery_phrase = mnemonic;

            grin_gui.wallet_state.setup_state.mode =
                crate::gui::element::wallet::setup::Mode::WalletCreateSuccess;

            // reset user input values
            grin_gui.wallet_state.setup_state.setup_wallet_state = Default::default();

            let _ = grin_gui.config.save();
        }
        LocalViewInteraction::WalletCreateError(err) => {
            grin_gui.error = err.write().unwrap().take();
            if let Some(e) = grin_gui.error.as_ref() {
                log_error(e);
            }
        }
    }

    Ok(Command::none())
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    state: &'a mut StateContainer,
) -> Container<'a, Message> {
    let check_password = || {
        state.password_state.input_value == state.password_state.repeat_input_value
            && !state.password_state.input_value.is_empty()
    };

    let disp_password_status = || {
        !state.password_state.input_value.is_empty()
            && !state.password_state.repeat_input_value.is_empty()
    };

    // Title row and back button
    let back_button_label_container =
        Container::new(Text::new(localized_string("back")).size(DEFAULT_FONT_SIZE))
            .height(Length::Units(20))
            .align_y(alignment::Vertical::Bottom)
            .align_x(alignment::Horizontal::Center);

    let back_button: Element<Interaction> =
        Button::new(&mut state.back_button_state, back_button_label_container)
            .style(style::NormalTextButton(color_palette))
            .on_press(Interaction::WalletSetupWalletViewInteraction(
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
            |s| {
                Interaction::WalletSetupWalletViewInteraction(LocalViewInteraction::PasswordInput(
                    s,
                ))
            },
        )
        .on_submit(Interaction::WalletSetupWalletViewInteraction(
            LocalViewInteraction::PasswordInputEnterPressed,
        ))
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
                Interaction::WalletSetupWalletViewInteraction(
                    LocalViewInteraction::PasswordRepeatInput(s),
                )
            },
        )
        .on_submit(Interaction::WalletSetupWalletViewInteraction(
            LocalViewInteraction::PasswordRepeatInputEnterPressed,
        ))
        .size(DEFAULT_FONT_SIZE)
        .padding(6)
        .width(Length::Units(200))
        .style(style::AddonsQueryInput(color_palette))
        .password();

        let repeat_password_input: Element<Interaction> = repeat_password_input.into();

        let mut password_entry_value = localized_string("setup-grin-passwords-dont-match");
        if check_password() {
            password_entry_value = localized_string("setup-grin-passwords-okay")
        }
        let password_entry_status = Text::new(password_entry_value)
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);
        let mut password_entry_status_container = Container::new(password_entry_status)
            //.width(Length::Fill)
            .style(style::NormalErrorBackgroundContainer(color_palette));

        let mut password_input_col = Column::new()
            .push(password_input.map(Message::Interaction))
            .push(repeat_password_input.map(Message::Interaction))
            .spacing(DEFAULT_PADDING)
            .align_items(Alignment::Start);

        if !check_password() && disp_password_status() {
            password_input_col = password_input_col.push(password_entry_status_container)
        } else if check_password() {
            password_entry_status_container = password_entry_status_container
                .style(style::NormalSuccessBackgroundContainer(color_palette));
            password_input_col = password_input_col.push(password_entry_status_container)
        }
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
            |b| {
                Interaction::WalletSetupWalletViewInteraction(
                    LocalViewInteraction::ToggleRestoreFromSeed(b),
                )
            },
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
            |b| {
                Interaction::WalletSetupWalletViewInteraction(
                    LocalViewInteraction::ToggleAdvancedOptions(b),
                )
            },
        )
        .style(style::DefaultCheckbox(color_palette))
        .text_size(DEFAULT_FONT_SIZE)
        .spacing(10);

        let checkbox: Element<Interaction> = checkbox.into();

        let checkbox_container = Container::new(checkbox.map(Message::Interaction))
            .style(style::NormalBackgroundContainer(color_palette));
        Column::new().push(checkbox_container)
    };

    // ** start hideable advanced options
    //let display_name_label =
    let display_name = Text::new(localized_string("display-name"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let display_name_container =
        Container::new(display_name).style(style::NormalBackgroundContainer(color_palette));

    let display_name_input = TextInput::new(
        &mut state.advanced_options_state.display_name_input_state,
        &localized_string("wallet-default-name"),
        &state.advanced_options_state.display_name_value,
        |s| Interaction::WalletSetupWalletViewInteraction(LocalViewInteraction::DisplayName(s)),
    )
    .size(DEFAULT_FONT_SIZE)
    .padding(6)
    .width(Length::Units(200))
    .style(style::AddonsQueryInput(color_palette));

    let tld = Text::new(localized_string("top-level-domain"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let tld_container = Container::new(tld).style(style::NormalBackgroundContainer(color_palette));

    let folder_select_button_container =
        Container::new(Text::new(localized_string("select-directory")).size(DEFAULT_FONT_SIZE));
    let folder_select_button = Button::new(
        &mut state.advanced_options_state.folder_select_button_state,
        folder_select_button_container,
    )
    .style(style::DefaultBoxedButton(color_palette))
    .on_press(Interaction::WalletSetupWalletViewInteraction(
        LocalViewInteraction::ShowFolderPicker,
    ));
    let folder_select_button: Element<Interaction> = folder_select_button.into();

    let tld_string = state
        .advanced_options_state
        .top_level_directory
        .to_str()
        .unwrap();
    let current_tld = Text::new(tld_string).size(DEFAULT_FONT_SIZE);

    let current_tld_container =
        Container::new(current_tld).style(style::NormalBackgroundContainer(color_palette));

    let current_tld_column = Column::new()
        .push(Space::new(Length::Units(0), Length::Units(5)))
        .push(current_tld_container);

    let folder_select_row = Row::new()
        .push(folder_select_button.map(Message::Interaction))
        .push(Space::new(Length::Units(DEFAULT_PADDING), Length::Units(0)))
        .push(current_tld_column);

    let display_name_input: Element<Interaction> = display_name_input.into();

    let is_testnet_checkbox =
        Checkbox::new(state.is_testnet, localized_string("use-testnet"), |b| {
            Interaction::WalletSetupWalletViewInteraction(LocalViewInteraction::ToggleIsTestnet(b))
        })
        .style(style::DefaultCheckbox(color_palette))
        .text_size(DEFAULT_FONT_SIZE)
        .spacing(10);

    let is_testnet_checkbox: Element<Interaction> = is_testnet_checkbox.into();

    let is_testnet_row = Row::new().push(is_testnet_checkbox.map(Message::Interaction));

    let advanced_options_column = Column::new()
        .push(display_name_container)
        .push(display_name_input.map(Message::Interaction))
        .push(Space::new(Length::Units(0), Length::Units(5)))
        .push(tld_container)
        .spacing(DEFAULT_PADDING)
        .push(folder_select_row)
        .spacing(DEFAULT_PADDING)
        .push(Space::new(Length::Units(0), Length::Units(5)))
        .push(is_testnet_row)
        .align_items(Alignment::Start);

    // ** end hideable advanced options

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
    if check_password() {
        submit_button = submit_button.on_press(Interaction::WalletSetupWalletViewInteraction(
            LocalViewInteraction::CreateWallet,
        ));
    }

    let submit_button: Element<Interaction> = submit_button.into();

    let unit_spacing = 15;

    let mut column = Column::new()
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
        .push(Space::new(
            Length::Units(0),
            Length::Units(unit_spacing + 10),
        ));

    if state.show_advanced_options {
        column = column.push(advanced_options_column).push(Space::new(
            Length::Units(0),
            Length::Units(unit_spacing + 10),
        ));
    }

    column = column
        .push(submit_button.map(Message::Interaction))
        .align_items(Alignment::Start);

    Container::new(column)
        .center_y()
        .center_x()
        .width(Length::Fill)
}
