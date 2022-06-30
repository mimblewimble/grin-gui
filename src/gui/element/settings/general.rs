use {
    super::{DEFAULT_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, Interaction, Message},
    crate::localization::{localized_string, LANG},
    crate::{Result, log_error},
    anyhow::Context,
    grin_gui_core::{
        config::{Config, Language},
        error::ThemeError,
        fs::{import_theme, PersistentData},
        theme::{ColorPalette, Theme},
    },
    iced::{
        button, pick_list, scrollable, text_input, Alignment, Button, Checkbox, Column, Command,
        Container, Element, Length, PickList, Row, Scrollable, Space, Text, TextInput,
    },
    std::sync::Arc,
};

#[derive(Debug, Clone)]
pub struct StateContainer {
    pub theme_state: ThemeState,
    pub scale_state: ScaleState,
    scrollable_state: scrollable::State,
    localization_picklist_state: pick_list::State<Language>,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            theme_state: Default::default(),
            scrollable_state: Default::default(),
            scale_state: Default::default(),
            localization_picklist_state: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThemeState {
    pub themes: Vec<(String, Theme)>,
    pub current_theme_name: String,
    pick_list_state: pick_list::State<String>,
    input_state: text_input::State,
    input_url: String,
    import_button_state: button::State,
    open_builder_button_state: button::State,
}

impl Default for ThemeState {
    fn default() -> Self {
        let themes = Theme::all();

        ThemeState {
            themes,
            current_theme_name: "Dark".to_string(),
            pick_list_state: Default::default(),
            input_state: Default::default(),
            input_url: Default::default(),
            import_button_state: Default::default(),
            open_builder_button_state: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScaleState {
    pub scale: f64,
    up_btn_state: button::State,
    down_btn_state: button::State,
}

impl Default for ScaleState {
    fn default() -> Self {
        ScaleState {
            scale: 1.0,
            up_btn_state: Default::default(),
            down_btn_state: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    ThemeSelected(String),
    LanguageSelected(Language),
    ScaleUp,
    ScaleDown,
    ThemeUrlInput(String),
    ImportTheme,
    ThemeImportedOk((String, Vec<Theme>)),
    ThemeImportedError(Arc<anyhow::Error>),
}

#[derive(Debug, Clone)]
pub enum Mode {
    Wallet,
    Node,
    General,
}

pub fn handle_message(
    state: &mut StateContainer,
    config: &mut Config,
    message: LocalViewInteraction,
    error: &mut Option<anyhow::Error>,
) -> Result<Command<Message>> {
    match message {
        LocalViewInteraction::ThemeSelected(theme_name) => {
            log::debug!(
                "settings::general::LocalViewInteraction::ThemeSelected({:?})",
                &theme_name
            );

            state.theme_state.current_theme_name = theme_name.clone();

            config.theme = Some(theme_name);
            let _ = config.save();
        }
        LocalViewInteraction::LanguageSelected(lang) => {
            log::debug!(
                "settings::general::LocalViewInteraction::LanguageSelected({:?})",
                &lang
            );
            // Update config.
            config.language = lang;
            let _ = config.save();

            // Update global LANG refcell.
            *LANG.get().expect("LANG not set").write().unwrap() = lang.language_code();
        }
        LocalViewInteraction::ScaleUp => {
            let prev_scale = state.scale_state.scale;

            state.scale_state.scale = ((prev_scale + 0.1).min(2.0) * 10.0).round() / 10.0;

            config.scale = Some(state.scale_state.scale);
            let _ = config.save();

            log::debug!(
                "settings::general::LocalViewInteraction::ScaleUp({} -> {})",
                prev_scale,
                state.scale_state.scale
            );
        }
        LocalViewInteraction::ScaleDown => {
            let prev_scale = state.scale_state.scale;

            state.scale_state.scale = ((prev_scale - 0.1).max(0.5) * 10.0).round() / 10.0;

            config.scale = Some(state.scale_state.scale);
            let _ = config.save();

            log::debug!(
                "settings::general::LocalViewInteraction::ScaleDown({} -> {})",
                prev_scale,
                state.scale_state.scale
            );
        }
        LocalViewInteraction::ThemeUrlInput(url) => {
            state.theme_state.input_url = url;
        }
        LocalViewInteraction::ImportTheme => {
            // Reset error
            error.take();

            let url = state.theme_state.input_url.clone();

            log::debug!("Interaction::ImportTheme({})", &url);

            return Ok(Command::perform(import_theme(url), |r| {
                match r.context("Failed to Import Theme") {
                    Ok(result) => {
                        Message::Interaction(Interaction::GeneralSettingsViewInteraction(
                            LocalViewInteraction::ThemeImportedOk(result),
                        ))
                    }
                    Err(mut e) => {
                        for cause in e.chain() {
                            if let Some(theme_error) = cause.downcast_ref::<ThemeError>() {
                                if matches!(theme_error, ThemeError::NameCollision { .. }) {
                                    e = e.context(localized_string(
                                        "import-theme-error-name-collision",
                                    ));
                                    break;
                                }
                            }
                        }
                        log_error(&e);
                        //*error = Some(e);

                        Message::Interaction(Interaction::GeneralSettingsViewInteraction(
                            LocalViewInteraction::ThemeImportedError(Arc::new(e)),
                        ))

                    }
                }
            }));
        }
        LocalViewInteraction::ThemeImportedOk((new_theme_name, mut new_themes)) => {
            log::debug!("Message::ThemeImported({})", &new_theme_name);

            state.theme_state = Default::default();

            new_themes.sort();

            for theme in new_themes {
                state.theme_state.themes.push((theme.name.clone(), theme));
            }

            state.theme_state.current_theme_name = new_theme_name.clone();
            config.theme = Some(new_theme_name);
            let _ = config.save();
        }
        LocalViewInteraction::ThemeImportedError(err) => {
            for cause in err.chain() {
                if let Some(theme_error) = cause.downcast_ref::<ThemeError>() {
                    if matches!(theme_error, ThemeError::NameCollision { .. }) {
                        *err = err.context(localized_string(
                            "import-theme-error-name-collision",
                        ));
                        break;
                    }
                }
            }
            log_error(&err);
            //*error = Some(*err);
            // Reset text input
            state.theme_state.input_url = Default::default();
            state.theme_state.input_state = Default::default();
        }
    }
    Ok(Command::none())
}

pub fn data_container<'a>(
    state: &'a mut StateContainer,
    config: &mut Config,
    color_palette: ColorPalette,
) -> Container<'a, Message> {
    let mut scrollable = Scrollable::new(&mut state.scrollable_state)
        .spacing(1)
        .height(Length::FillPortion(1))
        .style(style::Scrollable(color_palette));

    let language_container = {
        let title = Container::new(Text::new(localized_string("language")).size(DEFAULT_FONT_SIZE))
            .style(style::NormalBackgroundContainer(color_palette));
        let pick_list = PickList::new(
            &mut state.localization_picklist_state,
            &Language::ALL[..],
            Some(config.language),
            |l| {
                Message::Interaction(Interaction::GeneralSettingsViewInteraction(
                    LocalViewInteraction::LanguageSelected(l),
                ))
            },
        )
        .text_size(14)
        .width(Length::Units(120))
        .style(style::PickList(color_palette));
        let container = Container::new(pick_list)
            .center_y()
            .width(Length::Units(120))
            .style(style::NormalForegroundContainer(color_palette));

        Column::new()
            .push(title)
            .push(Space::new(Length::Units(0), Length::Units(5)))
            .push(container)
    };

    let theme_column = {
        let title_container =
            Container::new(Text::new(localized_string("theme")).size(DEFAULT_FONT_SIZE))
                .style(style::NormalBackgroundContainer(color_palette));

        let theme_names = state
            .theme_state
            .themes
            .iter()
            .cloned()
            .map(|(name, _)| name)
            .collect::<Vec<_>>();
        let theme_pick_list = PickList::new(
            &mut state.theme_state.pick_list_state,
            theme_names,
            Some(state.theme_state.current_theme_name.clone()),
            |t| {
                Message::Interaction(Interaction::GeneralSettingsViewInteraction(
                    LocalViewInteraction::ThemeSelected(t),
                ))
            },
        )
        .text_size(DEFAULT_FONT_SIZE)
        .width(Length::Units(120))
        .style(style::PickList(color_palette));

        // Data row for theme picker list.
        let theme_data_row = Row::new()
            .push(theme_pick_list)
            .align_items(Alignment::Center)
            .height(Length::Units(26));

        Column::new()
            .push(title_container)
            .push(Space::new(Length::Units(0), Length::Units(5)))
            .push(theme_data_row)
    };

    // Scale buttons for application scale factoring.
    let scale_column = {
        let title_container =
            Container::new(Text::new(localized_string("scale")).size(DEFAULT_FONT_SIZE))
                .style(style::NormalBackgroundContainer(color_palette));
        let scale_title_row = Row::new().push(title_container);

        let scale_down_button: Element<Interaction> = Button::new(
            &mut state.scale_state.down_btn_state,
            Text::new("  -  ").size(DEFAULT_FONT_SIZE),
        )
        .style(style::DefaultBoxedButton(color_palette))
        .on_press(Interaction::GeneralSettingsViewInteraction(
            LocalViewInteraction::ScaleDown,
        ))
        .into();

        let scale_up_button: Element<Interaction> = Button::new(
            &mut state.scale_state.up_btn_state,
            Text::new("  +  ").size(DEFAULT_FONT_SIZE),
        )
        .style(style::DefaultBoxedButton(color_palette))
        .on_press(Interaction::GeneralSettingsViewInteraction(
            LocalViewInteraction::ScaleUp,
        ))
        .into();

        let current_scale_text = Text::new(format!("  {:.2}  ", state.scale_state.scale))
            .size(DEFAULT_FONT_SIZE)
            .vertical_alignment(iced_native::alignment::Vertical::Center);
        let current_scale_container = Container::new(current_scale_text)
            .height(Length::Fill)
            .center_y()
            .style(style::BrightBackgroundContainer(color_palette));

        let scale_buttons_row = Row::new()
            .push(scale_down_button.map(Message::Interaction))
            .push(current_scale_container)
            .push(scale_up_button.map(Message::Interaction))
            .align_items(Alignment::Center)
            .height(Length::Units(26));

        Column::new()
            .push(scale_title_row)
            .push(Space::new(Length::Units(0), Length::Units(5)))
            .push(scale_buttons_row)
    };

    let import_theme_column = {
        let title_container =
            Container::new(Text::new(localized_string("import-theme")).size(DEFAULT_FONT_SIZE))
                .style(style::NormalBackgroundContainer(color_palette));

        let theme_input = TextInput::new(
            &mut state.theme_state.input_state,
            &localized_string("paste-url")[..],
            &state.theme_state.input_url,
            |s| Interaction::GeneralSettingsViewInteraction(LocalViewInteraction::ThemeUrlInput(s)),
        )
        .size(DEFAULT_FONT_SIZE)
        .padding(6)
        .width(Length::Units(185))
        .style(style::AddonsQueryInput(color_palette));

        let theme_input: Element<Interaction> = theme_input.into();

        let mut import_button = Button::new(
            &mut state.theme_state.import_button_state,
            Text::new(localized_string("import-theme-button")).size(DEFAULT_FONT_SIZE),
        )
        .style(style::DefaultBoxedButton(color_palette));

        if !state.theme_state.input_url.is_empty() {
            import_button = import_button.on_press(Interaction::GeneralSettingsViewInteraction(
                LocalViewInteraction::ImportTheme,
            ));
        }

        let import_button: Element<Interaction> = import_button.into();

        let theme_input_row = Row::new()
            .push(theme_input.map(Message::Interaction))
            .push(import_button.map(Message::Interaction))
            .spacing(DEFAULT_PADDING)
            .align_items(Alignment::Center)
            .height(Length::Units(26));

        Column::new()
            .push(title_container)
            .push(Space::new(Length::Units(0), Length::Units(5)))
            .push(theme_input_row)
    };

    let open_theme_row = {
        let open_button = Button::new(
            &mut state.theme_state.open_builder_button_state,
            Text::new(localized_string("open-theme-builder")).size(DEFAULT_FONT_SIZE),
        )
        .on_press(Interaction::OpenLink(String::from(
            "https://theme.getajour.com",
        )))
        .style(style::DefaultBoxedButton(color_palette));

        let open_button: Element<Interaction> = open_button.into();

        Row::new()
            .push(open_button.map(Message::Interaction))
            .align_items(Alignment::Center)
            .height(Length::Units(26))
    };

    let theme_scale_row = Row::new()
        .push(theme_column)
        .push(scale_column)
        .push(import_theme_column)
        .spacing(DEFAULT_PADDING);

    #[cfg(target_os = "windows")]
    let close_to_tray_column = {
        let checkbox = Checkbox::new(
            config.close_to_tray,
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

    #[cfg(target_os = "windows")]
    let toggle_autostart_column = {
        let checkbox = Checkbox::new(
            config.autostart,
            localized_string("toggle-autostart"),
            Interaction::ToggleAutoStart,
        )
        .style(style::DefaultCheckbox(color_palette))
        .text_size(DEFAULT_FONT_SIZE)
        .spacing(5);

        let checkbox: Element<Interaction> = checkbox.into();

        let checkbox_container = Container::new(checkbox.map(Message::Interaction))
            .style(style::NormalBackgroundContainer(color_palette));
        Column::new().push(checkbox_container)
    };

    #[cfg(target_os = "windows")]
    let start_closed_to_tray_column = {
        let checkbox = Checkbox::new(
            config.start_closed_to_tray,
            localized_string("start-closed-to-tray"),
            Interaction::ToggleStartClosedToTray,
        )
        .style(style::DefaultCheckbox(color_palette))
        .text_size(DEFAULT_FONT_SIZE)
        .spacing(5);

        let checkbox: Element<Interaction> = checkbox.into();

        let checkbox_container = Container::new(checkbox.map(Message::Interaction))
            .style(style::NormalBackgroundContainer(color_palette));
        Column::new().push(checkbox_container)
    };

    scrollable = scrollable
        .push(language_container)
        .push(Space::new(Length::Units(0), Length::Units(10)))
        .push(theme_scale_row)
        .push(Space::new(Length::Units(0), Length::Units(10)))
        .push(open_theme_row);

    #[cfg(target_os = "windows")]
    {
        scrollable = scrollable
            .push(Space::new(Length::Units(0), Length::Units(10)))
            .push(close_to_tray_column)
            .push(Space::new(Length::Units(0), Length::Units(10)))
            .push(start_closed_to_tray_column)
            .push(Space::new(Length::Units(0), Length::Units(10)))
            .push(toggle_autostart_column);
    }

    // Colum wrapping all the settings content.
    scrollable = scrollable.height(Length::Fill).width(Length::Fill);

    let col = Column::new()
        .push(Space::new(Length::Units(0), Length::Units(10)))
        .push(scrollable)
        .push(Space::new(Length::Units(0), Length::Units(20)));
    let row = Row::new()
        .push(Space::new(Length::Units(20), Length::Units(0)))
        .push(col);

    // Returns the final container.
    Container::new(row)
        .center_x()
        .width(Length::Fill)
        .height(Length::Shrink)
        .style(style::NormalBackgroundContainer(color_palette))
}
