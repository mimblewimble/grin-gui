pub mod action_menu;
pub mod apply_tx;
pub mod apply_tx_confirm;
pub mod chart;
pub mod create_tx;
pub mod show_slatepack;
pub mod home;
pub mod open;
pub mod tx_list;
pub mod tx_list_display;
pub mod tx_detail;
pub mod tx_done;

use {
    crate::gui::{GrinGui, Message},
    crate::Result,
    grin_gui_core::config::Config,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::theme::{
        Button, Column, Container, Element, PickList, Row, Scrollable, Text, TextInput,
    },
    iced::{Command, Length},
};

pub struct StateContainer {
    pub mode: Mode,
    pub open_state: open::StateContainer,
    pub home_state: home::StateContainer,
    pub create_tx_state: create_tx::StateContainer,
    pub show_slatepack_state: show_slatepack::StateContainer,
    pub apply_tx_state: apply_tx::StateContainer,
    pub tx_detail_state: tx_detail::StateContainer,
    pub tx_done_state: tx_done::StateContainer,
    // When changed to true, this should stay false until a wallet is opened with a password
    has_wallet_open_check_failed_one_time: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Open,
    Home,
    CreateTx,
    ApplyTx,
    ShowSlatepack,
    TxDetail,
    TxDone,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            mode: Mode::Home,
            open_state: Default::default(),
            home_state: Default::default(),
            create_tx_state: Default::default(),
            show_slatepack_state: Default::default(),
            apply_tx_state: Default::default(),
            tx_detail_state: Default::default(),
            tx_done_state: Default::default(),
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

pub fn data_container<'a>(state: &'a StateContainer, config: &'a Config) -> Container<'a, Message> {
    let content = match state.mode {
        Mode::Open => open::data_container(&state.open_state, config),
        Mode::Home => home::data_container(config, &state.home_state),
        Mode::CreateTx => create_tx::data_container(config, &state.create_tx_state),
        Mode::ShowSlatepack => {
            show_slatepack::data_container(config, &state.show_slatepack_state)
        }
        Mode::ApplyTx => apply_tx::data_container(config, &state.apply_tx_state),
        Mode::TxDetail => {
            tx_detail::data_container(config, &state.tx_detail_state)
        }
        Mode::TxDone => {
            tx_done::data_container(config, &state.tx_done_state)
        }
    };

    let column = Column::new().push(content);

    Container::new(column)
        .center_y()
        .center_x()
        .width(Length::Fill)
        .style(grin_gui_core::theme::ContainerStyle::NormalBackground)
}
