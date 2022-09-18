use std::{path::PathBuf, sync::{Arc, RwLock}};
use crate::log_error;
use grin_gui_core::{config::Wallet, fs::PersistentData};
use iced::button::StyleSheet;
use iced_native::Widget;

use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    anyhow::Context,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::{config::Config, wallet::WalletInterface},
    iced::{
        alignment, button, text_input, Alignment, Button, Checkbox, Column, Command, Container,
        Element, Length, Row, Space, Text, TextInput,
    },
    iced_aw::Card,
};

/*
TODO: 
- search help in the wordlist
- card text input
*/

#[derive(Debug)]
pub struct StateContainer {
    pub password: String,
    pub top_level_directory: PathBuf,
    pub display_name: String,
    pub recovery_phrase_state: SeedState,
    pub copy_button_state: button::State,
    pub next_button_state: button::State,
    pub is_valid: bool
    // TODO: ZeroingString this
    
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            password: Default::default(),
            top_level_directory: Default::default(),
            display_name: Default::default(),
            recovery_phrase_state: Default::default(),
            copy_button_state: Default::default(),
            next_button_state: Default::default(),
            is_valid: false
        }
    }
}
#[derive(Debug)]
pub struct SeedState {
    input_state: text_input::State,
    input_value: String,
}

impl Default for SeedState {
    fn default() -> Self {
        SeedState {
            input_state: Default::default(),
            input_value: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    SeedInput(String),
    SeedInputEnterPressed,
    Submit,
    WalletCreatedOk,
    WalletCreateError(Arc<RwLock<Option<anyhow::Error>>>),
}

pub fn handle_message(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let state = &mut grin_gui.wallet_state.setup_state.setup_wallet_restore;
    match message {
        LocalViewInteraction::SeedInput(seed) => {
            state.recovery_phrase_state.input_value = seed.clone();
            let words = seed.split_whitespace();
            let vec_words: Vec<&str> = words.collect();
            let size_seed = vec_words.len();
            if size_seed == 12 ||  size_seed == 24 {
                let w = grin_gui.wallet_interface.clone();
                let pass = state.recovery_phrase_state.input_value.clone();
                match WalletInterface::validate_mnemonic(w, pass) {
                    Ok(e) => {
                        log::debug!("valid seed {:?}", e);
                        state.is_valid = true
                    }, 
                    Err(e) => {
                        log::debug!("invalid seed {:?}", e);
                        state.is_valid = false
                    }
                }
            }
            
        }
        LocalViewInteraction::SeedInputEnterPressed => {
            state.recovery_phrase_state.input_state.unfocus();
        },
        LocalViewInteraction::Submit => {
            let password = state.password.clone();
            let w = grin_gui.wallet_interface.clone();
            

            let fut = move || {
                WalletInterface::init(
                    w,
                    password,
                    state.top_level_directory.clone(),
                    state.display_name.clone(),
                    32
                )
            };
        }
        LocalViewInteraction::WalletCreatedOk => {
            grin_gui.wallet_state.setup_state.mode =
                crate::gui::element::wallet::setup::Mode::WalletCreateSuccess;
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
    // Title row
    let title = Text::new(localized_string("setup-grin-wallet-success"))
        .size(DEFAULT_HEADER_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let title_container =
        Container::new(title).style(style::BrightBackgroundContainer(color_palette));

    let title_row = Row::new()
        .push(title_container)
        .align_items(Alignment::Center)
        .spacing(20);

    let description = Text::new(localized_string("setup-grin-wallet-recovery-phrase"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);
    let description_container =
        Container::new(description).style(style::NormalBackgroundContainer(color_palette)); 

    let recovery_phrase_column = {
        let recovery_phrase_input = TextInput::new(
            &mut state.recovery_phrase_state.input_state,
            &localized_string("password")[..],
            &state.recovery_phrase_state.input_value,
            |s| {
                Interaction::WalletSetupWalletRestoreViewInteraction(LocalViewInteraction::SeedInput(s))
            },
        )
        .on_submit(Interaction::WalletSetupWalletRestoreViewInteraction(
            LocalViewInteraction::SeedInputEnterPressed
        ))
        .size(DEFAULT_FONT_SIZE)
        .padding(6)
        .width(Length::Units(400))
        .style(style::AddonsQueryInput(color_palette));

        let recovery_phrase_input: Element<Interaction> = recovery_phrase_input.into();

        let recovery_phrase_input_col = Column::new()
            .push(recovery_phrase_input.map(Message::Interaction))
            .spacing(DEFAULT_PADDING)
            .align_items(Alignment::Center);
        
        /*let check_seed = || {
            !state.recovery_phrase_state.input_value.is_empty() && state.is_valid
        };
    
        let mut seed_entry_value = localized_string("");
        if check_seed() {
            seed_entry_value = localized_string("setup-grin-passwords-okay")
        }
        
        let seed_entry_status = Text::new(seed_entry_value)
                .size(DEFAULT_FONT_SIZE)
                .horizontal_alignment(alignment::Horizontal::Left);

        let seed_entry_status_container = Container::new(seed_entry_status)
            //.width(Length::Fill)
            .style(style::NormalSuccessBackgroundContainer(color_palette));*/

        Column::new().push(recovery_phrase_input_col)//.push(seed_entry_status_container)
        
    };

    

    let submit_button_label_container =
        Container::new(Text::new(localized_string("setup-grin-wallet-done")).size(DEFAULT_FONT_SIZE))
            .center_x()
            .align_x(alignment::Horizontal::Center);

    let mut next_button = Button::new(
        &mut state.next_button_state, 
        submit_button_label_container
    )
    .style(style::DefaultBoxedButton(color_palette));
    
    if state.is_valid {
        next_button = next_button.on_press(Interaction::WalletSetupWalletRestoreViewInteraction(
            LocalViewInteraction::Submit,
        ));

    }
        

    let next_button: Element<Interaction> = next_button.into();

    let unit_spacing = 15;

    let colum = Column::new()
        .push(title_row)
        .push(Space::new(
            Length::Units(0),
            Length::Units(unit_spacing + 5),
        ))
        .push(description_container)
        .push(Space::new(
            Length::Units(0),
            Length::Units(unit_spacing + 5),
        ))
        .push(recovery_phrase_column)
        .push(Space::new(
            Length::Units(0),
            Length::Units(unit_spacing + 10),
        ))
        .push(next_button.map(Message::Interaction))
        .align_items(Alignment::Center);

    Container::new(colum)
        .center_y()
        .center_x()
        .width(Length::Fill)
}
