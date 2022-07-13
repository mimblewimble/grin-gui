pub mod init;
pub mod wallet_setup;
pub mod wallet_success;

use {
    super::super::{DEFAULT_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, GrinGui, Message},
    crate::Result,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::{config::Config, wallet::WalletInterface},
    iced::{Column, Command, Container, Length, Space},
};

pub struct StateContainer {
    pub mode: Mode,
    pub setup_init_state: init::StateContainer,
    pub setup_wallet_state: wallet_setup::StateContainer,
    pub setup_wallet_success_state: wallet_success::StateContainer,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Init,
    CreateWallet,
    WalletCreateSuccess,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            mode: Mode::Init,
            setup_init_state: Default::default(),
            setup_wallet_state: Default::default(),
            setup_wallet_success_state: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {}

pub fn handle_message(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    Ok(Command::none())
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    state: &'a mut StateContainer,
) -> Container<'a, Message> {
    let content = match state.mode {
        Mode::Init => init::data_container(color_palette, &mut state.setup_init_state),
        Mode::CreateWallet => {
            wallet_setup::data_container(color_palette, &mut state.setup_wallet_state)
        }
        Mode::WalletCreateSuccess => {
            wallet_success::data_container(color_palette, &mut state.setup_wallet_success_state)
        }
    };

    let column = Column::new()
        .push(Space::new(Length::Units(0), Length::Units(20)))
        .push(content);

    Container::new(column)
        .center_y()
        .center_x()
        .width(Length::Fill)
        .style(style::NormalBackgroundContainer(color_palette))
}
