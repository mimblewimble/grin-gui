extern crate iced;
extern crate plotters;

use crate::gui::{element::DEFAULT_PADDING, Message};
use chrono::{DateTime, Utc};
use grin_gui_core::theme::{
    Button, Column, Container, Element, Header, PickList, Row, Scrollable, TableRow, Text,
    TextInput, Theme,
};
use iced::{
    alignment::{Horizontal, Vertical},
    executor,
    widget::{
        canvas::{self, event, Cache, Cursor, Frame, Geometry},
        Space,
    },
    Alignment, Command, Font, Length, Point, Settings, Size, Subscription,
};
use plotters::{
    coord::{types::RangedCoordf32, ReverseCoordTranslate},
    prelude::*,
};
use plotters_backend::DrawingBackend;
use plotters_iced::{Chart, ChartWidget};
use std::time::{Duration, Instant};
use std::{borrow::Borrow, collections::VecDeque};

const CHART_CAPTION_HEAD: u16 = 20;
const CHART_CAPTION_SUB: u16 = 12;

const FONT_REGULAR: Font = Font::External {
    name: "sans-serif-regular",
    bytes: include_bytes!("../../../../../fonts/notosans-regular.ttf"),
};

const FONT_BOLD: Font = Font::External {
    name: "sans-serif-bold",
    bytes: include_bytes!("../../../../../fonts/notosans-bold.ttf"),
};

#[derive(Default)]
pub struct BalanceChart {
    data_points: VecDeque<(DateTime<Utc>, f64)>,
    cursor_index: Option<usize>,
    theme: Theme,
}

impl BalanceChart {
    /// Create a new chart widget
    /// `data` is an iterator of `(DateTime<Utc>, f64)` tuples in descending order - newest datetime first
    pub fn new(
        theme: Theme,
        data: impl Iterator<Item = (DateTime<Utc>, f64)>,
        cursor_index: Option<usize>,
    ) -> Element<'static, Message> {
        let data_points: VecDeque<_> = data.collect();
        let chart = BalanceChart {
            data_points,
            theme,
            cursor_index,
        };

        Container::new(
            Column::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(5)
                .push(
                    ChartWidget::new(chart).height(Length::Fill).resolve_font(
                        |_, style| match style {
                            plotters_backend::FontStyle::Bold => FONT_BOLD,
                            _ => FONT_REGULAR,
                        },
                    ),
                ),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
    }

    pub fn push_data(&mut self, time: DateTime<Utc>, value: f64) {
        self.data_points.push_front((time, value));
    }
}

impl Chart<Message> for BalanceChart {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: canvas::Event,
        bounds: iced::Rectangle,
        cursor: canvas::Cursor,
    ) -> (iced_native::event::Status, Option<Message>) {
        if let Cursor::Available(point) = cursor {
            match event {
                canvas::Event::Mouse(_evt) if bounds.contains(point) => {
                    let p_origin = bounds.position();
                    let p = point - p_origin;
                    let percent = p.x / bounds.width;

                    let len = self.data_points.len() - 1;
                    let approx_index = len as f32 * percent;
                    let index = len.saturating_sub(approx_index.floor() as usize);

                    return (
                        iced_native::event::Status::Captured,
                        Some(Message::Interaction(
                            crate::gui::Interaction::WalletOperationHomeViewInteraction(
                                super::home::LocalViewInteraction::MouseIndex(index),
                            ),
                        )),
                    );
                }
                _ => {
                    return (
                        iced_native::event::Status::Captured,
                        Some(Message::Interaction(
                            crate::gui::Interaction::WalletOperationHomeViewInteraction(
                                super::home::LocalViewInteraction::MouseExit,
                            ),
                        )),
                    );
                }
            }
        }
        (event::Status::Ignored, None)
    }

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut chart: ChartBuilder<DB>) {
        use plotters::{prelude::*, style::Color};

        const PLOT_LINE_COLOR: RGBColor = RGBColor(0, 175, 255);

        // Acquire time range
        let newest_time = self
            .data_points
            .front()
            .unwrap_or(&(chrono::Utc::now(), 0.0))
            .0;

        let mut oldest_time = self
            .data_points
            .back()
            .unwrap_or(&(chrono::Utc::now() - chrono::Duration::days(7), 0.0))
            .0;

        if newest_time == oldest_time {
            oldest_time = chrono::Utc::now() - chrono::Duration::days(7);
        }

        // get largest amount from data points
        let mut max_value = 0.0;
        for (_, amount) in self.data_points.iter() {
            if *amount > max_value {
                max_value = *amount;
            }
        }
        // we add 10% to the max value to make sure the chart is not cut off
        max_value = max_value * 1.1;

        let mut chart = chart
            .x_label_area_size(6)
            .y_label_area_size(0)
            .build_cartesian_2d(oldest_time..newest_time, 0.0_f64..max_value)
            .expect("failed to build chart");

        let chart_color = self.theme.palette.bright.primary;
        let chart_color = RGBColor(
            (chart_color.r * 255.0) as u8,
            (chart_color.g * 255.0) as u8,
            (chart_color.b * 255.0) as u8,
        );

        let date_color = self.theme.palette.normal.surface;
        let date_color = RGBColor(
            (date_color.r * 255.0) as u8,
            (date_color.g * 255.0) as u8,
            (date_color.b * 255.0) as u8,
        );

        let background_color = self.theme.palette.base.background;
        let background_color = RGBColor(
            (background_color.r * 255.0) as u8,
            (background_color.g * 255.0) as u8,
            (background_color.b * 255.0) as u8,
        );

        let text_color = self.theme.palette.bright.surface;
        let text_color = RGBColor(
            (text_color.r * 255.0) as u8,
            (text_color.g * 255.0) as u8,
            (text_color.b * 255.0) as u8,
        );

        chart
            .configure_mesh()
            .bold_line_style(background_color)
            .light_line_style(background_color)
            .axis_style(background_color)
            .x_labels(4)
            .x_label_style(
                ("sans-serif", 15)
                    .into_font()
                    .color(&date_color)
                    .transform(FontTransform::Rotate90),
            )
            .x_label_formatter(&|x| format!("{}", x.format("%b %d, %Y")))
            .draw()
            .expect("failed to draw chart mesh");

        chart
            .draw_series(
                AreaSeries::new(
                    self.data_points.iter().map(|x| (x.0, x.1 as f64)),
                    0.0,
                    chart_color.mix(0.075),
                )
                .border_style(ShapeStyle::from(chart_color).stroke_width(2)),
            )
            .expect("failed to draw chart data");

        if let Some(cursor) = self.cursor_index {
            let (time, amount) = self.data_points[cursor];
            //debug!("index: {}, time: {}, amount: {}", index, time, amount);

            // draws a circle at (date, balance) point of the chart
            chart
                .draw_series(std::iter::once(Circle::new(
                    (time, amount),
                    5_i32,
                    chart_color.filled(),
                )))
                .expect("Failed to draw hover point");

            // draw balance above the point
            chart
                .draw_series(std::iter::once(Text::new(
                    format!("{}", amount),
                    (time, max_value),
                    ("sans-serif", CHART_CAPTION_HEAD)
                        .into_font()
                        .color(&text_color.mix(1.0))
                )))
                .expect("Failed to draw text");

            // date below balance with a slight faded color
            chart
                .draw_series(std::iter::once(Text::new(
                    format!("{}", time.format("%b %d, %Y")),
                    (time, max_value * 0.84),
                    ("sans-serif", CHART_CAPTION_SUB)
                        .into_font()
                        .color(&text_color.mix(0.7))
                )))
                .expect("Failed to draw text");
        }
    }
}
