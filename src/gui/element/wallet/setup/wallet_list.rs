use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::config::Config,
    iced::{
        alignment, button, Alignment, Button, Checkbox, Column, Command, Container,
        Element, Length, Row, Space, Text
    },
    native_dialog::FileDialog,
    std::path::PathBuf
};

use grin_gui_widgets::{TableRow, table_row::StyleSheet};
#[derive(Debug)]
pub struct StateContainer {
    pub back_button_state: button::State,
    selected_wallet_index: usize,
    load_wallet_button_state: button::State,
    select_folder_button_state: button::State

}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            back_button_state: Default::default(),
            selected_wallet_index:0,
            load_wallet_button_state: Default::default(),
            select_folder_button_state: Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    Back,
    WalletRowSelect(bool, usize),
    LoadWallet(usize),
    LocateWallet
}

pub fn handle_message<'a>(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    match message {
        LocalViewInteraction::Back => {
            grin_gui.wallet_state.setup_state.mode = super::Mode::Init;
        }
        LocalViewInteraction::WalletRowSelect(is_selected, index) => {
            if is_selected {
                grin_gui.wallet_state.setup_state
                    .setup_wallet_list_state.selected_wallet_index = index;
            }
        }
        LocalViewInteraction::LoadWallet(index) => {
            grin_gui.config.current_wallet_index = Some(index);
            grin_gui.wallet_state.mode = crate::gui::element::wallet::Mode::Operation;
        }
        LocalViewInteraction::LocateWallet => {
            match FileDialog::new().show_open_single_file(){
                Ok(path) => {
                    match path {
                        Some(d) => {
                            match validate_directory(d) {
                                Ok(wallet_was_imported) => {
                                    
                                }
                                Err(err) => {
                                    // tell the user why this directory failed
                                }
                            }
                        },
                        None => {}
                    }
                }
                Err(e) => {
                log::debug!("wallet_list.rs::LocalViewInteraction::LocateWallet {}", e);
                }
            };
        }
    }

    Ok(Command::none())
}

struct DirectoryValidationError;

