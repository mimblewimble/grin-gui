pub mod open;
pub mod home;
pub mod tx_list;

use {
    crate::gui::{style, GrinGui, Message},
    crate::Result,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::config::Config,
    iced::{Column, Command, Container, Length},
};

pub struct StateContainer {
    pub mode: Mode,
    pub open_state: open::StateContainer,
    pub home_state: home::StateContainer,
    // When changed to true, this should stay false until a wallet is opened with a password
    has_wallet_open_check_failed_one_time: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Open,
    Home,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            mode: Mode::Home,
            open_state: Default::default(),
            home_state: Default::default(),
            has_wallet_open_check_failed_one_time: false,
        }
    }
}

impl StateContainer {
    pub fn wallet_not_open(&self) -> bool {
        self.has_wallet_open_check_failed_one_time
    }

    pub fn set_wallet_not_open(&mut self) {
        self.has_wallet_open_check_failed_one_time = true;
        self.mode = Mode::Open;
    }

    pub fn clear_wallet_not_open(&mut self) {
        self.has_wallet_open_check_failed_one_time = false;
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
    config:&'a Config
) -> Container<'a, Message> {
    let content = match state.mode {
        Mode::Open => open::data_container(color_palette, &mut state.open_state, config),
        Mode::Home => {
            home::data_container(color_palette, config, &mut state.home_state)
        }
    };

    let column = Column::new()
        .push(content);

    Container::new(column)
        .center_y()
        .center_x()
        .width(Length::Fill)
        .style(style::NormalBackgroundContainer(color_palette))
}
