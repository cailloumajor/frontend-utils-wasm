mod errors;
mod iter;
mod model;
mod utils;

use std::collections::HashMap;
use std::io::Cursor;

use colorsys::Rgb;
use csv::ReaderBuilder;
use gloo_net::http::Request;
use itertools::{process_results, Itertools};
use plotters::prelude::*;
use plotters::style::RelativeSize;
use plotters_canvas::CanvasBackend;
use serde::Deserialize;
use tsify::Tsify;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, Event, HtmlCanvasElement};

use errors::TimelineError;
use iter::IdentifyLast;
use model::{Record, TimeRange};

#[derive(Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
struct InfluxdbConfig {
    influxdb_url: String,
    influxdb_org: String,
    influxdb_token: String,
    flux_query: String,
}

#[derive(Deserialize, Tsify)]
#[tsify(from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    font_family: String,
    #[serde(flatten)]
    influxdb: InfluxdbConfig,
}

#[wasm_bindgen]
pub struct Timeline {
    canvas: HtmlCanvasElement,
    config: Config,
}

#[wasm_bindgen]
impl Timeline {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement, config: Config) -> Self {
        utils::set_panic_hook();

        Self { canvas, config }
    }

    pub async fn draw(&self) -> Result<(), JsError> {
        let axis_color = {
            let rgb = web_sys::window()
                .unwrap()
                .get_computed_style(&self.canvas)
                .unwrap()
                .unwrap()
                .get_property_value("color")
                .unwrap()
                .parse::<Rgb>()
                .unwrap();
            let (r, g, b) = rgb.into();
            RGBColor(r, g, b)
        };

        self.canvas
            .get_context("2d")
            .unwrap()
            .ok_or(TimelineError::GetCanvasContext)?
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap()
            .clear_rect(
                0.0,
                0.0,
                self.canvas.width().into(),
                self.canvas.height().into(),
            );

        let data = get_data(&self.config.influxdb).await?;
        let mut reader = ReaderBuilder::new().comment(Some(b'#')).from_reader(data);

        let mut time_range_iter = reader.deserialize::<TimeRange>();
        let initial_position = time_range_iter.reader().position().clone();
        let time_range = time_range_iter
            .next()
            .ok_or(TimelineError::EmptyDataset)??;

        let backend = CanvasBackend::with_canvas_object(self.canvas.clone())
            .ok_or(TimelineError::BackendCreation)?;
        let root = backend.into_drawing_area();

        let x_range = time_range.start..time_range.stop;
        let mut chart = ChartBuilder::on(&root)
            .margin(RelativeSize::Height(0.03))
            .margin_left(RelativeSize::Height(0.13))
            .margin_right(RelativeSize::Height(0.1))
            .x_label_area_size(RelativeSize::Height(0.13))
            .build_cartesian_2d(x_range, 0usize..10)?;

        chart
            .configure_mesh()
            .axis_style(axis_color)
            .bold_line_style(axis_color.mix(0.4))
            .light_line_style(axis_color.mix(0.3))
            .label_style((
                FontFamily::Name(&self.config.font_family),
                RelativeSize::Height(0.12),
                &axis_color,
            ))
            .x_label_formatter(&|label| format!("{}", label.format("%H:%M")))
            .x_labels(13)
            .x_max_light_lines(2)
            .disable_y_mesh()
            .disable_y_axis()
            .draw()?;

        let mut palette: HashMap<String, RGBColor> = HashMap::new();

        reader.seek(initial_position)?;

        let deduplicated: Vec<_> = process_results(reader.deserialize::<Record>(), |iter| {
            iter.identify_last()
                .dedup_by(|(previous, _), (current, is_last)| {
                    !is_last && previous.color == current.color
                })
                .map(|(record, _)| record)
                .collect()
        })?;
        for hex in deduplicated.iter().map(|record| &record.color).unique() {
            let parsed = Rgb::from_hex_str(hex)?;
            let (r, g, b) = parsed.into();
            palette.insert(hex.to_owned(), RGBColor(r, g, b));
        }
        let series = deduplicated.iter().tuple_windows().map(|(start, end)| {
            let style = ShapeStyle {
                color: palette[&start.color].into(),
                filled: true,
                stroke_width: 0,
            };
            Rectangle::new([(start.time, 1), (end.time, 9)], style)
        });

        chart.draw_series(series)?;
        root.present()?;

        let drawed_event = Event::new("drawed").unwrap();
        self.canvas.dispatch_event(&drawed_event).unwrap();
        Ok(())
    }
}

async fn get_data(config: &InfluxdbConfig) -> Result<Cursor<Vec<u8>>, JsError> {
    let token = format!("Token {}", config.influxdb_token);
    let request = Request::post(&config.influxdb_url)
        .query([("org", &config.influxdb_org)])
        .header("Accept", "application/csv")
        .header("Authorization", &token)
        .header("Content-Type", "application/vnd.flux")
        .body(&config.flux_query);
    let response = request.send().await?;
    if !response.ok() {
        return Err(TimelineError::ResponseStatus(response.status()).into());
    }
    let data = response.binary().await?;
    Ok(Cursor::new(data))
}
