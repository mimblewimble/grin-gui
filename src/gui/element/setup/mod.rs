use {
    super::{DEFAULT_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::{
        config::Config,
        wallet::WalletInterface,
    },
    iced::{
        alignment, button, Alignment, Button, Checkbox, Column, Command, Container, Element, Length, Space,
        Text,
    },
};

pub struct StateContainer {
    pub setup_wallet_defaults_is_selected: bool,
    execute_btn: button::State,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            setup_wallet_defaults_is_selected: true,
            execute_btn: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    ExecuteWalletSetup,
}

pub fn handle_message(
    state: &mut StateContainer,
    config: &mut Config,
    wallet_interface: &mut WalletInterface,
    message: LocalViewInteraction,
    error: &mut Option<anyhow::Error>,
) -> Result<Command<Message>> {
    match message {
        LocalViewInteraction::ExecuteWalletSetup => {
            wallet_interface.init()
        }
    }
    Ok(Command::none())
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    state: &'a mut StateContainer,
) -> Container<'a, Message> {
    let title = Text::new(localized_string("setup-grin-wallet-title"))
        .size(DEFAULT_FONT_SIZE)
        //.width(Length::Fill)
        .horizontal_alignment(alignment::Horizontal::Left);
    let title_container = Container::new(title)
        //.width(Length::Fill)
        .style(style::BrightBackgroundContainer(color_palette));

    let description = Text::new(localized_string("setup-grin-wallet-description"))
        .size(DEFAULT_FONT_SIZE)
        //.width(Length::Fill)
        .horizontal_alignment(alignment::Horizontal::Left);
    let description_container = Container::new(description)
        //.width(Length::Fill)
        .style(style::NormalBackgroundContainer(color_palette));

    //let mut wallet_setup_modal_column =
    let wallet_setup_select_column = {
        let checkbox = Checkbox::new(
            state.setup_wallet_defaults_is_selected,
            localized_string("close-to-tray"),
            Interaction::ToggleCloseToTray,
        )
        .style(style::DefaultCheckbox(color_palette))
        .text_size(DEFAULT_FONT_SIZE)
        .spacing(5);

        let checkbox: Element<Interaction> = checkbox.into();

        let checkbox_container = Container::new(checkbox.map(Message::Interaction))
            .style(style::NormalBackgroundContainer(color_palette));

        Column::new().push(checkbox_container)
    };

    let mut colum = Column::new()
        .push(title_container)
        .push(Space::new(Length::Units(0), Length::Units(2)))
        .push(description_container)
        .push(Space::new(Length::Units(0), Length::Units(2)))
        .push(wallet_setup_select_column);

    let title_container = Container::new(Text::new(localized_string("execute-initial-setup")).size(DEFAULT_FONT_SIZE))
        .center_x()
        .align_x(alignment::Horizontal::Left);
    let button: Element<Interaction> = Button::new(&mut state.execute_btn, title_container)
        .style(style::DefaultButton(color_palette))
        .on_press(Interaction::SetupViewInteraction(
            LocalViewInteraction::ExecuteWalletSetup,
        ))
        .into();

    colum = colum
        .push(Space::new(Length::Units(0), Length::Units(DEFAULT_PADDING)))
        .push(button.map(Message::Interaction))
        .align_items(Alignment::Start);

    Container::new(colum)
        .center_y()
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
}
