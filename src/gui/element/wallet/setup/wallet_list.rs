use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING, BUTTON_HEIGHT, BUTTON_WIDTH},
    crate::gui::{style, GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    grin_gui_core::config::Config,
    grin_gui_core::{
        theme::ColorPalette,
        wallet::{create_grin_wallet_path, ChainTypes},
    },
    iced::{alignment, Alignment, Command, Length},
    grin_gui_core::theme::{Button, Column, Element, Container, PickList, Row, Scrollable, Text, TextInput, Header, TableRow},
    iced::widget::{
        button, pick_list, scrollable, text_input, Checkbox, Space,
    },
    native_dialog::FileDialog,
    std::path::PathBuf,
};

use grin_gui_widgets::{table_row::StyleSheet};
use isahc::head;

use crate::gui::element::DEFAULT_SUB_HEADER_FONT_SIZE;


pub struct StateContainer {
    selected_wallet_index: usize,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            selected_wallet_index: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    Back,
    WalletRowSelect(bool, usize),
    LoadWallet(usize),
    LocateWallet,
    CreateWallet,
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
                grin_gui
                    .wallet_state
                    .setup_state
                    .setup_wallet_list_state
                    .selected_wallet_index = index;
            }
        }
        LocalViewInteraction::LoadWallet(index) => {
            grin_gui.config.current_wallet_index = Some(index);
            grin_gui.wallet_state.mode = crate::gui::element::wallet::Mode::Operation;
        }
        LocalViewInteraction::LocateWallet => {
            match FileDialog::new().show_open_single_file() {
                Ok(path) => {
                    match path {
                        Some(d) => {
                            match validate_directory(d) {
                                Ok(wallet_was_imported) => {}
                                Err(err) => {
                                    // tell the user why this directory failed
                                }
                            }
                        }
                        None => {}
                    }
                }
                Err(e) => {
                    log::debug!("wallet_list.rs::LocalViewInteraction::LocateWallet {}", e);
                }
            };
        }
        LocalViewInteraction::CreateWallet => {
            let state = &mut grin_gui.wallet_state.setup_state;
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

            grin_gui.wallet_state.mode =
                crate::gui::element::wallet::Mode::CreateWallet(wallet_display_name);
        }
    }

    Ok(Command::none())
}

struct DirectoryValidationError;

