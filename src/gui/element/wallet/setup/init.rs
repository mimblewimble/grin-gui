use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE},
    crate::gui::{GrinGui, Interaction, Message, element::settings::wallet},
    crate::localization::localized_string,
    crate::Result,
    grin_gui_core::{
        theme::ColorPalette,
        wallet::{create_grin_wallet_path, ChainTypes},
    },
    grin_gui_core::theme::{Column, Element, Container, PickList, Row, Scrollable, Text, TextInput},
    iced::{alignment, Alignment, Command, Length},
    iced::widget::{
        button, pick_list, scrollable, text_input, Button, Checkbox, Space,
    },
};

pub struct StateContainer {
    pub setup_wallet_defaults_is_selected: bool,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            setup_wallet_defaults_is_selected: true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    WalletSetup,
    WalletList,
}

pub fn handle_message(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let state = &mut grin_gui.wallet_state.setup_state;
    match message {
        LocalViewInteraction::WalletSetup => {
            let config = &grin_gui.config;
            let wallet_default_name = localized_string("wallet-default-name");
            let mut wallet_display_name = wallet_default_name.clone();
            let mut i = 1;

            // wallet display name must be unique: i.e. Default 1, Default 2, ...
            while let Some(_) = config
                .wallets
                .iter()
                .find(|wallet| wallet.display_name == wallet_display_name)
            {
                wallet_display_name = format!("{} {}", wallet_default_name, i);
                i += 1;
            }

            // i.e. default_1, default_2, ...
            let wallet_dir: String = str::replace(&wallet_display_name.to_lowercase(), " ", "_");

            state
                .setup_wallet_state
                .advanced_options_state
                .top_level_directory = create_grin_wallet_path(&ChainTypes::Mainnet, &wallet_dir);

            state.mode = super::Mode::CreateWallet(wallet_display_name);
        }
        LocalViewInteraction::WalletList => state.mode = super::Mode::ListWallets,
    }
    Ok(Command::none())
}

pub fn data_container<'a>() -> Container<'a, Message> {
    // Title row
    let title = Text::new(localized_string("setup-grin-first-time"))
        .size(DEFAULT_HEADER_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);

    let title_container =
        Container::new(title).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let title_row = Row::new()
        .push(title_container)
        .align_items(Alignment::Center)
        .padding(6)
        .spacing(20);

    let description = Text::new(localized_string("setup-grin-wallet-description"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);
    let description_container =
        Container::new(description).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let or_text = Text::new(localized_string("or-caps"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);

    let or_text_container =
        Container::new(or_text).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let create_default_wallet_button_label_container = Container::new(
        Text::new(localized_string("setup-grin-autogenerate-wallet")).size(DEFAULT_FONT_SIZE),
    )
    .center_x()
    .align_x(alignment::Horizontal::Center);

    let create_default_wallet_button: Element<Interaction> = Button::new(
        create_default_wallet_button_label_container,
    )
    .style(grin_gui_core::theme::ButtonStyle::Bordered)
    .on_press(Interaction::WalletSetupInitViewInteraction(
        LocalViewInteraction::WalletSetup,
    ))
    .into();

    let select_wallet_button_label_container =
        Container::new(Text::new(localized_string("select-wallet-toml")).size(DEFAULT_FONT_SIZE))
            .center_x()
            .align_x(alignment::Horizontal::Center);

    let select_wallet_button: Element<Interaction> = Button::new(
        select_wallet_button_label_container,
    )
    .style(grin_gui_core::theme::ButtonStyle::Bordered)
    .on_press(Interaction::WalletSetupInitViewInteraction(
        LocalViewInteraction::WalletList,
    ))
    .into();

    let select_wallet_button_container =
        Container::new(select_wallet_button.map(Message::Interaction)).center_x();

    //let mut wallet_setup_modal_column =
    /*let wallet_setup_select_column = {
        let checkbox = Checkbox::new(
            state.setup_wallet_defaults_is_selected,
            localized_string("setup-grin-autogenerate-wallet"),
            Interaction::ToggleCloseToTray,
        )
        .style(grin_gui_core::theme::CheckboxStyles::Normal)
        .text_size(DEFAULT_FONT_SIZE)
        .spacing(5);

        let checkbox: Element<Interaction> = checkbox.into();

        let checkbox_container = Container::new(checkbox.map(Message::Interaction))
            .style(grin_gui_core::theme::container::Container::NormalBackground);

        Column::new().push(checkbox_container)
    };*/

    let unit_spacing = 15;

    let select_column = Column::new()
        .push(create_default_wallet_button.map(Message::Interaction))
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
        .push(or_text_container)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
        .push(select_wallet_button_container)
        .align_items(Alignment::Center);

    let column = Column::new()
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(20)))
        .push(title_row)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
        .push(description_container)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)))
        .push(select_column)
        .align_items(Alignment::Center);

    Container::new(column)
        .center_y()
        .center_x()
        .width(Length::Fill)
}
