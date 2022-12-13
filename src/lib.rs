mod errors;
mod model;
mod utils;

use std::collections::HashMap;
use std::io::Cursor;

use csv::ReaderBuilder;
use gloo_net::http::Request;
use hex_color::HexColor;
use itertools::Itertools;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use serde::Deserialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use errors::DrawInternalError;
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
    canvas_id: String,
    #[serde(flatten)]
    influxdb: InfluxdbConfig,
}

#[wasm_bindgen]
pub struct Timeline;

#[wasm_bindgen]
impl Timeline {
    pub async fn draw(config: Config) -> Result<(), JsError> {
        utils::set_panic_hook();

        let data = {
            let _timer = gloo_console::Timer::new("getting InfluxDB data");
            get_data(&config.influxdb).await?
        };
        let mut reader = ReaderBuilder::new().comment(Some(b'#')).from_reader(data);

        let _drawing_timer = gloo_console::Timer::new("timeline drawing");

        let mut time_range_iter = reader.deserialize::<TimeRange>();
        let initial_position = time_range_iter.reader().position().clone();
        let time_range = time_range_iter
            .next()
            .ok_or(DrawInternalError::EmptyDataset)??;

        let backend =
            CanvasBackend::new(&config.canvas_id).ok_or(DrawInternalError::CanvasNotFound)?;
        let root = backend.into_drawing_area();

        let x_range = time_range.start..time_range.stop;
        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .x_label_area_size(15)
            .y_label_area_size(10)
            .build_cartesian_2d(x_range, 0usize..10)?;

        chart
            .configure_mesh()
            .x_label_formatter(&|label| format!("{}", label.format("%H:%M")))
            .x_labels(13)
            .x_max_light_lines(2)
            .disable_y_mesh()
            .disable_y_axis()
            .draw()?;

        let mut palette: HashMap<String, RGBColor> = HashMap::new();

        reader.seek(initial_position)?;

        let deduplicated: Vec<_> = reader
            .deserialize::<Record>()
            .dedup_by(|x, y| match (x, y) {
                (Ok(x), Ok(y)) => x.color == y.color,
                _ => false,
            })
            .try_collect()?;
        for hex in deduplicated.iter().map(|record| &record.color).unique() {
            let HexColor { r, g, b, .. } =
                HexColor::parse_rgb(hex).map_err(|err| DrawInternalError::HexColorParse {
                    source: err,
                    value: hex.clone(),
                })?;
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

        Ok(())
    }
}

async fn get_data(config: &InfluxdbConfig) -> Result<Cursor<Vec<u8>>, JsError> {
    let token = format!("Token {}", config.influxdb_token);
    let request = Request::post(&config.influxdb_url)
        .query([("org", &config.influxdb_org)])
        .header("Authorization", &token)
        .header("Content-Type", "application/vnd.flux")
        .body(&config.flux_query);
    let response = request.send().await?;
    let data = response.binary().await?;
    Ok(Cursor::new(data))
}
