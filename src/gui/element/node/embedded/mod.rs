use crate::gui::element::DEFAULT_PADDING;
use iced_style::container::StyleSheet;

pub mod summary;

use {
	crate::gui::{GrinGui, Message},
	crate::Result,
	grin_gui_core::node::ChainTypes,
	grin_gui_core::node::ServerStats,
	grin_gui_core::theme::ColorPalette,
	grin_gui_core::theme::{Column, Container, Element},
	iced::widget::container,
	iced::Command,
	iced_core::Length,
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
	state: &'a StateContainer,
	chain_type: ChainTypes,
) -> Container<'a, Message> {
	let content = match state.mode {
		Mode::Summary => {
			summary::data_container(&state.summary_state, &state.server_stats, chain_type)
		}
		_ => Container::new(Column::new()).into(),
	};

	let column = Column::new().push(content);

	Container::new(column)
		.center_y()
		.center_x()
		.width(Length::Fill)
		.height(Length::Fill)
		.style(grin_gui_core::theme::ContainerStyle::NormalBackground)
		.padding(iced::Padding {
			top: DEFAULT_PADDING,    // top
			right: DEFAULT_PADDING,  // right
			bottom: DEFAULT_PADDING, // bottom
			left: DEFAULT_PADDING,   // left
		})
}
