use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, SMALLER_FONT_SIZE},
    crate::gui::{GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    grin_gui_core::theme::ColorPalette,
    iced::{alignment, Alignment, Command, Length},
    grin_gui_core::theme::{Column, Element, Container, PickList, Row, Scrollable, Text, TextInput},
    iced::widget::{
        button, pick_list, scrollable, text_input, Button, Checkbox, Space, 
    },
    iced_aw::Card,
};

pub struct StateContainer {
    // TODO: ZeroingString this
    pub recovery_phrase: String,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            recovery_phrase: Default::default(),
        }
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
    let state = &mut grin_gui.wallet_state.setup_state.setup_wallet_state;
    match message {
        LocalViewInteraction::Submit => {
            grin_gui.wallet_state.mode = super::super::Mode::Operation;
            grin_gui.wallet_state.setup_state.mode = crate::gui::element::wallet::setup::Mode::Init;
        }
    }
    Ok(Command::none())
}

pub fn data_container<'a>(
    state: &'a StateContainer,
) -> Container<'a, Message> {
    // Title row
    let title = Text::new(localized_string("setup-grin-wallet-success"))
        .size(DEFAULT_HEADER_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);

    let title_container =
        Container::new(title).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let title_row = Row::new()
        .push(title_container)
        .align_items(Alignment::Center)
        .spacing(20);

    let description = Text::new(localized_string("setup-grin-wallet-recovery-phrase"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);
    let description_container =
        Container::new(description).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let recovery_phrase_card = Card::new(
        Text::new(localized_string("setup-grin-wallet-recovery-phrase-title")).size(DEFAULT_HEADER_FONT_SIZE),
        Text::new(&state.recovery_phrase).size(DEFAULT_FONT_SIZE),
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
                    state.recovery_phrase.clone(),
                ))),
            ),
    )
    .max_width(400)
    .style(grin_gui_core::theme::CardStyle::Normal);

    let submit_button_label_container =
        Container::new(Text::new(localized_string("setup-grin-wallet-done")).size(DEFAULT_FONT_SIZE))
            .center_x()
            .align_x(alignment::Horizontal::Center);

    let next_button = Button::new(submit_button_label_container)
        .style(grin_gui_core::theme::ButtonStyle::Bordered)
        .on_press(Interaction::WalletSetupWalletSuccessViewInteraction(
            LocalViewInteraction::Submit,
        ));

    let next_button: Element<Interaction> = next_button.into();

    let unit_spacing = 15;

    let colum = Column::new()
        .push(title_row)
        .push(Space::new(
            Length::Fixed(0.0),
            Length::Fixed(unit_spacing + 5),
        ))
        .push(description_container)
        .push(Space::new(
            Length::Fixed(0.0),
            Length::Fixed(unit_spacing + 5),
        ))
        .push(recovery_phrase_card)
        .push(Space::new(
            Length::Fixed(0.0),
            Length::Fixed(unit_spacing + 10),
        ))
        .push(next_button.map(Message::Interaction))
        .align_items(Alignment::Center);

    Container::new(colum)
        .center_y()
        .center_x()
        .width(Length::Fill)
}
