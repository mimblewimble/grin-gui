use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE},
    crate::gui::{style, GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::{config::Config, wallet::WalletInterface},
    iced::{
        alignment, button, Alignment, Button, Column, Command, Container, Element, Length, Row,
        Space, Text,
    },
};
#[derive(Debug)]
pub struct StateContainer {
    pub setup_wallet_defaults_is_selected: bool,
    create_default_wallet_btn: button::State,
    restore_from_seed_btn: button::State,
    select_wallet_toml_btn: button::State,
    execute_btn: button::State,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            setup_wallet_defaults_is_selected: true,
            create_default_wallet_btn: Default::default(),
            restore_from_seed_btn: Default::default(),
            select_wallet_toml_btn: Default::default(),
            execute_btn: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    WalletSetupCreate,
    WalletSetupRestore,
    WalletList,
}

pub fn handle_message(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let state = &mut grin_gui.wallet_state.setup_state;
    match message {
        LocalViewInteraction::WalletSetupCreate => {
            state.setup_wallet_state.restore_from_seed = false;
            state.mode = super::Mode::CreateWallet
        }
        LocalViewInteraction::WalletSetupRestore => {
            state.setup_wallet_state.restore_from_seed = true;
            state.mode = super::Mode::RestoreWallet
        }
        LocalViewInteraction::WalletList => state.mode = super::Mode::ListWallets
    }
    Ok(Command::none())
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    state: &'a mut StateContainer,
) -> Container<'a, Message> {

    // Title row
    let title = Text::new(localized_string("setup-grin-first-time"))
        .size(DEFAULT_HEADER_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);

    let title_container = Container::new(title)
        .style(style::BrightBackgroundContainer(color_palette));

    let title_row = Row::new()
        .push(title_container)
        .align_items(Alignment::Center)
        .padding(6)
        .spacing(20);
        

    let description = Text::new(localized_string("setup-grin-wallet-description"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Left);
    let description_container =
        Container::new(description).style(style::NormalBackgroundContainer(color_palette));
        
    let or_text = Text::new(localized_string("or-caps"))
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);

    let or_text_container =
        Container::new(or_text).style(style::NormalBackgroundContainer(color_palette));

    /*  Create new wallet */

    let create_default_wallet_button_label_container = Container::new(
        Text::new(localized_string("setup-grin-autogenerate-wallet")).size(18),
    )
    .center_x()
    .align_x(alignment::Horizontal::Center);

    let create_default_wallet_button: Element<Interaction> = Button::new(
        &mut state.create_default_wallet_btn,
        create_default_wallet_button_label_container,
    )
    .style(style::DefaultBoxedButton(color_palette))
    .on_press(Interaction::WalletSetupInitViewInteraction(
        LocalViewInteraction::WalletSetupCreate,
    ))
    .into();

    /*  Restore from seed */

    let restore_from_seed_button_label_container = Container::new(
        Text::new(localized_string("restore-from-seed")).size(DEFAULT_FONT_SIZE),
    )
    .center_x()
    .align_x(alignment::Horizontal::Center);

    let restore_from_seed_button: Element<Interaction> = Button::new(
        &mut state.restore_from_seed_btn,
        restore_from_seed_button_label_container,
    )
    .style(style::DefaultBoxedButton(color_palette))
    .on_press(Interaction::WalletSetupInitViewInteraction(
        LocalViewInteraction::WalletSetupRestore,
    ))
    .into();

    /*  Select from file */

    let select_wallet_button_label_container =
        Container::new(Text::new(localized_string("select-wallet-toml")).size(DEFAULT_FONT_SIZE))
            .center_x()
            .align_x(alignment::Horizontal::Center);

    let select_wallet_button: Element<Interaction> = Button::new(
        &mut state.select_wallet_toml_btn,
        select_wallet_button_label_container,
    )
    .style(style::DefaultBoxedButton(color_palette))
    .on_press(Interaction::WalletSetupInitViewInteraction(
        LocalViewInteraction::WalletList,
    ))
    .into();

    let select_wallet_button_container =
        Container::new(select_wallet_button.map(Message::Interaction))
    .center_x();

    //let mut wallet_setup_modal_column =
    /*let wallet_setup_select_column = {
        let checkbox = Checkbox::new(
            state.setup_wallet_defaults_is_selected,
            localized_string("setup-grin-autogenerate-wallet"),
            Interaction::ToggleCloseToTray,
        )
        .style(style::DefaultCheckbox(color_palette))
        .text_size(DEFAULT_FONT_SIZE)
        .spacing(5);

        let checkbox: Element<Interaction> = checkbox.into();

        let checkbox_container = Container::new(checkbox.map(Message::Interaction))
            .style(style::NormalBackgroundContainer(color_palette));

        Column::new().push(checkbox_container)
    };*/

    let unit_spacing = 20;

    let select_row = Row::new()
        .push(restore_from_seed_button.map(Message::Interaction))
        .push(Space::new(Length::Units(unit_spacing), Length::Units(0)))
        .push(select_wallet_button_container)
        .align_items(Alignment::Center);
 
    let colum = Column::new()
        .push(title_row)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(description_container)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(create_default_wallet_button.map(Message::Interaction))
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(or_text_container)
        .push(Space::new(Length::Units(0), Length::Units(unit_spacing)))
        .push(select_row)
       .align_items(Alignment::Center);

       

    Container::new(colum)
        .center_y()
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
}