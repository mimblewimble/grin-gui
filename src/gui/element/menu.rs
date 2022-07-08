use isahc::http::version;

use {
    super::{DEFAULT_FONT_SIZE, SMALLER_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::VERSION,
    grin_gui_core::theme::ColorPalette,
    grin_gui_widgets::TableRow,
    iced::{
        alignment, button, Alignment, Button, Column, Command, Container, Element, Length, Row,
        Space, Text,
    },
    serde::{Deserialize, Serialize},
    std::sync::{Arc, RwLock},
};

#[derive(Debug, Clone)]
pub struct StateContainer {
    pub mode: Mode,
    wallet_mode_btn: button::State,
    node_mode_btn: button::State,
    settings_mode_btn: button::State,
    about_mode_btn: button::State,
    error_detail_btn: button::State,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            mode: Mode::Wallet,
            wallet_mode_btn: Default::default(),
            node_mode_btn: Default::default(),
            settings_mode_btn: Default::default(),
            about_mode_btn: Default::default(),
            error_detail_btn: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LocalViewInteraction {
    SelectMode(Mode),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    Wallet,
    Node,
    Settings,
    About,
}

pub fn handle_message(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> crate::Result<Command<Message>> {
    match message {
        LocalViewInteraction::SelectMode(mode) => {
            log::debug!("Interaction::ModeSelectedSettings({:?})", mode);
            // Set Mode
            grin_gui.menu_state.mode = mode
        }
    }
    Ok(Command::none())
}

pub fn data_container<'a>(
    state: &'a mut StateContainer,
    color_palette: ColorPalette,
    error: &mut Option<anyhow::Error>,
) -> Container<'a, Message> {
    let mut settings_row = Row::new()
        .height(Length::Units(50))
        .push(Space::new(Length::Units(DEFAULT_PADDING), Length::Units(0)));

    let mut wallet_mode_button: Button<Interaction> = Button::new(
        &mut state.wallet_mode_btn,
        Text::new(localized_string("wallet")).size(DEFAULT_FONT_SIZE),
    )
    .on_press(Interaction::MenuViewInteraction(
        LocalViewInteraction::SelectMode(Mode::Wallet),
    ));

    let mut node_mode_button: Button<Interaction> = Button::new(
        &mut state.node_mode_btn,
        Text::new(localized_string("node")).size(DEFAULT_FONT_SIZE),
    )
    .on_press(Interaction::MenuViewInteraction(
        LocalViewInteraction::SelectMode(Mode::Node),
    ));

    let mut settings_mode_button: Button<Interaction> = Button::new(
        &mut state.settings_mode_btn,
        Text::new(localized_string("settings"))
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(DEFAULT_FONT_SIZE),
    )
    .on_press(Interaction::MenuViewInteraction(
        LocalViewInteraction::SelectMode(Mode::Settings),
    ));

    let mut about_mode_button: Button<Interaction> = Button::new(
        &mut state.about_mode_btn,
        Text::new(localized_string("about"))
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(DEFAULT_FONT_SIZE),
    )
    .on_press(Interaction::MenuViewInteraction(
        LocalViewInteraction::SelectMode(Mode::About),
    ));

    match state.mode {
        Mode::Wallet => {
            wallet_mode_button =
                wallet_mode_button.style(style::SelectedDefaultButton(color_palette));
            node_mode_button = node_mode_button.style(style::DefaultButton(color_palette));
            about_mode_button = about_mode_button.style(style::DefaultButton(color_palette));
            settings_mode_button = settings_mode_button.style(style::DefaultButton(color_palette));
        }
        Mode::Node => {
            wallet_mode_button = wallet_mode_button.style(style::DefaultButton(color_palette));
            node_mode_button = node_mode_button.style(style::SelectedDefaultButton(color_palette));
            about_mode_button = about_mode_button.style(style::DefaultButton(color_palette));
            settings_mode_button = settings_mode_button.style(style::DefaultButton(color_palette));
        }
        Mode::Settings => {
            wallet_mode_button = wallet_mode_button.style(style::DefaultButton(color_palette));
            node_mode_button = node_mode_button.style(style::DefaultButton(color_palette));
            about_mode_button = about_mode_button.style(style::DefaultButton(color_palette));
            settings_mode_button =
                settings_mode_button.style(style::SelectedDefaultButton(color_palette));
        }
        Mode::About => {
            wallet_mode_button = wallet_mode_button.style(style::DefaultButton(color_palette));
            node_mode_button = node_mode_button.style(style::DefaultButton(color_palette));
            about_mode_button =
                about_mode_button.style(style::SelectedDefaultButton(color_palette));
            settings_mode_button = settings_mode_button.style(style::DefaultButton(color_palette));
        }
        /*Mode::Setup => {
            wallet_mode_button =
                wallet_mode_button.style(style::DisabledDefaultButton(color_palette));
            node_mode_button = node_mode_button.style(style::DisabledDefaultButton(color_palette));
            about_mode_button =
                about_mode_button.style(style::DisabledDefaultButton(color_palette));
            settings_mode_button =
                settings_mode_button.style(style::DisabledDefaultButton(color_palette));
        }*/
    }

    let wallet_mode_button: Element<Interaction> = wallet_mode_button.into();
    let node_mode_button: Element<Interaction> = node_mode_button.into();
    let settings_mode_button: Element<Interaction> = settings_mode_button.into();
    let about_mode_button: Element<Interaction> = about_mode_button.into();

    let segmented_addons_row = Row::new()
        .push(wallet_mode_button.map(Message::Interaction))
        .push(node_mode_button.map(Message::Interaction))
        .spacing(1);

    /*let mut segmented_mode_row = Row::new().push(my_wallet_table_row).spacing(1);

    let segmented_mode_container = Container::new(segmented_mode_row)
        .padding(2)
        .style(style::SegmentedContainer(color_palette));*/

    let segmented_addon_container = Container::new(segmented_addons_row)
        .padding(2)
        .style(style::SegmentedContainer(color_palette));

    // Empty container shown if no error message
    let mut error_column= Column::new();

    if let Some(e) = error {
        // Displays an error + detail button, if any has occured.
        let error_text = Text::new(e.to_string()).size(DEFAULT_FONT_SIZE);

        let error_detail_button: Button<Interaction> = Button::new(
            &mut state.error_detail_btn,
            Text::new(localized_string("more-error-detail"))
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center)
                .size(SMALLER_FONT_SIZE),
        )
        .style(style::NormalTextButton(color_palette))
        .on_press(Interaction::OpenErrorModal);

        let error_detail_button: Element<Interaction> = error_detail_button.into();

        error_column = Column::new()
            .push(Space::new(Length::Units(0), Length::Units(5)))
            .push(error_text)
            .push(error_detail_button.map(Message::Interaction))
            .align_items(Alignment::Center)
    }

    let error_container: Container<Message> = Container::new(error_column)
        .center_y()
        .center_x()
        .width(Length::Fill)
        .style(style::NormalErrorForegroundContainer(color_palette));

    /*let version_text = Text::new(if let Some(release) = &self_update_state.latest_release {
        if VersionCompare::compare_to(&release.tag_name, VERSION, &CompOp::Gt).unwrap_or(false) {
            if is_updatable {
                needs_update = true;
            }

            format!(
                "{} {} -> {}",
                localized_string("new-update-available"),
                VERSION,
                &release.tag_name
            )
        } else {
            VERSION.to_owned()
        }
    } else {
        VERSION.to_owned()
    })*/

    let version_text = Text::new(VERSION.to_owned())
        .size(DEFAULT_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Right);

    let version_container = Container::new(version_text)
        .center_y()
        .padding(5)
        .style(style::NormalForegroundContainer(color_palette));

    // Surrounds the elements with spacers, in order to make the GUI look good.
    settings_row = settings_row
        //.push(segmented_mode_container)
        //.push(Space::new(Length::Units(DEFAULT_PADDING), Length::Units(0)))
        .push(segmented_addon_container)
        .push(Space::new(Length::Units(DEFAULT_PADDING), Length::Units(0)))
        .push(error_container)
        .push(version_container);

    let mut segmented_mode_control_row: Row<Message> = Row::new().spacing(1);
    segmented_mode_control_row =
        segmented_mode_control_row.push(about_mode_button.map(Message::Interaction));
    segmented_mode_control_row =
        segmented_mode_control_row.push(settings_mode_button.map(Message::Interaction));

    let segmented_mode_control_container = Container::new(segmented_mode_control_row)
        .padding(2)
        .style(style::SegmentedContainer(color_palette));

    settings_row = settings_row
        .push(segmented_mode_control_container)
        .push(Space::new(
            Length::Units(DEFAULT_PADDING + 5),
            Length::Units(0),
        ))
        .align_items(Alignment::Center);

    // Add space above settings_row.
    let settings_column = Column::new().push(settings_row);

    // Wraps it in a container.
    Container::new(settings_column).style(style::BrightForegroundContainer(color_palette))
}