fn validate_directory(_d:PathBuf) -> Result<bool, DirectoryValidationError> {
    
    Ok(true)
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    state: &'a mut StateContainer,
    config:&Config
) -> Container<'a, Message> {

    let title = Text::new(localized_string("wallet-list"))
        .size(DEFAULT_HEADER_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);
    let title_container = Container::new(title)
        .style(style::NormalBackgroundContainer(color_palette))
        .align_x(alignment::Horizontal::Center);

    let back_button_label_container =
        Container::new(Text::new(localized_string("back")).size(DEFAULT_FONT_SIZE))
            .height(Length::Units(20))
            .align_y(alignment::Vertical::Bottom)
            .align_x(alignment::Horizontal::Center);

    let back_button: Element<Interaction> =
        Button::new(&mut state.back_button_state, back_button_label_container)
            .style(style::NormalTextButton(color_palette))
            .on_press(Interaction::WalletListWalletViewInteraction(
                LocalViewInteraction::Back,
            ))
            .into();

    let title_row = Row::new()
        .push(title_container)
        .push(Space::new(Length::Units(100), Length::Units(0)))
        .push(back_button.map(Message::Interaction))
        .align_items(Alignment::Center)
        .spacing(20);

    let title_column = Column::new()
        .push(title_row)
        //.push(Space::new(Length::Units(DEFAULT_PADDING), Length::Units(0)))
        .align_items(Alignment::Fill);
   
    let name_header = Text::new(localized_string("Name"))
        .size(DEFAULT_HEADER_FONT_SIZE);

    let name_header_container = Container::new(name_header)
        .style(style::NormalBackgroundContainer(color_palette));

    let directory_header = Text::new(localized_string("Location"))
        .size(DEFAULT_HEADER_FONT_SIZE);

    let directory_header_container = Container::new(directory_header)
        .style(style::NormalBackgroundContainer(color_palette));

    let header_row = Row::new()
        .push(name_header_container)
        .push(Space::new(Length::Units(135), Length::Units(0)))
        .push(directory_header_container);
        
    let mut wallet_rows:Vec<_> = vec![];
    for (pos, w) in config.wallets.iter().enumerate() {
        let checkbox = Checkbox::new(
            state.selected_wallet_index == pos,
            "",
            move |b| {
                Message::Interaction(
                    Interaction::WalletListWalletViewInteraction(
                        LocalViewInteraction::WalletRowSelect(b, pos),
                    )
                )
            },
        )
        .style(style::DefaultCheckbox(color_palette))
        .text_size(DEFAULT_FONT_SIZE)
        .spacing(10);

        let wallet_name = Text::new(w.display_name.clone())
            .size(DEFAULT_HEADER_FONT_SIZE);

        let wallet_name_container = Container::new(wallet_name)
            .style(style::NormalForegroundContainer(color_palette));

        let tld_string = match &w.tld {
            Some(path_buf) => path_buf.display().to_string(),
            None => String::from("Unknown")
        };
        let wallet_directory = Text::new(tld_string)
            .size(DEFAULT_HEADER_FONT_SIZE);

        let wallet_directory_container = Container::new(wallet_directory)
            .style(style::NormalForegroundContainer(color_palette));

        let wallet_row = Row::new()
            .push(checkbox)
            .push(Column::new().push(wallet_name_container).width(Length::Units(80)))
            //.push(wallet_name_container)
            .push(Space::new(Length::Units(58), Length::Units(0)))
            .push(wallet_directory_container);

        let tr_style = grin_gui_widgets::style::table_row::Default;
        tr_style.style().text_color = Some(color_palette.bright.primary);
        tr_style.hovered().text_color = Some(color_palette.bright.secondary);

        let table_row = TableRow::new(wallet_row)
            .padding(iced::Padding::from(2))
            .style(tr_style);
            /*.on_press(move |_e| {*/
                /*println!("table_row on press");*/
                /*Message::Interaction(*/
                    /*Interaction::WalletListWalletViewInteraction(*/
                        /*LocalViewInteraction::WalletRowSelect(true, pos)))*/
            /*});*/
        wallet_rows.push(table_row.into());
    }

    let c = Container::new(Column::new()
        .push(Space::new(Length::Units(0), Length::Units(3)))
        .push(Column::with_children(wallet_rows))
        .push(Space::new(Length::Units(0), Length::Units(3))))
        .style(style::ChannelBadge(color_palette))
        .padding(iced::Padding::from(DEFAULT_PADDING));

    let wallet_column = Column::new()
        .push(header_row)
        .push(Space::new(Length::Units(0), Length::Units(5)))
        .push(c);

    let load_wallet_button_container =
        Container::new(Text::new(localized_string("load-wallet")).size(DEFAULT_FONT_SIZE))
            .height(Length::Units(20))
            .align_y(alignment::Vertical::Center)
            .align_x(alignment::Horizontal::Center);

    let load_wallet_button : Element<Interaction> =
        Button::new(&mut state.load_wallet_button_state, load_wallet_button_container)
            .style(style::DefaultBoxedButton(color_palette))
            .on_press(Interaction::WalletListWalletViewInteraction(
                LocalViewInteraction::LoadWallet(state.selected_wallet_index),
            ))
            .into();

    let select_folder_button_container = 
        Container::new(Text::new(localized_string("select-other")).size(DEFAULT_FONT_SIZE))
            .height(Length::Units(20))
            .align_y(alignment::Vertical::Center)
            .align_x(alignment::Horizontal::Center);

    let select_other_button : Element<Interaction> = 
        Button::new(&mut state.select_folder_button_state, select_folder_button_container)
            .style(style::DefaultBoxedButton(color_palette))
            .on_press(Interaction::WalletListWalletViewInteraction(
                LocalViewInteraction::LocateWallet
        ))
        .into();

    let button_row = Row::new()
        .push(load_wallet_button.map(Message::Interaction))
        .push(Space::new(Length::Units(DEFAULT_PADDING), Length::Units(0)))
        .push(select_other_button.map(Message::Interaction))
        .padding(10);
        
    let parent = Column::new()
        .push(title_column)
        .push(Space::new(Length::Units(0), Length::Units(15)))
        .push(wallet_column)
        .push(Space::new(Length::Units(0), Length::Units(DEFAULT_PADDING)))
        .push(button_row);

    Container::new(parent)
        //.center_y()
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
}
