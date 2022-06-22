use isahc::http::version;

use {
    super::{DEFAULT_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, Interaction, Message},
    crate::localization::localized_string,
    crate::VERSION,
    ajour_core::theme::ColorPalette,
    ajour_widgets::TableRow,
    iced::{
        alignment, button, Alignment, Button, Column, Command, Container, Element, Length, Row,
        Space, Text,
    },
    serde::{Deserialize, Serialize},
    serde_json,
    std::sync::{Arc, RwLock},
    uuid::Uuid,
};

#[derive(Debug, Clone)]
pub struct StateContainer {
    pub mode: Mode,
    catalog_mode_btn: button::State,
    install_mode_btn: button::State,
    settings_mode_btn: button::State,
    about_mode_btn: button::State,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            mode: Mode::Catalog,
            catalog_mode_btn: Default::default(),
            install_mode_btn: Default::default(),
            settings_mode_btn: Default::default(),
            about_mode_btn: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LocalViewInteraction {
    SelectMode(Mode),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    Catalog,
    Install,
    Settings,
    About,
}

pub fn handle_message(
    state: &mut StateContainer,
    message: LocalViewInteraction,
) -> crate::Result<Command<Message>> {
    match message {
        LocalViewInteraction::SelectMode(mode) => {
            log::debug!("Interaction::ModeSelectedSettings({:?})", mode);
            // Set Mode
            state.mode = mode
        }
    }
    Ok(Command::none())
}

pub fn data_container<'a>(
    state: &'a mut StateContainer,
    color_palette: ColorPalette,
) -> Container<'a, Message> {
    /*let flavor = config.wow.flavor;
    let mut valid_flavors = config
        .wow
        .directories
        .keys()
        .copied()
        .collect::<Vec<Flavor>>();

    valid_flavors.sort();

    // State.
    let myaddons_state = state.get(&Mode::MyAddons(flavor));*/
    // A row contain general settings.
    let mut settings_row = Row::new()
        .height(Length::Units(50))
        .push(Space::new(Length::Units(DEFAULT_PADDING), Length::Units(0)));

    let mut my_wallet_table_row = {
        let title_container = Container::new(
            Text::new(localized_string("my-addons"))
                .horizontal_alignment(alignment::Horizontal::Center)
                .size(DEFAULT_FONT_SIZE),
        )
        .style(style::HoverableSegmentContainer(color_palette));
        /*let text = {
            match updatable_addons {
                0..=9 => format!("{}", updatable_addons),
                _ => "9+".to_owned(),
            }
        };
        let notification_row = Row::new()
            .push(Space::new(Length::Units(7), Length::Units(0)))
            .push(
                Text::new(text)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(10),
            )
            .push(Space::new(Length::Units(7), Length::Units(0)));
        let notification_container = Container::new(notification_row)
            .padding(3)
            .style(style::HoverableSegmentAlternateContainer(color_palette));*/
        let mut row = Row::new()
            .height(Length::Units(24))
            .align_items(Alignment::Center)
            .push(Space::new(Length::Units(6), Length::Units(1)))
            .push(title_container)
            .push(Space::new(Length::Units(6), Length::Units(1)));

        // Only display the notification container if we have any updatable addons.
        /*if updatable_addons > 0 {
            row = row
                .push(notification_container)
                .push(Space::new(Length::Units(6), Length::Units(1)));
        }*/

        TableRow::new(row).inner_row_height(24).on_press(move |_| {
            Message::Interaction(Interaction::MenuViewInteraction(
                LocalViewInteraction::SelectMode(Mode::Catalog),
            ))
        })
    };

    let mut catalog_mode_button: Button<Interaction> = Button::new(
        &mut state.catalog_mode_btn,
        Text::new(localized_string("catalog")).size(DEFAULT_FONT_SIZE),
    )
    .style(style::DisabledDefaultButton(color_palette));

    let mut install_mode_button: Button<Interaction> = Button::new(
        &mut state.install_mode_btn,
        Text::new(localized_string("install-from-url")).size(DEFAULT_FONT_SIZE),
    )
    .style(style::DisabledDefaultButton(color_palette));

    let mut settings_mode_button: Button<Interaction> = Button::new(
        &mut state.settings_mode_btn,
        Text::new(localized_string("settings"))
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(DEFAULT_FONT_SIZE),
    )
    .on_press(Interaction::MenuViewInteraction(LocalViewInteraction::SelectMode(Mode::Settings)));

    let mut about_mode_button: Button<Interaction> = Button::new(
        &mut state.about_mode_btn,
        Text::new(localized_string("about"))
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(DEFAULT_FONT_SIZE),
    )
    .on_press(Interaction::MenuViewInteraction(LocalViewInteraction::SelectMode(Mode::About)));

    match state.mode {
        Mode::Install => {
            catalog_mode_button = catalog_mode_button.style(style::DefaultButton(color_palette));
            install_mode_button =
                install_mode_button.style(style::SelectedDefaultButton(color_palette));
            about_mode_button = about_mode_button.style(style::DefaultButton(color_palette));
            settings_mode_button = settings_mode_button.style(style::DefaultButton(color_palette));
        }
        Mode::Catalog => {
            catalog_mode_button =
                catalog_mode_button.style(style::SelectedDefaultButton(color_palette));
            install_mode_button = install_mode_button.style(style::DefaultButton(color_palette));
            about_mode_button = about_mode_button.style(style::DefaultButton(color_palette));
            settings_mode_button = settings_mode_button.style(style::DefaultButton(color_palette));
        }
        Mode::Settings => {
            catalog_mode_button = catalog_mode_button.style(style::DefaultButton(color_palette));
            install_mode_button = install_mode_button.style(style::DefaultButton(color_palette));
            about_mode_button = about_mode_button.style(style::DefaultButton(color_palette));
            settings_mode_button =
                settings_mode_button.style(style::SelectedDefaultButton(color_palette));
        }
        Mode::About => {
            catalog_mode_button = catalog_mode_button.style(style::DefaultButton(color_palette));
            install_mode_button = install_mode_button.style(style::DefaultButton(color_palette));
            about_mode_button =
                about_mode_button.style(style::SelectedDefaultButton(color_palette));
            settings_mode_button = settings_mode_button.style(style::DefaultButton(color_palette));
        }
    }

    let catalog_mode_button: Element<Interaction> = catalog_mode_button.into();
    let install_mode_button: Element<Interaction> = install_mode_button.into();
    let settings_mode_button: Element<Interaction> = settings_mode_button.into();
    let about_mode_button: Element<Interaction> = about_mode_button.into();

    let segmented_addons_row = Row::new()
        .push(catalog_mode_button.map(Message::Interaction))
        .push(install_mode_button.map(Message::Interaction))
        .spacing(1);

    let mut segmented_mode_row = Row::new().push(my_wallet_table_row).spacing(1);

    let segmented_mode_container = Container::new(segmented_mode_row)
        .padding(2)
        .style(style::SegmentedContainer(color_palette));

    let segmented_addon_container = Container::new(segmented_addons_row)
        .padding(2)
        .style(style::SegmentedContainer(color_palette));

    // Displays an error, if any has occured.
    /*let error_text = if let Some(error) = error {
        Text::new(error.to_string()).size(DEFAULT_FONT_SIZE)
    } else {
        // Display nothing.
        Text::new("")
    };*/
    let error_text = Text::new("");

    let error_container: Container<Message> = Container::new(error_text)
        .center_y()
        .center_x()
        .padding(5)
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
        .push(segmented_mode_container)
        .push(Space::new(Length::Units(DEFAULT_PADDING), Length::Units(0)))
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
