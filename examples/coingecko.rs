use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};
use coingecko_requests::data::MarketChart;
use hotplot::chart::line::data::PlotSettings;
use iced::{Application, Canvas, Color, Command, Container, Element, Length, Settings, window};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = coingecko_requests::caching_client::Client::new(coingecko_requests::api_client::Client::new()).await?;
    let from = NaiveDate::from_ymd(2016, 10, 10).and_hms(0, 0, 0).timestamp();
    let to = NaiveDate::from_ymd(2019, 10, 10).and_hms(0, 0, 0).timestamp();
    println!("From: {}", from);
    println!("To: {}", to);
    let chart = client.market_chart("bitcoin", "usd", from as u64, to as u64).await?;
    let mut settings = Settings::with_flags(MyFlags {
        chart
    });
    settings.antialiasing = true;
    settings.window = window::Settings {
        min_size: Some((400, 400)),
        ..Default::default()
    };
    let settings = settings;
    MyApp::run(settings).unwrap();
    Ok(())
}

struct MyFlags {
    chart: MarketChart
}

#[derive(Debug)]
struct MyAppMsg {}

struct MyApp {
    chart: MarketChart
}

impl Application for MyApp {
    type Message = MyAppMsg;
    type Executor = iced::executor::Default;
    type Flags = MyFlags;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self {
            chart: flags.chart
        }, Command::none())
    }
    
    fn title(&self) -> String {
        "Example CoinGecko data plot".to_owned()
    }

    fn update(
        &mut self,
        message: Self::Message,
        clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        todo!()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let settings = hotplot::chart::line::data::Settings {
            title: "Bitcoin plot".to_owned(),
            title_color: Color::from_rgb8(200, 0, 0),
            min_x_label_distance: hotplot::chart::line::data::DistanceValue::Fixed(200.0),
            ..Default::default()
        };
        let plot_settings1 = PlotSettings {
            ..Default::default()
        };
        let plot_settings2 = PlotSettings {
            color: Color::from_rgb8(0, 200, 0),
            ..Default::default()
        };
        let edges = self.chart.raw.prices
            .iter()
            .map(|(timestamp, value)| {
                (NaiveDateTime::from_timestamp((*timestamp / 1000) as i64, 0), *value)
            })
            .collect();
        let mut data = HashMap::new();
        data.insert(plot_settings1, edges);
        let line = hotplot::chart::line::ChartBuilder::new(settings)
            .data(data)
            .calculate_min_max_x_values()
            .calculate_min_max_y_values()
            .build();

        let canvas = Canvas::new(line).width(Length::Fill).height(Length::Fill);
        let container = Container::new(canvas)
            .width(Length::Fill)
            .height(Length::Fill);
        let elem: Element<_> = container.into();
        elem.map(|_| MyAppMsg {})
    }
}
