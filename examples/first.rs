use std::collections::HashMap;

use chrono::NaiveDate;
use hotplot::chart::line::data::PlotSettings;
use iced::{window, Canvas, Color, Container, Element, Length, Sandbox, Settings};

fn main() {
    let settings = Settings {
        antialiasing: true,
        window: window::Settings {
            min_size: Some((400, 400)),
            ..Default::default()
        },
        ..Default::default()
    };
    MyApp::run(settings).unwrap()
}

#[derive(Debug)]
struct MyAppMsg {}

struct MyApp {}

impl Sandbox for MyApp {
    type Message = MyAppMsg;

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        "Super Title".to_owned()
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let settings = hotplot::chart::line::data::Settings {
            title: "Test".to_owned(),
            title_color: Color::from_rgb8(200, 0, 0),
            ..Default::default()
        };
        let plot_settings1 = PlotSettings {
            ..Default::default()
        };
        let plot_settings2 = PlotSettings {
            color: Color::from_rgb8(0, 200, 0),
            ..Default::default()
        };
        let edges1: Vec<(NaiveDate, i32)> = vec![
            (NaiveDate::from_ymd(2020, 12, 7), 55),
            (NaiveDate::from_ymd(2020, 12, 9), -18),
            (NaiveDate::from_ymd(2020, 12, 11), 15),
        ];
        let edges2: Vec<(NaiveDate, i32)> = vec![
            (NaiveDate::from_ymd(2020, 12, 6), 11),
            (NaiveDate::from_ymd(2020, 12, 10), 117),
            (NaiveDate::from_ymd(2020, 12, 12), 12),
        ];
        let mut data = HashMap::new();
        data.insert(plot_settings1, edges1);
        data.insert(plot_settings2, edges2);
        let min_x_value = NaiveDate::from_ymd(2020, 12, 4);
        let max_x_value = NaiveDate::from_ymd(2020, 12, 13);
        let custom_y_labels = vec![
            (0, Some("Lol!".to_string())),
            (50, Some("Kek!!".to_string())),
            (100, Some("Top!!!".to_string())),
        ];
        let line = hotplot::chart::line::ChartBuilder::new(settings)
            .data(data)
            .min_x_value(min_x_value)
            .max_x_value(max_x_value)
            .calculate_min_max_y_values()
            .custom_y_labels(custom_y_labels)
            .build();

        let canvas = Canvas::new(line).width(Length::Fill).height(Length::Fill);
        let container = Container::new(canvas)
            .width(Length::Fill)
            .height(Length::Fill);
        let elem: Element<_> = container.into();
        elem.map(|_| MyAppMsg {})
    }
}
