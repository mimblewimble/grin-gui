pub mod embedded;

use {
    super::super::{DEFAULT_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, GrinGui, Message},
    crate::Result,
    grin_gui_core::theme::ColorPalette,
    iced::{Column, Command, Container, Length, Space},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    External,
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
            //summary_state: Default::default(),
        }
    }
}

impl StateContainer {
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    state: &'a mut StateContainer,
) -> Container<'a, Message> {
    let content = match state.mode {
        Mode::Embedded => embedded::data_container(color_palette, &mut state.embedded_state),
        _ => Container::new(Column::new()),
    };

    let column = Column::new()
        //.push(Space::new(Length::Units(0), Length::Units(20)))
        .push(content);

    Container::new(column)
        .center_y()
        .center_x()
        .width(Length::Fill)
        .style(style::NormalBackgroundContainer(color_palette))
}
