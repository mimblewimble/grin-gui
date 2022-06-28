pub mod init;
pub mod wallet;

use {
    super::{DEFAULT_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, Message},
    crate::Result,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::{config::Config, wallet::WalletInterface},
    iced::{
        Command, Column, Container, Length, Space
    },
};

pub struct StateContainer {
    pub mode: Mode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Init,
    CreateWallet,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            mode: Mode::Init,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
}

pub fn handle_message(
    state: &mut StateContainer,
    config: &mut Config,
    wallet_interface: &mut WalletInterface,
    message: LocalViewInteraction,
    error: &mut Option<anyhow::Error>,
) -> Result<Command<Message>> {
    Ok(Command::none())
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    state: &'a mut StateContainer,
    init_state: &'a mut init::StateContainer,
    wallet_state: &'a mut wallet::StateContainer,
) -> Container<'a, Message> {
    let content = match state.mode {
        Mode::Init => {
           init::data_container(color_palette, init_state)
        }
        Mode::CreateWallet => {
           wallet::data_container(color_palette, wallet_state)
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
