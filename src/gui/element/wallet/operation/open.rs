use crate::gui::element::{self, BUTTON_HEIGHT, BUTTON_WIDTH};
use crate::log_error;
//use iced::button::StyleSheet;
//use iced_native::Widget;
//use std::path::PathBuf;

use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    anyhow::Context,
    grin_gui_core::config::Config,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::{
        node::ChainTypes::Mainnet, node::ChainTypes::Testnet, wallet::WalletInterface,
    },
    grin_gui_core::theme::{
        Button, Column, Container, Element, PickList, Row, Scrollable, Text, TextInput,
    },
    iced::{alignment, Alignment, Command, Length},
    iced::widget::{
        button, pick_list, scrollable, text_input, Space,
    },
    std::sync::{Arc, RwLock},
};

static INPUT_WIDTH: u16 = 200;
static UNIT_SPACE: u16 = 15;

pub struct StateContainer {
    pub password_state: PasswordState,
    // pub submit_button_state: button::State,
    // cancel_button_state: button::State,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            password_state: Default::default(),
            // submit_button_state: Default::default(),
            // cancel_button_state: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PasswordState {
    // pub input_state: text_input::State,
    pub input_value: String,
}

impl Default for PasswordState {
    fn default() -> Self {
        PasswordState {
            // input_state: Default::default(),
            input_value: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    //TODO: ZeroingString these
    PasswordInput(String),
    PasswordInputEnterPressed,
    OpenWallet,
    CancelOpenWallet,
    WalletOpenedOkay,
    WalletOpenError(Arc<RwLock<Option<anyhow::Error>>>),
}

pub fn handle_message<'a>(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let state = &mut grin_gui.wallet_state.operation_state.open_state;
    match message {
        LocalViewInteraction::CancelOpenWallet => {
            // TODO @sheldonth do we need to "close" any wallet interface?
            //      @sheldonth if the wallet we're currently prompted for uses
            //                 the node it needs to be shutdown.

            // return user to wallet list
            grin_gui.wallet_state.mode = element::wallet::Mode::Init;
            grin_gui.wallet_state.setup_state.mode = element::wallet::setup::Mode::ListWallets;

            // reset user input values
            grin_gui.wallet_state.operation_state.open_state = Default::default();
        }
        LocalViewInteraction::PasswordInput(password) => {
            state.password_state.input_value = password;
        }
        LocalViewInteraction::PasswordInputEnterPressed => {
            //state.password_state.input_state.unfocus();
        }
        LocalViewInteraction::OpenWallet => {
            grin_gui.error.take();

            log::debug!("setup::wallet::operation::open::OpenWallet");

            let password = state.password_state.input_value.clone();
            let wallet_interface = grin_gui.wallet_interface.clone();
            let running_chain_type = grin_gui.node_interface.read().unwrap().chain_type.unwrap();
            let wallet_index = grin_gui.config.current_wallet_index.unwrap();
            let current_wallet = &grin_gui.config.wallets[wallet_index];
            let wallet_chain_type = current_wallet.chain_type;

            if current_wallet.use_embedded_node {
                // restart embedded server is chain types differ
                if running_chain_type != wallet_chain_type {
                    let mut node = grin_gui.node_interface.write().unwrap();
                    node.restart_server(wallet_chain_type);
                }

                let node_interface = grin_gui.node_interface.read().unwrap();
                if let Some(c) = &node_interface.config {
                    if let Some(m) = &c.members {
                        WalletInterface::set_use_embedded_node(wallet_interface.clone(), true);
                        let mut w = wallet_interface.write().unwrap();
                        w.check_node_foreign_api_secret_path =
                            m.server.foreign_api_secret_path.clone();
                    }
                }
            }
            let tld = current_wallet.tld.clone().unwrap();
            let fut = move || {
                WalletInterface::open_wallet(
                    wallet_interface,
                    password.clone(),
                    tld,
                    current_wallet.chain_type,
                )
            };

            return Ok(Command::perform(fut(), |r| {
                match r.context("Failed to Open Wallet") {
                    Ok(()) => {
                        Message::Interaction(Interaction::WalletOperationOpenViewInteraction(
                            LocalViewInteraction::WalletOpenedOkay,
                        ))
                    }
                    Err(e) => {
                        Message::Interaction(Interaction::WalletOperationOpenViewInteraction(
                            LocalViewInteraction::WalletOpenError(Arc::new(RwLock::new(Some(e)))),
                        ))
                    }
                }
            }));
        }
        LocalViewInteraction::WalletOpenedOkay => {
            grin_gui
                .wallet_state
                .operation_state
                .clear_wallet_not_open();
            grin_gui.wallet_state.operation_state.mode =
                crate::gui::element::wallet::operation::Mode::Home;

            // reset user input values
            grin_gui.wallet_state.operation_state.open_state = Default::default();
        }

        LocalViewInteraction::WalletOpenError(err) => {
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
    config: &Config,
) -> Container<'a, Message> {
    let mut display_name_string = match config.current_wallet_index {
        Some(index) => config.wallets[index].display_name.clone(),
        None => "".to_owned(),
    };

    // if there is no wallet display name
    if display_name_string.is_empty() {
        display_name_string = localized_string("open-wallet");
    }

    let display_name = Text::new(display_name_string)
        .size(DEFAULT_HEADER_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);
    let display_name_container =
        Container::new(display_name).style(grin_gui_core::theme::container::Container::NormalBackground(color_palette));

    let password_column = {
        let password_input = TextInput::new(
            // &mut state.password_state.input_state,
            &localized_string("password")[..],
            &state.password_state.input_value,
            |s| {
                Interaction::WalletOperationOpenViewInteraction(
                    LocalViewInteraction::PasswordInput(s),
                )
            },
        )
        .on_submit(Interaction::WalletOperationOpenViewInteraction(
            LocalViewInteraction::PasswordInputEnterPressed,
        ))
        .size(DEFAULT_FONT_SIZE)
        .padding(6)
        .width(Length::Units(INPUT_WIDTH))
        .style(grin_gui_core::theme::text_input::TextInputStyles::AddonsQuery(color_palette))
        .password();

        let password_input: Element<Interaction> = password_input.into();

        let password_input_col = Column::new()
            .push(password_input.map(Message::Interaction))
            .spacing(DEFAULT_PADDING)
            .align_items(Alignment::Center);

        Column::new().push(password_input_col)
    };

    let description = Text::new(localized_string("open-wallet-password"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);

    let description_container = Container::new(description)
        .width(Length::Units(INPUT_WIDTH))
        .style(grin_gui_core::theme::container::Container::NormalBackground(color_palette));

    let submit_button_label_container =
        Container::new(Text::new(localized_string("open")).size(DEFAULT_FONT_SIZE))
            .center_x()
            .center_y()
            .width(Length::Units(BUTTON_WIDTH))
            .height(Length::Units(BUTTON_HEIGHT))
            .align_x(alignment::Horizontal::Center);

    let mut submit_button = Button::new(
        // &mut state.submit_button_state,
        submit_button_label_container,
    )
    .style(grin_gui_core::theme::button::Button::Primary(color_palette));

    submit_button = submit_button.on_press(Interaction::WalletOperationOpenViewInteraction(
        LocalViewInteraction::OpenWallet,
    ));

    let submit_button: Element<Interaction> = submit_button.into();

    let cancel_button_label_container =
        Container::new(Text::new(localized_string("cancel")).size(DEFAULT_FONT_SIZE))
            .center_x()
            .center_y()
            .width(Length::Units(BUTTON_WIDTH))
            .height(Length::Units(BUTTON_HEIGHT))
            .align_x(alignment::Horizontal::Center);

    let mut cancel_button = Button::new(
        // &mut state.cancel_button_state,
        cancel_button_label_container,
    )
    .style(grin_gui_core::theme::button::Button::Primary(color_palette));

    cancel_button = cancel_button.on_press(Interaction::WalletOperationOpenViewInteraction(
        LocalViewInteraction::CancelOpenWallet,
    ));

    // give our buttons a nice double bordered look to match toolbar buttons
    let submit_button: Element<Interaction> = submit_button.into();
    let submit_container = Container::new(submit_button.map(Message::Interaction)).padding(1);
    let submit_container = Container::new(submit_container)
        .style(grin_gui_core::theme::container::Container::Segmented(color_palette))
        .padding(1);

    let cancel_button: Element<Interaction> = cancel_button.into();
    let cancel_container = Container::new(cancel_button.map(Message::Interaction)).padding(1);
    let cancel_container = Container::new(cancel_container)
        .style(grin_gui_core::theme::container::Container::Segmented(color_palette))
        .padding(1);

    let button_row = Row::new()
        .push(submit_container)
        .push(Space::with_width(Length::Units(UNIT_SPACE)))
        .push(cancel_container);

    let column = Column::new()
        .push(display_name_container)
        .push(Space::with_height(Length::Units(
            UNIT_SPACE + DEFAULT_PADDING,
        )))
        .push(description_container)
        .push(Space::with_height(Length::Units(UNIT_SPACE)))
        .push(password_column)
        .push(Space::with_height(Length::Units(
            UNIT_SPACE + DEFAULT_PADDING,
        )))
        .push(button_row)
        .align_items(Alignment::Center);

    Container::new(column)
        .center_y()
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
}