fn validate_directory(_d: PathBuf) -> Result<bool, DirectoryValidationError> {
    Ok(true)
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    state: &'a StateContainer,
    config: &Config,
) -> Container<'a, Message> {
    let button_height = Length::Units(BUTTON_HEIGHT);
    let button_width = Length::Units(BUTTON_WIDTH);

    let title = Text::new(localized_string("wallet-list")).size(DEFAULT_HEADER_FONT_SIZE);
    let title_container =
        Container::new(title).style(grin_gui_core::theme::container::Container::NormalBackground(color_palette));

    let new_wallet_container =
        Container::new(Text::new(localized_string("create-wallet")).size(DEFAULT_FONT_SIZE))
            //.height(button_height)
            .align_y(alignment::Vertical::Center)
            .align_x(alignment::Horizontal::Center);

    let new_wallet_button: Element<Interaction> =
        Button::new( new_wallet_container)
            .style(grin_gui_core::theme::button::Button::Primary(color_palette))
            .on_press(Interaction::WalletListWalletViewInteraction(
                LocalViewInteraction::CreateWallet,
            ))
            .into();

    // add additional buttons here
    let button_row = Row::new().push(new_wallet_button.map(Message::Interaction));

    let segmented_mode_container = Container::new(button_row).padding(1);
    let segmented_mode_control_container = Container::new(segmented_mode_container)
        .style(grin_gui_core::theme::container::Container::Segmented(color_palette))
        .padding(1);

    let header_row = Row::new()
        .push(title_container)
        .push(Space::with_width(Length::Fill))
        .push(segmented_mode_control_container)
        .align_items(Alignment::Center);

    let header_container = Container::new(header_row).padding(iced::Padding::from([
        0,               // top
        0,               // right
        DEFAULT_PADDING, // bottom
        5,               // left
    ]));

    let name_header = Text::new(localized_string("name")).size(DEFAULT_SUB_HEADER_FONT_SIZE);
    let name_header_container =
        Container::new(name_header).style(grin_gui_core::theme::container::Container::NormalBackground(color_palette));

    let chain_header = Text::new(localized_string("type")).size(DEFAULT_SUB_HEADER_FONT_SIZE);
    let chain_header_container =
        Container::new(chain_header).style(grin_gui_core::theme::container::Container::NormalBackground(color_palette));

    let directory_header = Text::new(localized_string("folder")).size(DEFAULT_SUB_HEADER_FONT_SIZE);
    let directory_header_container = Container::new(directory_header)
        .style(grin_gui_core::theme::container::Container::NormalBackground(color_palette));

    let table_header_row = Row::new()
        .push(
            Column::new()
                .push(name_header_container)
                .width(Length::FillPortion(1)),
        )
        .push(
            Column::new()
                .push(chain_header_container)
                .width(Length::FillPortion(1)),
        )
        .push(
            Column::new()
                .push(directory_header_container)
                .width(Length::FillPortion(3)),
        );

    let table_header_container = Container::new(table_header_row)
        .padding(iced::Padding::from([
            9,               // top
            DEFAULT_PADDING, // right - should roughly match width of content scroll bar to align table headers
            9,               // bottom
            9,               // left
        ]))
        .style(grin_gui_core::theme::container::Container::PanelForeground(color_palette));

    let mut wallet_rows: Vec<_> = vec![];
    for (pos, w) in config.wallets.iter().enumerate() {
        // si quieres el checkbox
        // let checkbox = Checkbox::new(state.selected_wallet_index == pos, "", move |b| {
        //     Message::Interaction(Interaction::WalletListWalletViewInteraction(
        //         LocalViewInteraction::WalletRowSelect(b, pos),
        //     ))
        // })
        // .style(grin_gui_core::theme::checkbox::CheckboxStyles::Normal(color_palette))
        // .text_size(DEFAULT_FONT_SIZE)
        // .spacing(10);

        let selected_wallet = state.selected_wallet_index == pos;
        let wallet_name = Text::new(w.display_name.clone()).size(DEFAULT_FONT_SIZE);
        let chain_name = Text::new(w.chain_type.shortname()).size(DEFAULT_FONT_SIZE);

        let mut wallet_name_container =
            Container::new(wallet_name).style(grin_gui_core::theme::container::Container::HoverableForeground(color_palette));

        let mut wallet_chain_container =
            Container::new(chain_name).style(grin_gui_core::theme::container::Container::HoverableForeground(color_palette));

        let tld_string = match &w.tld {
            Some(path_buf) => path_buf.display().to_string(),
            None => String::from("Unknown"),
        };
        let wallet_directory = Text::new(tld_string).size(DEFAULT_FONT_SIZE);

        let mut wallet_directory_container = Container::new(wallet_directory)
            .style(grin_gui_core::theme::container::Container::HoverableForeground(color_palette));

        if selected_wallet {
            wallet_name_container = wallet_name_container
                .style(grin_gui_core::theme::container::Container::HoverableBrightForeground(color_palette));
            wallet_chain_container = wallet_chain_container
                .style(grin_gui_core::theme::container::Container::HoverableBrightForeground(color_palette));
            wallet_directory_container = wallet_directory_container
                .style(grin_gui_core::theme::container::Container::HoverableBrightForeground(color_palette));
        }

        let wallet_row = Row::new()
            //.push(checkbox)
            .push(
                Column::new()
                    .push(wallet_name_container)
                    .width(Length::FillPortion(1)),
            )
            .push(
                Column::new()
                    .push(wallet_chain_container)
                    .width(Length::FillPortion(1)),
            )
            .push(
                Column::new()
                    .push(wallet_directory_container)
                    .width(Length::FillPortion(3)),
            );

        let mut table_row = TableRow::new(wallet_row)
            .padding(iced::Padding::from(9))
            .width(Length::Fill)
            .on_press(move |_| {
                log::debug!("data_container::table_row::on_press {}", pos);

                Message::Interaction(Interaction::WalletListWalletViewInteraction(
                    LocalViewInteraction::WalletRowSelect(true, pos),
                ))
            });

        if selected_wallet {
            // selected wallet should be highlighted
            table_row = table_row.style(style::TableRowSelected(color_palette));
        } else {
            // contrast row styles to spice things up
            if pos % 2 == 0 {
                table_row = table_row.style(style::TableRowLowlife(color_palette));
            } else {
                table_row = table_row.style(style::TableRowHighlife(color_palette));
            }
        }

        wallet_rows.push(table_row.into());
    }

    let wallet_column = Column::new().push(Column::with_children(wallet_rows));

    let load_wallet_button_container =
        Container::new(Text::new(localized_string("load-wallet")).size(DEFAULT_FONT_SIZE))
            .width(button_width)
            .height(button_height)
            .align_y(alignment::Vertical::Center)
            .align_x(alignment::Horizontal::Center);

    let mut load_wallet_button = Button::new(
        load_wallet_button_container,
    )
    .style(grin_gui_core::theme::button::Button::Primary(color_palette));

    // the load wallet button should be disabled if there are no wallets
    if !config.wallets.is_empty() {
        load_wallet_button =
            load_wallet_button.on_press(Interaction::WalletListWalletViewInteraction(
                LocalViewInteraction::LoadWallet(state.selected_wallet_index),
            ))
    }

    let load_wallet_button: Element<Interaction> = load_wallet_button.into();

    let select_folder_button_container =
        Container::new(Text::new(localized_string("select-other")).size(DEFAULT_FONT_SIZE))
            .width(button_width)
            .height(button_height)
            .align_y(alignment::Vertical::Center)
            .align_x(alignment::Horizontal::Center);

    let select_other_button: Element<Interaction> = Button::new(
        select_folder_button_container,
    )
    .style(grin_gui_core::theme::button::Button::Primary(color_palette))
    .on_press(Interaction::WalletListWalletViewInteraction(
        LocalViewInteraction::LocateWallet,
    ))
    .into();

    // button lipstick
    let load_container = Container::new(load_wallet_button.map(Message::Interaction)).padding(1);
    let load_container = Container::new(load_container)
        .style(grin_gui_core::theme::container::Container::Segmented(color_palette))
        .padding(1);

    let select_container = Container::new(select_other_button.map(Message::Interaction)).padding(1);
    let select_container = Container::new(select_container)
        .style(grin_gui_core::theme::container::Container::Segmented(color_palette))
        .padding(1);

    let button_row = Row::new()
        .push(load_container)
        .push(Space::with_width(Length::Units(DEFAULT_PADDING)))
        .push(select_container)
        .height(Length::Shrink);

    let scrollable = Scrollable::new(wallet_column)
        .style(grin_gui_core::theme::scrollable::ScrollableStyles::Primary(color_palette));

    let table_colummn = Column::new().push(table_header_container).push(scrollable);
    let table_container = Container::new(table_colummn)
        .style(grin_gui_core::theme::container::Container::PanelBordered(color_palette))
        .height(Length::Fill)
        .padding(1);

    let row = Row::new().push(
        Column::new()
            .push(table_container)
            .push(Space::with_height(Length::Units(DEFAULT_PADDING)))
            .push(button_row),
    );

    let content = Container::new(row)
        .center_x()
        .width(Length::Fill)
        .height(Length::Shrink)
        .style(grin_gui_core::theme::container::Container::NormalBackground(color_palette));

    let wrapper_column = Column::new()
        .height(Length::Fill)
        .push(header_container)
        .push(content);

    // Returns the final container.
    Container::new(wrapper_column).padding(iced::Padding::from([
        DEFAULT_PADDING, // top
        DEFAULT_PADDING, // right
        DEFAULT_PADDING, // bottom
        DEFAULT_PADDING, // left
    ]))
}
