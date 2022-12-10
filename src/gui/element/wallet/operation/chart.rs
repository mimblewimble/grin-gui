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

const PLOT_SECONDS: usize = 60; //1 min
const TITLE_FONT_SIZE: u16 = 22;
const SAMPLE_EVERY: Duration = Duration::from_millis(1000);

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
    current_position: Option<f32>,
    theme: Theme,
}

// impl Default for BalanceChart {
//     fn default() -> Self {
//         Self {
//             data_points: VecDeque::default(),
//             theme: Theme::default(),
//             current_position: None,
//         }
//     }
// }

impl BalanceChart {
    /// Create a new chart widget
    /// `data` is an iterator of `(DateTime<Utc>, f64)` tuples in descending order - newest datetime first
    pub fn new(
        theme: Theme,
        data: impl Iterator<Item = (DateTime<Utc>, f64)>,
        current_position: Option<f32>,
    ) -> Element<'static, Message> {
        let data_points: VecDeque<_> = data.collect();
        let chart = BalanceChart {
            data_points,
            theme,
            current_position,
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
        let cur_ms = time.timestamp_millis();
        self.data_points.push_front((time, value));
        // loop {
        //     if let Some((time, _)) = self.data_points.back() {
        //         let diff = Duration::from_millis((cur_ms - time.timestamp_millis()) as u64);
        //         if diff > self.limit {
        //             self.data_points.pop_back();
        //             continue;
        //         }
        //     }
        //     break;
        // }
        //self.cache.clear();
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
                canvas::Event::Mouse(evt) if bounds.contains(point) => {
                    let p_origin = bounds.position();
                    let p = point - p_origin;
                    let percent = p.x / bounds.width;
                    return (
                        iced_native::event::Status::Captured,
                        Some(Message::Interaction(
                            crate::gui::Interaction::WalletOperationHomeViewInteraction(
                                super::home::LocalViewInteraction::MouseEvent(
                                    evt, percent,
                                    //Point::new(p.x, p.y),
                                ),
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

    // #[inline]
    // fn draw<F: Fn(&mut Frame)>(&self, bounds: Size, draw_fn: F) -> Geometry {
    //     //self.cache.draw(bounds, draw_fn)
    // }

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
        let mut largest_amount = 0.0;
        for (_, amount) in self.data_points.iter() {
            if *amount > largest_amount {
                largest_amount = *amount;
            }
        }

        // TODO y spec max value
        let mut chart = chart
            .x_label_area_size(6)
            .y_label_area_size(0)
            //.margin(DEFAULT_PADDING as u32)
            .build_cartesian_2d(oldest_time..newest_time, 0.0_f64..(largest_amount * 1.1))
            .expect("failed to build chart");

        let color = self.theme.palette.bright.primary;
        let color = RGBColor(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
        );

        chart
            .configure_mesh()
            //.bold_line_style(plotters::style::colors::BLUE.mix(0.0001))
            //.light_line_style(plotters::style::colors::BLUE.mix(0.005))
            //.axis_style(ShapeStyle::from(plotters::style::colors::BLUE.mix(0.45)).stroke_width(1))
            //.y_labels(4)
            .x_labels(4)
            .x_label_style(
                ("sans-serif", 15)
                    .into_font()
                    .color(&color.mix(0.7))
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
                    color.mix(0.075),
                )
                .border_style(ShapeStyle::from(color).stroke_width(2)),
            )
            .expect("failed to draw chart data");

        if let Some(current_p) = self.current_position {
            let len = self.data_points.len() - 1;
            let approx_index = len as f32 * current_p;
            let index = len.saturating_sub(approx_index.floor() as usize);
            let (time, amount) = self.data_points[index];
            debug!("index: {}, time: {}, amount: {}", index, time, amount);

            chart
                .draw_series(std::iter::once(Circle::new(
                    (time, amount),
                    5_i32,
                    color.filled(),
                )))
                .expect("Failed to draw hover point");

            chart
                .draw_series(std::iter::once(Text::new(
                    format!("{}", amount),
                    (time, amount + 1000.0),
                    ("sans-serif", 20).into_font().color(&plotters::style::colors::
                        WHITE.mix(1.0)),
                )))
                .expect("Failed to draw text");
        }
    }
}
