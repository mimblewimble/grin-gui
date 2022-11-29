pub mod embedded;

use {
    crate::gui::{Message},
    grin_gui_core::{theme::ColorPalette, node::ChainTypes},
    iced::Length,
    grin_gui_core::theme::{Container, Column},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Embedded,
    // etc, as in TUI for now
}

pub struct StateContainer {
    pub mode: Mode,
    //pub external_state: external::StateContainer,
    pub embedded_state: embedded::StateContainer,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            mode: Mode::Embedded,
            embedded_state: Default::default(),
        }
    }
}

impl StateContainer {
}

pub fn data_container<'a>(
    state: &'a StateContainer,
    chain_type: ChainTypes,
) -> Container<'a, Message> {
    let content = match state.mode {
        Mode::Embedded => embedded::data_container(&state.embedded_state, chain_type),
        //_ => Container::new(Column::new()),
    };

    let column = Column::new()
        //.push(Space::new(Length::Units(0), Length::Units(20)))
        .push(content);

    Container::new(column)
        .center_y()
        .center_x()
        .width(Length::Fill)
        .style(grin_gui_core::theme::ContainerStyle::NormalBackground)
}
