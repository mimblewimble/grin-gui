use {
    super::super::super::{
        BUTTON_HEIGHT, BUTTON_WIDTH, DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING,
        SMALLER_FONT_SIZE,
    },
    crate::gui::{GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    grin_gui_core::config::Config,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::theme::{
        Column, Container, Element, PickList, Row, Scrollable, Text, TextInput,
    },
    iced::widget::{button, pick_list, scrollable, text_input, Button, Checkbox, Space},
    iced::{alignment, Alignment, Command, Length},
    iced_aw::Card,
};

pub struct StateContainer {
    // Encrypted slate to send to recipient
    pub encrypted_slate: Option<String>,
    // Where the 'submit' or back button leads to 
    pub submit_mode: Option<crate::gui::element::wallet::operation::Mode>,
    // Label to display as title
    pub title_label: String,
    // description
    pub desc: String,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            encrypted_slate: Default::default(),
            submit_mode: None,
            title_label: localized_string("tx-view"),
            desc: localized_string("tx-view-desc")
        }
    }
}

impl StateContainer {
    pub fn reset_defaults(&mut self) {
        self.title_label = localized_string("tx-view");
        self.desc = localized_string("tx-view-desc");
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    Submit,
}

pub fn handle_message(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let state = &mut grin_gui.wallet_state.operation_state.show_slatepack_state;
    match message {
        LocalViewInteraction::Submit => {
            state.encrypted_slate = None;
            state.reset_defaults();
            if let Some(ref m) = state.submit_mode {
                grin_gui.wallet_state.operation_state.mode = m.clone();
            } else {
                grin_gui.wallet_state.operation_state.mode =
                    crate::gui::element::wallet::operation::Mode::Home;
            }
            state.submit_mode = None;
        }
    }
    Ok(Command::none())
}

pub fn data_container<'a>(
    _config: &'a Config,
    state: &'a StateContainer,
) -> Container<'a, Message> {
    // Title row
    let title = Text::new(state.title_label.clone())
        .size(DEFAULT_HEADER_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);

    let title_container = Container::new(title)
        .style(grin_gui_core::theme::ContainerStyle::BrightBackground)
        .padding(iced::Padding::from([
            2, // top
            0, // right
            2, // bottom
            5, // left
        ]));

    // push more items on to header here: e.g. other buttons, things that belong on the header
    let header_row = Row::new().push(title_container);

    let header_container = Container::new(header_row).padding(iced::Padding::from([
        0,               // top
        0,               // right
        DEFAULT_PADDING, // bottom
        0,               // left
    ]));

    let description = Text::new(state.desc.clone())
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);
    let description_container =
        Container::new(description).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let card_contents = match &state.encrypted_slate {
        Some(s) => s.to_owned(),
        None => "".to_owned()
    };

    let encrypted_slate_card = Card::new(
        Text::new(localized_string("tx-paste-success-title"))
            .size(DEFAULT_HEADER_FONT_SIZE),
        Text::new(card_contents.clone()).size(DEFAULT_FONT_SIZE),
    )
    .foot(
        Column::new()
            .spacing(10)
            .padding(5)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(
                Button::new(
                    Text::new(localized_string("copy-to-clipboard"))
                        .size(SMALLER_FONT_SIZE)
                        .horizontal_alignment(alignment::Horizontal::Center),
                )
                .style(grin_gui_core::theme::ButtonStyle::NormalText)
                .on_press(Message::Interaction(Interaction::WriteToClipboard(
                    card_contents.clone(),
                ))),
            ),
    )
    .max_width(400.0)
    .style(grin_gui_core::theme::CardStyle::Normal);

    let unit_spacing = 15.0;

    let button_height = Length::Fixed(BUTTON_HEIGHT);
    let button_width = Length::Fixed(BUTTON_WIDTH);

    let cancel_button_label_container =
        Container::new(Text::new(localized_string("ok-caps")).size(DEFAULT_FONT_SIZE))
            .width(button_width)
            .height(button_height)
            .center_x()
            .center_y()
            .align_x(alignment::Horizontal::Center);

    let cancel_button: Element<Interaction> = Button::new(cancel_button_label_container)
        .style(grin_gui_core::theme::ButtonStyle::Primary)
        .on_press(Interaction::WalletOperationShowSlatepackViewInteraction(
            LocalViewInteraction::Submit,
        ))
        .into();

    let cancel_container = Container::new(cancel_button.map(Message::Interaction)).padding(1);
    let cancel_container = Container::new(cancel_container)
        .style(grin_gui_core::theme::ContainerStyle::Segmented)
        .padding(1);

    let unit_spacing = 15.0;
    let button_row = Row::new().push(cancel_container);

    let column = Column::new()
        .push(description_container)
        .push(Space::new(
            Length::Fixed(0.0),
            Length::Fixed(unit_spacing + 5.0),
        ))
        .push(encrypted_slate_card)
        .push(Space::new(
            Length::Fixed(0.0),
            Length::Fixed(unit_spacing + 10.0),
        ))
        .push(button_row)
        .push(Space::new(
            Length::Fixed(0.0),
            Length::Fixed(unit_spacing + 10.0),
        ));

    let form_container = Container::new(column)
        .width(Length::Fill)
        .padding(iced::Padding::from([
            0, // top
            0, // right
            0, // bottom
            5, // left
        ]));

    // form container should be scrollable in tiny windows
    let scrollable = Scrollable::new(form_container)
        .height(Length::Fill)
        .style(grin_gui_core::theme::ScrollableStyle::Primary);

    let content = Container::new(scrollable)
        .width(Length::Fill)
        .height(Length::Shrink)
        .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let wrapper_column = Column::new()
        .height(Length::Fill)
        .push(header_container)
        .push(content);

    // Returns the final container.
    Container::new(wrapper_column).padding(iced::Padding::from([
        DEFAULT_PADDING, // top
        DEFAULT_PADDING, // right
        DEFAULT_PADDING, // bottom
        DEFAULT_PADDING, // left
    ]))
}
