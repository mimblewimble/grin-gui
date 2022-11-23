use {
    super::{DEFAULT_FONT_SIZE, DEFAULT_PADDING, SMALLER_FONT_SIZE},
    crate::gui::{GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::VERSION,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::theme::{Button, Column, Element, Scrollable, Text, Row, Container, PickList},
    iced::{alignment, Alignment, Command, Length},
    iced::widget::{
        button, pick_list, scrollable, text_input, Checkbox, Space, TextInput,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone)]
pub struct StateContainer {
    pub mode: Mode,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            mode: Mode::Wallet,
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
    state: &'a StateContainer,
    color_palette: ColorPalette,
    error: &Option<anyhow::Error>,
) -> Container<'a, Message> {
    let mut wallet_mode_button: Button<Interaction> = Button::new(
        Text::new(localized_string("wallet")).size(DEFAULT_FONT_SIZE),
    )
    .on_press(Interaction::MenuViewInteraction(
        LocalViewInteraction::SelectMode(Mode::Wallet),
    ));

    let mut node_mode_button: Button<Interaction> = Button::new(
        Text::new(localized_string("node")).size(DEFAULT_FONT_SIZE),
    )
    .on_press(Interaction::MenuViewInteraction(
        LocalViewInteraction::SelectMode(Mode::Node),
    ));

    let mut settings_mode_button: Button<Interaction> = Button::new(
        Text::new(localized_string("settings"))
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(DEFAULT_FONT_SIZE),
    )
    .on_press(Interaction::MenuViewInteraction(
        LocalViewInteraction::SelectMode(Mode::Settings),
    ));

    let mut about_mode_button: Button<Interaction> = Button::new(
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
                wallet_mode_button.style(grin_gui_core::theme::button::Button::Selected(color_palette));
            node_mode_button = node_mode_button.style(grin_gui_core::theme::button::Button::Primary(color_palette));
            about_mode_button = about_mode_button.style(grin_gui_core::theme::button::Button::Primary(color_palette));
            settings_mode_button = settings_mode_button.style(grin_gui_core::theme::button::Button::Primary(color_palette));
        }
        Mode::Node => {
            wallet_mode_button = wallet_mode_button.style(grin_gui_core::theme::button::Button::Primary(color_palette));
            node_mode_button = node_mode_button.style(grin_gui_core::theme::button::Button::Selected(color_palette));
            about_mode_button = about_mode_button.style(grin_gui_core::theme::button::Button::Primary(color_palette));
            settings_mode_button = settings_mode_button.style(grin_gui_core::theme::button::Button::Primary(color_palette));
        }
        Mode::Settings => {
            wallet_mode_button = wallet_mode_button.style(grin_gui_core::theme::button::Button::Primary(color_palette));
            node_mode_button = node_mode_button.style(grin_gui_core::theme::button::Button::Primary(color_palette));
            about_mode_button = about_mode_button.style(grin_gui_core::theme::button::Button::Primary(color_palette));
            settings_mode_button =
                settings_mode_button.style(grin_gui_core::theme::button::Button::Selected(color_palette));
        }
        Mode::About => {
            wallet_mode_button = wallet_mode_button.style(grin_gui_core::theme::button::Button::Primary(color_palette));
            node_mode_button = node_mode_button.style(grin_gui_core::theme::button::Button::Primary(color_palette));
            about_mode_button =
                about_mode_button.style(grin_gui_core::theme::button::Button::Selected(color_palette));
            settings_mode_button = settings_mode_button.style(grin_gui_core::theme::button::Button::Primary(color_palette));
        } /*Mode::Setup => {
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

    let segmented_addons_row = Row::with_children(vec![
        wallet_mode_button.map(Message::Interaction),
        node_mode_button.map(Message::Interaction),
    ])
    .spacing(1);

    /*let mut segmented_mode_row = Row::new().push(my_wallet_table_row).spacing(1);
    let segmented_mode_container = Container::new(segmented_mode_row)
        .padding(2)
        .style(grin_gui_core::theme::container::Container::Segmented(color_palette));*/

    let segmented_addon_container = Container::new(segmented_addons_row)
        .padding(2)
        .style(grin_gui_core::theme::container::Container::Segmented(color_palette));

    // Empty container shown if no error message
    let mut error_column = Column::new();

    if let Some(e) = error {
        // Displays an error + detail button, if any has occured.
        let error_text = Text::new(e.to_string()).size(DEFAULT_FONT_SIZE);

        let error_detail_button: Button<Interaction> = Button::new(
            Text::new(localized_string("more-error-detail"))
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center)
                .size(SMALLER_FONT_SIZE),
        )
        .style(grin_gui_core::theme::button::Button::NormalText(color_palette))
        .on_press(Interaction::OpenErrorModal);

        let error_detail_button: Element<Interaction> = error_detail_button.into();

        error_column = Column::with_children(vec![
            Space::with_height(Length::Units(5)).into(),
            error_text.into(),
            error_detail_button.map(Message::Interaction),
        ])
        .align_items(Alignment::Center);
    }

    let error_container: Container<Message> = Container::new(error_column)
        .center_y()
        .center_x()
        .width(Length::Fill)
        .style(grin_gui_core::theme::container::Container::ErrorForeground(color_palette));

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
        .style(grin_gui_core::theme::container::Container::NormalBackground(color_palette));

    let segmented_mode_control_row: Row<Message> = Row::with_children(vec![
        about_mode_button.map(Message::Interaction),
        settings_mode_button.map(Message::Interaction),
    ])
    .spacing(1);

    let segmented_mode_control_container = Container::new(segmented_mode_control_row)
        .padding(2)
        .style(grin_gui_core::theme::container::Container::Segmented(color_palette));

    let settings_row = Row::with_children(vec![
        segmented_addon_container.into(),
        Space::with_width(Length::Units(DEFAULT_PADDING)).into(),
        error_container.into(),
        version_container.into(),
        segmented_mode_control_container.into(),
    ])
    .align_items(Alignment::Center);

    // Wraps it in a container with even padding on all sides
    Container::new(settings_row)
        .style(grin_gui_core::theme::container::Container::BrightForeground(color_palette))
        .padding(iced::Padding::from([
            DEFAULT_PADDING, // top
            DEFAULT_PADDING, // right
            DEFAULT_PADDING, // bottom
            DEFAULT_PADDING, // left
        ]))
}
