use futures::future;
use grin_gui_core::config::Currency;

use {
    super::{DEFAULT_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{GrinGui, Interaction, Message},
    crate::localization::{localized_string, LANG},
    crate::{log_error, Result},
    anyhow::Context,
    grin_gui_core::{
        config::{Config, Language},
        error::ThemeError,
        fs::{import_theme, PersistentData},
        theme::{
            Button, ColorPalette, Column, Container, Element, PickList, Row, Scrollable, Text,
            TextInput, Theme,
        },
    },
    iced::widget::{button, pick_list, scrollable, text_input, Checkbox, Space},
    iced::{Alignment, Command, Length},
    std::sync::{Arc, RwLock},
};

#[derive(Debug, Clone)]
pub struct StateContainer {
    pub theme_state: ThemeState,
    pub scale_state: ScaleState,
    //scrollable_state: scrollable::State,
    //localization_picklist_state: pick_list::State<Language>,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            theme_state: Default::default(),
            //scrollable_state: Default::default(),
            scale_state: Default::default(),
            //localization_picklist_state: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThemeState {
    pub themes: Vec<(String, Theme)>,
    pub current_theme_name: String,
    //pick_list_state: pick_list::State<String>,
    //input_state: text_input::State,
    input_url: String,
    //import_button_state: button::State,
    //open_builder_button_state: button::State,
}

impl Default for ThemeState {
    fn default() -> Self {
        let themes = Theme::all();

        ThemeState {
            themes,
            current_theme_name: "Dark".to_string(),
            //pick_list_state: Default::default(),
            //input_state: Default::default(),
            input_url: Default::default(),
            //import_button_state: Default::default(),
            //open_builder_button_state: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScaleState {
    pub scale: f64,
    //up_btn_state: button::State,
    //down_btn_state: button::State,
}

impl Default for ScaleState {
    fn default() -> Self {
        ScaleState {
            scale: 1.0,
            //up_btn_state: Default::default(),
            //down_btn_state: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    ThemeSelected(String),
    CurrencySelected(Currency),
    LanguageSelected(Language),
    ScaleUp,
    ScaleDown,
    ThemeUrlInput(String),
    ImportTheme,
    ThemeImportedOk((String, Vec<Theme>)),
    ThemeImportedError(Arc<RwLock<Option<anyhow::Error>>>),
}

#[derive(Debug, Clone)]
pub enum Mode {
    Wallet,
    Node,
    General,
}

pub fn handle_message(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let state = &mut grin_gui.general_settings_state;
    match message {
        LocalViewInteraction::CurrencySelected(currency) => {
            log::debug!(
                "settings::general::LocalViewInteraction::CurrencySelected({:?})",
                &currency
            );

            grin_gui.config.currency = currency;
            let _ = grin_gui.config.save();

            return Ok(Command::perform(future::ready(()), |r| {
                // update the prices
                Message::Interaction(Interaction::WalletOperationHomeViewInteraction(
                    crate::gui::element::wallet::operation::home::LocalViewInteraction::UpdatePrices,
                ))
            }));
        }
        LocalViewInteraction::ThemeSelected(theme_name) => {
            log::debug!(
                "settings::general::LocalViewInteraction::ThemeSelected({:?})",
                &theme_name
            );

            // set theme for gui
            let theme = &state
                .theme_state
                .themes
                .iter()
                .find(|x| theme_name == x.0)
                .unwrap()
                .1;
            grin_gui.theme = theme.clone();

            state.theme_state.current_theme_name = theme_name.clone();
            grin_gui.config.theme = Some(theme_name);

            let _ = grin_gui.config.save();
        }
        LocalViewInteraction::LanguageSelected(lang) => {
            log::debug!(
                "settings::general::LocalViewInteraction::LanguageSelected({:?})",
                &lang
            );
            // Update config.
            grin_gui.config.language = lang;
            let _ = grin_gui.config.save();

            // Update global LANG refcell.
            *LANG.get().expect("LANG not set").write().unwrap() = lang.language_code();
        }
        LocalViewInteraction::ScaleUp => {
            let prev_scale = state.scale_state.scale;

            state.scale_state.scale = ((prev_scale + 0.1).min(2.0) * 10.0).round() / 10.0;

            grin_gui.config.scale = Some(state.scale_state.scale);
            let _ = grin_gui.config.save();

            log::debug!(
                "settings::general::LocalViewInteraction::ScaleUp({} -> {})",
                prev_scale,
                state.scale_state.scale
            );
        }
        LocalViewInteraction::ScaleDown => {
            let prev_scale = state.scale_state.scale;

            state.scale_state.scale = ((prev_scale - 0.1).max(0.5) * 10.0).round() / 10.0;

            grin_gui.config.scale = Some(state.scale_state.scale);
            let _ = grin_gui.config.save();

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
            grin_gui.error.take();

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
                        Message::Interaction(Interaction::GeneralSettingsViewInteraction(
                            LocalViewInteraction::ThemeImportedError(Arc::new(RwLock::new(Some(
                                e,
                            )))),
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
            grin_gui.config.theme = Some(new_theme_name);
            let _ = grin_gui.config.save();
        }
        LocalViewInteraction::ThemeImportedError(err) => {
            grin_gui.error = err.write().unwrap().take();
            if let Some(e) = grin_gui.error.as_ref() {
                log_error(e);
            }
            // Reset text input
            state.theme_state.input_url = Default::default();
            //state.theme_state.input_state = Default::default();
        }
    }
    Ok(Command::none())
}

pub fn data_container<'a>(state: &'a StateContainer, config: &Config) -> Container<'a, Message> {
    let language_container = {
        let title = Container::new(Text::new(localized_string("language")).size(DEFAULT_FONT_SIZE))
            .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let pick_list = PickList::new(&Language::ALL[..], Some(config.language), |l| {
            Message::Interaction(Interaction::GeneralSettingsViewInteraction(
                LocalViewInteraction::LanguageSelected(l),
            ))
        })
        .text_size(14)
        .width(Length::Fixed(120))
        .style(grin_gui_core::theme::PickListStyle::Primary);

        let container = Container::new(pick_list)
            .center_y()
            .width(Length::Fixed(120))
            .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        Column::new()
            .push(title)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(5.0)))
            .push(container)
    };

    let currency_container = {
        let title = Container::new(Text::new(localized_string("currency")).size(DEFAULT_FONT_SIZE))
            .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let pick_list = PickList::new(&Currency::ALL[..], Some(config.currency), |c| {
            Message::Interaction(Interaction::GeneralSettingsViewInteraction(
                LocalViewInteraction::CurrencySelected(c),
            ))
        })
        .text_size(14)
        .width(Length::Fixed(120))
        .style(grin_gui_core::theme::PickListStyle::Primary);

        let container = Container::new(pick_list)
            .center_y()
            .width(Length::Fixed(120))
            .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        Column::new()
            .push(title)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(5.0)))
            .push(container)
    };

    let theme_column = {
        let title_container =
            Container::new(Text::new(localized_string("theme")).size(DEFAULT_FONT_SIZE))
                .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let theme_names = state
            .theme_state
            .themes
            .iter()
            .cloned()
            .map(|(name, _)| name)
            .collect::<Vec<_>>();

        let theme_pick_list = PickList::new(
            theme_names,
            Some(state.theme_state.current_theme_name.clone()),
            |t| {
                Message::Interaction(Interaction::GeneralSettingsViewInteraction(
                    LocalViewInteraction::ThemeSelected(t),
                ))
            },
        )
        .text_size(DEFAULT_FONT_SIZE)
        .width(Length::Fixed(120))
        .style(grin_gui_core::theme::PickListStyle::Primary);

        // Data row for theme picker list.
        let theme_data_row = Row::new()
            .push(theme_pick_list)
            .align_items(Alignment::Center)
            .height(Length::Fixed(26));

        Column::new()
            .push(title_container)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(5.0)))
            .push(theme_data_row)
    };

    // Scale buttons for application scale factoring.
    let scale_column = {
        let title_container =
            Container::new(Text::new(localized_string("scale")).size(DEFAULT_FONT_SIZE))
                .style(grin_gui_core::theme::ContainerStyle::NormalBackground);
        let scale_title_row = Row::new().push(title_container);

        let scale_down_button: Element<Interaction> = Button::new(
            // &mut state.scale_state.down_btn_state,
            Text::new("  -  ").size(DEFAULT_FONT_SIZE),
        )
        .style(grin_gui_core::theme::ButtonStyle::Bordered)
        .on_press(Interaction::GeneralSettingsViewInteraction(
            LocalViewInteraction::ScaleDown,
        ))
        .into();

        let scale_up_button: Element<Interaction> = Button::new(
            // &mut state.scale_state.up_btn_state,
            Text::new("  +  ").size(DEFAULT_FONT_SIZE),
        )
        .style(grin_gui_core::theme::ButtonStyle::Bordered)
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
            .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let scale_buttons_row = Row::new()
            .push(scale_down_button.map(Message::Interaction))
            .push(current_scale_container)
            .push(scale_up_button.map(Message::Interaction))
            .align_items(Alignment::Center)
            .height(Length::Fixed(26));

        Column::new()
            .push(scale_title_row)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(5.0)))
            .push(scale_buttons_row)
    };

    let import_theme_column = {
        let title_container =
            Container::new(Text::new(localized_string("import-theme")).size(DEFAULT_FONT_SIZE))
                .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let theme_input = TextInput::new(
            &localized_string("paste-url")[..],
            &state.theme_state.input_url,
            |s| Interaction::GeneralSettingsViewInteraction(LocalViewInteraction::ThemeUrlInput(s)),
        )
        .size(DEFAULT_FONT_SIZE)
        .padding(6)
        .width(Length::Fixed(185))
        .style(grin_gui_core::theme::TextInputStyle::AddonsQuery);

        let theme_input: Element<Interaction> = theme_input.into();

        let mut import_button = Button::new(
            // &mut state.theme_state.import_button_state,
            Text::new(localized_string("import-theme-button")).size(DEFAULT_FONT_SIZE),
        )
        .style(grin_gui_core::theme::ButtonStyle::Bordered);

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
            .height(Length::Fixed(26));

        Column::new()
            .push(title_container)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(5.0)))
            .push(theme_input_row)
    };

    let open_theme_row = {
        let open_button = Button::new(
            // &mut state.theme_state.open_builder_button_state,
            Text::new(localized_string("open-theme-builder")).size(DEFAULT_FONT_SIZE),
        )
        .on_press(Interaction::OpenLink(String::from(
            "https://theme.getajour.com",
        )))
        .style(grin_gui_core::theme::ButtonStyle::Bordered);

        let open_button: Element<Interaction> = open_button.into();

        Row::new()
            .push(open_button.map(Message::Interaction))
            .align_items(Alignment::Center)
            .height(Length::Fixed(26))
    };

    let theme_scale_row = Row::new()
        .push(theme_column)
        .push(scale_column)
        .push(import_theme_column)
        .spacing(DEFAULT_PADDING);

    #[cfg(target_os = "windows")]
    let close_to_tray_column = {
        let checkbox = Checkbox::new(
            localized_string("close-to-tray"),
            config.close_to_tray,
            Interaction::ToggleCloseToTray,
        )
        .style(grin_gui_core::theme::CheckboxStyle::Normal)
        .text_size(DEFAULT_FONT_SIZE)
        .spacing(5);

        let checkbox: Element<Interaction> = checkbox.into();

        let checkbox_container = Container::new(checkbox.map(Message::Interaction))
            .style(grin_gui_core::theme::ContainerStyle::NormalBackground);
        Column::new().push(checkbox_container)
    };

    #[cfg(target_os = "windows")]
    let toggle_autostart_column = {
        let checkbox = Checkbox::new(
            localized_string("toggle-autostart"),
            config.autostart,
            Interaction::ToggleAutoStart,
        )
        .style(grin_gui_core::theme::CheckboxStyle::Normal)
        .text_size(DEFAULT_FONT_SIZE)
        .spacing(5);

        let checkbox: Element<Interaction> = checkbox.into();

        let checkbox_container = Container::new(checkbox.map(Message::Interaction))
            .style(grin_gui_core::theme::ContainerStyle::NormalBackground);
        Column::new().push(checkbox_container)
    };

    #[cfg(target_os = "windows")]
    let start_closed_to_tray_column = {
        let checkbox = Checkbox::new(
            localized_string("start-closed-to-tray"),
            config.start_closed_to_tray,
            Interaction::ToggleStartClosedToTray,
        )
        .style(grin_gui_core::theme::CheckboxStyle::Normal)
        .text_size(DEFAULT_FONT_SIZE)
        .spacing(5);

        let checkbox: Element<Interaction> = checkbox.into();

        let checkbox_container = Container::new(checkbox.map(Message::Interaction))
            .style(grin_gui_core::theme::ContainerStyle::NormalBackground);
        Column::new().push(checkbox_container)
    };

    let mut column = Column::new()
        .spacing(1)
        .push(language_container)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(10.0)))
        .push(currency_container)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(10.0)))
        .push(theme_scale_row)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(10.0)))
        .push(open_theme_row)
        .spacing(1);

    // Systray settings
    #[cfg(target_os = "windows")]
    {
        column = column
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(10.0)))
            .push(close_to_tray_column)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(10.0)))
            .push(start_closed_to_tray_column)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(10.0)))
            .push(toggle_autostart_column);
    }

    let scrollable = Scrollable::new(column)
        .height(Length::Fill)
        .style(grin_gui_core::theme::ScrollableStyle::Primary);

    // Colum wrapping all the settings content.
    //scrollable = scrollable.height(Length::Fill);

    let col = Column::new()
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(10.0)))
        .push(scrollable)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(20)));
    let row = Row::new()
        .push(Space::new(Length::Fixed(5.0), Length::Fixed(0.0)))
        .push(col);

    // Returns the final container.
    Container::new(row)
        .width(Length::Fill)
        .height(Length::Shrink)
        .style(grin_gui_core::theme::ContainerStyle::NormalBackground)
}
