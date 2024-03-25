#![allow(clippy::type_complexity)]
use crate::style::header::StyleSheet;
use iced::advanced::renderer;
use iced_core::{
	event, layout, mouse,
	widget::{self, Tree},
	Alignment, Clipboard, Element, Event, Layout, Length, Padding, Rectangle, Shell, Widget,
};

use iced::{widget::space::Space, Size};

mod state;
pub use state::State;

pub struct Header<'a, Message, Theme, Renderer = iced::Renderer>
where
	Renderer: renderer::Renderer,
	Theme: StyleSheet,
	Message: Clone,
{
	spacing: u16,
	width: Length,
	height: Length,
	state: State,
	leeway: u16,
	on_resize: Option<(u16, Box<dyn Fn(ResizeEvent) -> Message + 'a>)>,
	children: Vec<Element<'a, Message, Theme, Renderer>>,
	left_margin: bool,
	right_margin: bool,
	names: Vec<String>,
	style: <Theme as StyleSheet>::Style,
}

impl<'a, Message, Theme, Renderer> Header<'a, Message, Theme, Renderer>
where
	Renderer: renderer::Renderer,
	Theme: StyleSheet,
	Message: Clone,
{
	pub fn new(
		state: State,
		headers: Vec<(String, Element<'a, Message, Theme, Renderer>)>,
		left_margin: Option<Length>,
		right_margin: Option<Length>,
	) -> Self
	where
		Theme: iced_style::container::StyleSheet,
		Renderer: 'a + iced_core::Renderer,
		Message: 'a,
	{
		let mut names = vec![];
		let mut left = false;
		let mut right = false;

		let mut children = vec![];

		if let Some(margin) = left_margin {
			children.push(Space::with_width(margin).into());
			left = true;
		}

		for (key, container) in headers {
			names.push(key);

			// add container to children
			children.push(container.into());
		}

		if let Some(margin) = right_margin {
			children.push(Space::with_width(margin).into());
			right = true;
		}

		Self {
			spacing: 0,
			width: Length::Fill,
			height: Length::Fill,
			leeway: 0,
			state,
			on_resize: None,
			children,
			left_margin: left,
			right_margin: right,
			names,
			style: Default::default(),
		}
	}

	pub fn spacing(mut self, units: u16) -> Self {
		self.spacing = units;
		self
	}

	pub fn width(mut self, width: Length) -> Self {
		self.width = width;
		self
	}

	pub fn height(mut self, height: Length) -> Self {
		self.height = height;
		self
	}

	pub fn on_resize<F>(mut self, leeway: u16, f: F) -> Self
	where
		F: 'a + Fn(ResizeEvent) -> Message,
	{
		self.leeway = leeway;
		self.on_resize = Some((leeway, Box::new(f)));
		self
	}

	fn trigger_resize(
		&self,
		left_name: String,
		left_width: u16,
		right_name: String,
		right_width: u16,
		shell: &mut Shell<'_, Message>,
	) {
		if let Some((_, on_resize)) = &self.on_resize {
			//TODO: Update
			shell.publish(on_resize(ResizeEvent::ResizeColumn {
				left_name,
				left_width,
				right_name,
				right_width,
			}));
		}
	}

	fn trigger_finished(&self, shell: &mut Shell<'_, Message>) {
		if let Some((_, on_resize)) = &self.on_resize {
			shell.publish(on_resize(ResizeEvent::Finished));
		}
	}
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
	for Header<'a, Message, Theme, Renderer>
where
	Renderer: renderer::Renderer,
	Theme: StyleSheet,
	Message: Clone,
{
	fn children(&self) -> Vec<Tree> {
		self.children.iter().map(Tree::new).collect()
	}

	fn diff(&self, tree: &mut Tree) {
		tree.diff_children(&self.children);
	}

	fn size(&self) -> Size<Length> {
		Size {
			width: self.width,
			height: self.height,
		}
	}

	fn layout(
		&self,
		tree: &mut Tree,
		renderer: &Renderer,
		limits: &layout::Limits,
	) -> layout::Node {
		let limits = limits.width(self.width).height(self.height);
		let mut trees = self.children();

		layout::flex::resolve(
			layout::flex::Axis::Horizontal,
			renderer,
			&limits,
			self.width,
			self.height,
			Padding::ZERO,
			self.spacing as f32,
			Alignment::Start,
			&self.children,
			&mut trees,
		)
	}

	fn on_event(
		&mut self,
		tree: &mut Tree,
		event: Event,
		layout: Layout<'_>,
		cursor: mouse::Cursor,
		renderer: &Renderer,
		clipboard: &mut dyn Clipboard,
		shell: &mut Shell<'_, Message>,
		viewport: &Rectangle,
	) -> event::Status {
		let cursor_position = cursor.position().unwrap_or_default();
		let in_bounds = layout.bounds().contains(cursor_position);

		if self.state.resizing || in_bounds {
			let child_len = self.children.len();
			let start_offset = if self.left_margin { 1 } else { 0 };
			let end_offset = if self.right_margin { 1 } else { 0 };

			let dividers = self
				.children
				.iter()
				.enumerate()
				.zip(layout.children())
				.filter_map(|((idx, _), layout)| {
					if idx >= start_offset && idx < (child_len - 1 - end_offset) {
						Some((idx, layout.position().x + layout.bounds().width))
					} else {
						None
					}
				})
				.collect::<Vec<_>>();

			if self.on_resize.is_some() {
				if !self.state.resizing {
					self.state.resize_hovering = false;
				}

				for (idx, divider) in dividers.iter() {
					if cursor_position.x > (divider - self.leeway as f32)
						&& cursor_position.x < (divider + self.leeway as f32)
					{
						if !self.state.resize_hovering {
							self.state.resizing_idx = *idx;
						}

						self.state.resize_hovering = true;
					}
				}
			}

			match event {
				Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
					if self.state.resize_hovering {
						self.state.resizing = true;
						self.state.starting_cursor_pos = Some(cursor_position);
						self.state.starting_left_width = layout
							.children()
							.nth(self.state.resizing_idx)
							.unwrap()
							.bounds()
							.width;
						self.state.starting_right_width = layout
							.children()
							.nth(self.state.resizing_idx + 1)
							.unwrap()
							.bounds()
							.width;
						return event::Status::Captured;
					}
				}
				Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
					if self.state.resizing {
						self.state.resizing = false;
						self.state.starting_cursor_pos.take();
						// TODO: UPDATE
						//shell.publish(messages);
						return event::Status::Captured;
					}
				}
				Event::Mouse(mouse::Event::CursorMoved { position }) => {
					if self.state.resizing {
						let delta = position.x - self.state.starting_cursor_pos.unwrap().x;

						let left_width = self.state.starting_left_width;
						let right_width = self.state.starting_right_width;

						let max_width = left_width + right_width - 30.0;

						let left_width = (left_width + delta).max(30.0).min(max_width) as u16;
						let left_name = &self.names[self.state.resizing_idx - start_offset];
						let right_width = (right_width - delta).max(30.0).min(max_width) as u16;
						let right_name = &self.names[self.state.resizing_idx + 1 - start_offset];

						self.trigger_resize(
							left_name.clone(),
							left_width,
							right_name.clone(),
							right_width,
							shell,
						);
						return event::Status::Captured;
					}
				}
				_ => {}
			}
		} else {
			self.state.resize_hovering = false;
		}

		self.children
			.iter_mut()
			.zip(&mut tree.children)
			.zip(layout.children())
			.map(|((child, state), layout)| {
				child.as_widget_mut().on_event(
					state,
					event.clone(),
					layout,
					cursor,
					renderer,
					clipboard,
					shell,
					viewport,
				)
			})
			.fold(event::Status::Ignored, event::Status::merge)
	}

	fn draw(
		&self,
		tree: &Tree,
		renderer: &mut Renderer,
		theme: &Theme,
		style: &iced_core::renderer::Style,
		layout: Layout<'_>,
		cursor: mouse::Cursor,
		viewport: &Rectangle,
	) {
		for ((child, state), layout) in self
			.children
			.iter()
			.zip(&tree.children)
			.zip(layout.children())
		{
			child
				.as_widget()
				.draw(state, renderer, theme, style, layout, cursor, viewport);
		}
	}

	fn mouse_interaction(
		&self,
		tree: &Tree,
		layout: Layout<'_>,
		cursor: mouse::Cursor,
		viewport: &Rectangle,
		renderer: &Renderer,
	) -> mouse::Interaction {
		let bounds = layout.bounds();
		let cursor_position = cursor.position().unwrap_or_default();
		let is_mouse_over = bounds.contains(cursor_position);

		if is_mouse_over {
			mouse::Interaction::Pointer
		} else {
			mouse::Interaction::default()
		}
	}

	/*fn hash_layout(&self, state: &mut Hasher) {
		use std::hash::Hash;

		struct Marker;
		std::any::TypeId::of::<Marker>().hash(state);

		self.width.hash(state);
		self.height.hash(state);
		self.spacing.hash(state);
		self.left_margin.hash(state);
		self.right_margin.hash(state);
		self.leeway.hash(state);

		for child in &self.children {
			child.hash_layout(state);
		}
	}*/
}

impl<'a, Message, Theme> From<Header<'a, Message, Theme>>
	for Element<'a, Message, Theme, iced::Renderer>
where
	//Renderer: 'a + renderer::Renderer,
	Theme: 'a + StyleSheet + iced::widget::container::StyleSheet + widget::text::StyleSheet,
	Message: 'a + Clone,
{
	fn from(header: Header<'a, Message, Theme>) -> Self {
		Element::new(header)
	}
}

#[derive(Debug, Clone)]
pub enum ResizeEvent {
	ResizeColumn {
		left_name: String,
		left_width: u16,
		right_name: String,
		right_width: u16,
	},
	Finished,
}
