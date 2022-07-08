pub mod setup;

use {
    super::super::{DEFAULT_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, GrinGui, Message},
    crate::Result,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::{config::Config, wallet::WalletInterface},
    iced::{
        Command, Column, Container, Length, Space
    },
};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Init,
    CreateWallet,
}

pub struct StateContainer {
    pub mode: Mode,
    pub setup_state: setup::StateContainer,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            mode: Mode::Init,
            setup_state: Default::default(),
        }
    }
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    state: &'a mut StateContainer,
) -> Container<'a, Message> {
    let content = match state.mode {
        Mode::Init => setup::data_container(
            color_palette,
            &mut state.setup_state,
        ),
        _ => Container::new(Column::new())
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
