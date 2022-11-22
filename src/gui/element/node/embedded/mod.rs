use crate::gui::element::DEFAULT_PADDING;

pub mod summary;

use {
    crate::gui::{style, GrinGui, Message},
    crate::Result,
    grin_gui_core::node::ChainTypes,
    grin_gui_core::node::ServerStats,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::theme::{Container, Column},
    iced::{Command, Length},
    iced::widget::{container},
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
    chain_type: ChainTypes,
) -> Container<'a, Message> {
    let content = match state.mode {
        Mode::Summary => summary::data_container(
            color_palette,
            &mut state.summary_state,
            &state.server_stats,
            chain_type,
        ),
        _ => Container::new(Column::new()),
    };

    let column = Column::new().push(content);
    
    Container::new(column)
        .center_y()
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
        .style(grin_gui_core::theme::container::Container::NormalBackground(color_palette))
        .padding(iced::Padding::from([
            DEFAULT_PADDING, // top
            DEFAULT_PADDING, // right
            DEFAULT_PADDING, // bottom
            DEFAULT_PADDING, // left
        ]))
}
