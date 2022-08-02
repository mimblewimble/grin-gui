pub mod summary;

use {
    super::super::{DEFAULT_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, GrinGui, Message},
    crate::Result,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::{config::Config, wallet::WalletInterface},
    grin_gui_core::node::ServerStats,
    iced::{Column, Command, Container, Length, Space},
};

pub struct StateContainer {
    pub mode: Mode,
    pub server_stats: Option<ServerStats>,
    pub summary_state: summary::StateContainer,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Summary,
    Peers,
    // etc as in TUI
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            mode: Mode::Summary,
            server_stats: None,
            summary_state: Default::default(),
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
        Mode::Summary => summary::data_container(color_palette, &mut state.summary_state, &state.server_stats),
        _ => Container::new(Column::new())
    };

    let column = Column::new()
        .push(content);

    Container::new(column)
        .center_y()
        .center_x()
        .width(Length::Fill)
        .style(style::NormalBackgroundContainer(color_palette))
}
