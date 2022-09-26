pub mod operation;
pub mod setup;

use {
    crate::gui::{style, Message},
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::{config::Config},
    iced::{Column, Container, Length},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Init,
    CreateWallet,
    Operation,
}

pub struct StateContainer {
    pub mode: Mode,
    pub setup_state: setup::StateContainer,
    pub operation_state: operation::StateContainer,
    // When changed to true, this should stay false until a config exists
    has_config_check_failed_one_time: bool,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            mode: Mode::Operation,
            setup_state: Default::default(),
            operation_state: Default::default(),
            has_config_check_failed_one_time: false,
        }
    }
}

impl StateContainer {
    pub fn config_missing(&self) -> bool {
        self.has_config_check_failed_one_time
    }

    pub fn set_config_missing(&mut self) {
        self.has_config_check_failed_one_time = true;
        self.mode = Mode::Init;
        self.setup_state.mode = crate::gui::element::wallet::setup::Mode::Init;
    }

    pub fn clear_config_missing(&mut self) {
        self.has_config_check_failed_one_time = false;
    }
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    state: &'a mut StateContainer,
    config: &'a Config
) -> Container<'a, Message> {
    let content = match state.mode {
        Mode::Init => setup::data_container(color_palette, &mut state.setup_state, config),
        Mode::Operation => operation::data_container(color_palette, &mut state.operation_state, config),
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
