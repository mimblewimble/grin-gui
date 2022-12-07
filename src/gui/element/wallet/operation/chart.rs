
extern crate iced;
extern crate plotters;
extern crate sysinfo;

use crate::gui::{Message, element::DEFAULT_PADDING};
use chrono::{DateTime, Utc};
use grin_gui_core::theme::{
    Button, Column, Container, Element, PickList, Row, Scrollable, Text, TextInput, Header, TableRow, Theme
};
use iced::{
    alignment::{Horizontal, Vertical},
    executor,
    widget::{
        canvas::{Cache, Frame, Geometry}, Space
    },
    Alignment, Application, Command, Font, Length, Settings, Size, Subscription,
};
use plotters::prelude::ChartBuilder;
use plotters_backend::DrawingBackend;
use plotters_iced::{Chart, ChartWidget};
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, System, SystemExt};

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

pub struct BalanceChart {
    cache: Cache,
    data_points: VecDeque<(DateTime<Utc>, f64)>,
    limit: Duration,
    theme: Theme,
}

impl BalanceChart {

    // data points should be presorted with the most recent first
    pub fn new(theme: Theme, data: impl Iterator<Item = (DateTime<Utc>, f64)>) -> Self {
        let data_points: VecDeque<_> = data.collect();
        Self {
            cache: Cache::new(),
            data_points,
            limit: Duration::from_secs(PLOT_SECONDS as u64),
            theme,
        }
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
        self.cache.clear();
    }

    pub fn view(&self, idx: usize) -> Element<Message> {
        Container::new(
            Column::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(5)
                .push(
                    ChartWidget::new(self).height(Length::Fill).resolve_font(
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
}

impl Chart<Message> for BalanceChart {
    type State = ();
    // fn update(
    //     &mut self,
    //     event: Event,
    //     bounds: Rectangle,
    //     cursor: Cursor,
    // ) -> (event::Status, Option<Message>) {
    //     self.cache.clear();
    //     (event::Status::Ignored, None)
    // }

    #[inline]
    fn draw<F: Fn(&mut Frame)>(&self, bounds: Size, draw_fn: F) -> Geometry {
        self.cache.draw(bounds, draw_fn)
    }

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut chart: ChartBuilder<DB>) {
        use plotters::{prelude::*, style::Color};

        const PLOT_LINE_COLOR: RGBColor = RGBColor(0, 175, 255);

        // Acquire time range
        let newest_time = self
            .data_points
            .front()
            .unwrap_or(&(
                chrono::DateTime::from_utc(
                    chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
                    chrono::Utc,
                ),
                0.0,
            ))
            .0;

        let oldest_time = self
            .data_points
            .back()
            .unwrap_or(&(
                chrono::DateTime::from_utc(
                    chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
                    chrono::Utc,
                ),
                0.0,
            ))
            .0;

        // TODO y spec max value
        let mut chart = chart
            .x_label_area_size(6)
            .y_label_area_size(0)
            //.margin(DEFAULT_PADDING as u32)
            .build_cartesian_2d(oldest_time..newest_time, 0.0_f64..500.0_f64)
            .expect("failed to build chart");
    
        let color =  self.theme.palette.bright.primary;
        let color = RGBColor((color.r * 255.0) as u8, (color.g  * 255.0) as u8, (color.b * 255.0) as u8);

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
    }
}