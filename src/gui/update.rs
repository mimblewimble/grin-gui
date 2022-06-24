use {
    super::{GrinGui, Interaction, Message, Mode},
    crate::{gui::element, localization::localized_string, log_error, Result},
    grin_gui_core::{error::ThemeError, fs::PersistentData},
    anyhow::Context,
    iced::Command,
    //grin_gui_widgets::header::ResizeEvent,
    std::path::PathBuf,
};

#[cfg(target_os = "windows")]
use crate::tray::{TrayMessage, SHOULD_EXIT, TRAY_SENDER};
#[cfg(target_os = "windows")]
use std::sync::atomic::Ordering;

pub fn handle_message(grin_gui: &mut GrinGui, message: Message) -> Result<Command<Message>> {
    match message {
        Message::Interaction(Interaction::MenuViewInteraction(local_interaction)) => {
            let _ = element::menu::handle_message(&mut grin_gui.menu_state, local_interaction);
        }
        Message::Interaction(Interaction::SettingsViewInteraction(local_interaction)) => {
            element::settings::handle_message(&mut grin_gui.settings_state, local_interaction);
        }
        // Wallet Settings
        Message::Interaction(Interaction::WalletSettingsViewInteraction(local_interaction)) => {
            element::settings::wallet::handle_message(
                &mut grin_gui.wallet_settings_state,
                local_interaction,
            );
        }
        // Node Settings
        Message::Interaction(Interaction::NodeSettingsViewInteraction(local_interaction)) => {
            element::settings::node::handle_message(
                &mut grin_gui.node_settings_state,
                local_interaction,
            );
        }
        // General Settings
        Message::Interaction(Interaction::GeneralSettingsViewInteraction(local_interaction)) => {
            return element::settings::general::handle_message(
                &mut grin_gui.general_settings_state,
                &mut grin_gui.config,
                local_interaction,
                &mut grin_gui.error,
            );
        }
        Message::GeneralSettingsViewThemeSelected(selected) => {
            let _ = element::settings::general::handle_message(
                &mut grin_gui.general_settings_state,
                &mut grin_gui.config,
                element::settings::general::LocalViewInteraction::ThemeSelected(selected),
                &mut grin_gui.error,
            );
        }
        Message::GeneralSettingsViewThemeImported(result) => {
            match result.context("Failed to Import Theme") {
                Ok(result) => {
                    let _ = element::settings::general::handle_message(
                        &mut grin_gui.general_settings_state,
                        &mut grin_gui.config,
                        element::settings::general::LocalViewInteraction::ThemeImportedOk(result),
                        &mut grin_gui.error,
                    );
                }
                Err(mut error) => {
                    let _ = element::settings::general::handle_message(
                        &mut grin_gui.general_settings_state,
                        &mut grin_gui.config,
                        element::settings::general::LocalViewInteraction::ThemeImportedError,
                        &mut grin_gui.error,
                    );
                    // Assign special error message when updating failed due to
                    // collision
                    for cause in error.chain() {
                        if let Some(theme_error) = cause.downcast_ref::<ThemeError>() {
                            if matches!(theme_error, ThemeError::NameCollision { .. }) {
                                error = error
                                    .context(localized_string("import-theme-error-name-collision"));
                                break;
                            }
                        }
                    }

                    log_error(&error);
                    grin_gui.error = Some(error);
                }
            }
        }
        Message::Interaction(Interaction::GeneralSettingsViewLanguageSelected(language)) => {
            let _ = element::settings::general::handle_message(
                &mut grin_gui.general_settings_state,
                &mut grin_gui.config,
                element::settings::general::LocalViewInteraction::LanguageSelected(language),
                &mut grin_gui.error,
            );
        }
        Message::Interaction(Interaction::GeneralSettingsViewThemeUrlInput(url)) => {
            let _ = element::settings::general::handle_message(
                &mut grin_gui.general_settings_state,
                &mut grin_gui.config,
                element::settings::general::LocalViewInteraction::ThemeUrlInput(url),
                &mut grin_gui.error,
            );
        }
        Message::Interaction(Interaction::ModeSelected(mode)) => {
            log::debug!("Interaction::ModeSelected({:?})", mode);
            // Set Mode
            grin_gui.mode = mode;
        }
        /*Message::MessageInteraction(m) => {
            m.handle_message()
        }*/
        Message::Interaction(Interaction::ModeSelectedSettings(mode)) => {
            log::debug!("Interaction::ModeSelectedSettings({:?})", mode);
            // Set Mode
            //grin_gui.settings_state.mode = mode;
        }
        Message::Error(error) => {
            log_error(&error);
            grin_gui.error = Some(error);
        }
        Message::RuntimeEvent(iced_native::Event::Window(
            iced_native::window::Event::Resized { width, height },
        )) => {
            let width = (width as f64 * grin_gui.general_settings_state.scale_state.scale) as u32;
            let height = (height as f64 * grin_gui.general_settings_state.scale_state.scale) as u32;

            // Minimizing Grin GUI on Windows will call this function with 0, 0.
            // We don't want to save that in config, because then it will start with zero size.
            if width > 0 && height > 0 {
                grin_gui.config.window_size = Some((width, height));
                let _ = grin_gui.config.save();
            }
        }
        #[cfg(target_os = "windows")]
        Message::RuntimeEvent(iced_native::Event::Window(
            iced_native::window::Event::CloseRequested,
        )) => {
            log::debug!("Message::RuntimeEvent(CloseRequested)");

            if let Some(sender) = TRAY_SENDER.get() {
                if grin_gui.config.close_to_tray {
                    let _ = sender.try_send(TrayMessage::CloseToTray);
                } else {
                    SHOULD_EXIT.store(true, Ordering::Relaxed);
                }
            }
        }
        Message::RuntimeEvent(iced_native::Event::Keyboard(
            iced_native::keyboard::Event::KeyReleased {
                key_code,
                modifiers,
            },
        )) => {
            // Bail out of keybindings if keybindings is diabled, or we are
            // pressing any modifiers.
            if !grin_gui.config.is_keybindings_enabled
                || modifiers != iced::keyboard::Modifiers::default()
            {
                return Ok(Command::none());
            }

            match key_code {
                iced::keyboard::KeyCode::A => {
                    /*let flavor = grin_gui.config.wow.flavor;
                    grin_gui.mode = Mode::MyAddons(flavor);*/
                }
                iced::keyboard::KeyCode::C => {
                    grin_gui.mode = Mode::Catalog;
                }
                iced::keyboard::KeyCode::R => {
                    /*let mode = grin_gui.mode.clone();
                    return handle_message(grin_gui, Message::Interaction(Interaction::Refresh(mode)));*/
                }
                iced::keyboard::KeyCode::S => {
                    grin_gui.mode = Mode::Settings;
                }
                iced::keyboard::KeyCode::U => {
                    /*let mode = grin_gui.mode.clone();
                    return handle_message(
                        grin_gui,
                        Message::Interaction(Interaction::UpdateAll(mode)),
                    );*/
                }
                iced::keyboard::KeyCode::W => {
                    /*let flavor = grin_gui.config.wow.flavor;
                    grin_gui.mode = Mode::MyWeakAuras(flavor);*/
                }
                iced::keyboard::KeyCode::I => {
                    grin_gui.mode = Mode::Install;
                }
                iced::keyboard::KeyCode::Escape => match grin_gui.mode {
                    /*Mode::Settings | Mode::About => {
                        grin_gui.mode = Mode::MyAddons(grin_gui.config.wow.flavor);
                    }
                    Mode::MyAddons(_) => {
                        grin_gui.addons_search_state.query = None;
                    }
                    Mode::Catalog => {
                        grin_gui.catalog_search_state.query = None;
                    }*/
                    _ => (),
                },
                _ => (),
            }
        }
        #[cfg(target_os = "windows")]
        Message::Interaction(Interaction::ToggleCloseToTray(enable)) => {
            log::debug!("Interaction::ToggleCloseToTray({})", enable);

            grin_gui.config.close_to_tray = enable;

            // Remove start closed to tray if we are disabling
            if !enable {
                grin_gui.config.start_closed_to_tray = false;
            }

            let _ = grin_gui.config.save();

            if let Some(sender) = TRAY_SENDER.get() {
                let msg = if enable {
                    TrayMessage::Enable
                } else {
                    TrayMessage::Disable
                };

                let _ = sender.try_send(msg);
            }
        }
        #[cfg(target_os = "windows")]
        Message::Interaction(Interaction::ToggleAutoStart(enable)) => {
            log::debug!("Interaction::ToggleAutoStart({})", enable);

            grin_gui.config.autostart = enable;

            let _ = grin_gui.config.save();

            if let Some(sender) = TRAY_SENDER.get() {
                let _ = sender.try_send(TrayMessage::ToggleAutoStart(enable));
            }
        }
        #[cfg(target_os = "windows")]
        Message::Interaction(Interaction::ToggleStartClosedToTray(enable)) => {
            log::debug!("Interaction::ToggleStartClosedToTray({})", enable);

            grin_gui.config.start_closed_to_tray = enable;

            // Enable tray if this feature is enabled
            if enable && !grin_gui.config.close_to_tray {
                grin_gui.config.close_to_tray = true;

                if let Some(sender) = TRAY_SENDER.get() {
                    let _ = sender.try_send(TrayMessage::Enable);
                }
            }

            let _ = grin_gui.config.save();
        }
        Message::Interaction(Interaction::OpenLink(link)) => {
            log::debug!("Interaction::OpenLink({})", &link);

            return Ok(Command::perform(
                async {
                    let _ = opener::open(link);
                },
                Message::None,
            ));
        }
        Message::Interaction(_) => {}
        Message::RuntimeEvent(_) => {}
        Message::None(_) => {}
    }

    Ok(Command::none())
}

#[cfg(not(target_os = "linux"))]
async fn select_directory() -> Option<PathBuf> {
    use rfd::AsyncFileDialog;

    let dialog = AsyncFileDialog::new();
    if let Some(show) = dialog.pick_folder().await {
        return Some(show.path().to_path_buf());
    }

    None
}

#[cfg(target_os = "linux")]
async fn select_directory() -> Option<PathBuf> {
    use native_dialog::FileDialog;

    let dialog = FileDialog::new();
    if let Ok(Some(show)) = dialog.show_open_single_dir() {
        return Some(show);
    }

    None
}

/// Hardcoded binary names for each compilation target
/// that gets published to the Github Release
const fn bin_name() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "grin-gui.exe"
    }

    #[cfg(target_os = "macos")]
    {
        "grin-gui"
    }

    #[cfg(target_os = "linux")]
    {
        "grin-gui.AppImage"
    }
}
