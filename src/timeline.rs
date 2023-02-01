use std::collections::HashMap;
use std::iter::{successors, zip};

use chrono::{DateTime, Duration, DurationRound, Local};
use colorsys::Rgb;
use csv::ReaderBuilder;
use itertools::{process_results, Itertools};
use plotters::coord::combinators::WithKeyPoints;
use plotters::prelude::*;
use plotters::style::RelativeSize;
use plotters_canvas::CanvasBackend;
use serde::Deserialize;
use tsify::Tsify;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, Event, HtmlCanvasElement};

use crate::errors::TimelineError;
use crate::identify_last::IdentifyLast;
use crate::influxdb::{self, InfluxdbConfig};
use crate::model::{Record, TimeRange};
use crate::utils;

#[derive(Deserialize, Tsify)]
#[tsify(from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    font_family: String,
    opacity: f64,
    x_interval_minutes: u16,
    x_offset_minutes: u16,
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

        let data = influxdb::get_data(&self.config.influxdb).await?;
        let mut reader = ReaderBuilder::new().comment(Some(b'#')).from_reader(data);

        let mut time_range_iter = reader.deserialize::<TimeRange>();
        let initial_position = time_range_iter.reader().position().clone();
        let time_range = time_range_iter
            .next()
            .ok_or(TimelineError::EmptyDataset)??;

        let backend = CanvasBackend::with_canvas_object(self.canvas.clone())
            .ok_or(TimelineError::BackendCreation)?;
        let root = backend.into_drawing_area();

        let x_range = make_x_spec(
            time_range.start,
            time_range.stop,
            Duration::minutes(self.config.x_interval_minutes.into()),
            Duration::minutes(self.config.x_offset_minutes.into()),
        );

        let mut chart = ChartBuilder::on(&root)
            .margin(RelativeSize::Height(0.03))
            .margin_left(RelativeSize::Height(0.13))
            .margin_right(RelativeSize::Height(0.1))
            .x_label_area_size(RelativeSize::Height(0.13))
            .build_cartesian_2d(x_range, 0usize..10)?;

        chart
            .configure_mesh()
            .axis_style(axis_color)
            .bold_line_style(axis_color.mix(0.5))
            .light_line_style(axis_color.mix(0.2))
            .label_style((
                FontFamily::Name(&self.config.font_family),
                RelativeSize::Height(0.12),
                &axis_color,
            ))
            .x_label_formatter(&|label| format!("{}", label.format("%H:%M")))
            .disable_y_mesh()
            .disable_y_axis()
            .draw()?;

        let mut palette: HashMap<String, RGBAColor> = HashMap::new();

        reader.seek(initial_position)?;

        let deduplicated: Vec<_> = process_results(reader.deserialize::<Record>(), |iter| {
            iter.identify_last()
                .dedup_by(|(previous, _), (current, is_last)| {
                    !is_last && previous.color == current.color
                })
                .map(|(record, _)| record)
                .collect()
        })?;
        for hex in deduplicated
            .iter()
            .filter_map(|record| record.color.as_ref())
            .unique()
        {
            let parsed = Rgb::from_hex_str(hex)?;
            let (r, g, b) = parsed.into();
            palette.insert(hex.to_owned(), RGBColor(r, g, b).mix(self.config.opacity));
        }
        let series = deduplicated
            .iter()
            .tuple_windows()
            .filter_map(|(start, end)| {
                if let Some(color) = &start.color {
                    let style = ShapeStyle {
                        color: palette[color],
                        filled: true,
                        stroke_width: 0,
                    };
                    Some(Rectangle::new([(start.time, 1), (end.time, 9)], style))
                } else {
                    None
                }
            });

        chart.draw_series(series)?;
        root.present()?;

        let drawed_event = Event::new("drawed").unwrap();
        self.canvas.dispatch_event(&drawed_event).unwrap();
        Ok(())
    }
}

fn make_x_spec(
    start: DateTime<Local>,
    end: DateTime<Local>,
    bold_interval: Duration,
    offset: Duration,
) -> WithKeyPoints<RangedDateTime<DateTime<Local>>> {
    let initial = start.duration_trunc(bold_interval).unwrap() + offset - bold_interval;
    let mut bold_points = Vec::new();
    let mut light_points = Vec::new();
    let is_bold_iter = successors(Some(true), |is_bold| Some(!is_bold));
    let datetime_iter = successors(Some(initial), |dt| Some(*dt + bold_interval / 2));
    let zipped = zip(is_bold_iter, datetime_iter);
    for (is_bold, dt) in zipped {
        if dt < start {
            continue;
        }
        if dt > end {
            break;
        }
        if is_bold {
            bold_points.push(dt);
        } else {
            light_points.push(dt);
        }
    }
    (start..end)
        .with_key_points(bold_points)
        .with_light_points(light_points)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod make_x_spec {
        use chrono::TimeZone;
        use plotters::coord::ranged1d::{BoldPoints, LightPoints};

        use super::*;

        trait FormattedKeyPoints {
            fn formatted_bold_points(&self) -> Vec<String>;
            fn formatted_light_points(&self) -> Vec<String>;
        }

        impl FormattedKeyPoints for WithKeyPoints<RangedDateTime<DateTime<Local>>> {
            fn formatted_bold_points(&self) -> Vec<String> {
                self.bold_points()
                    .iter()
                    .map(|dt| dt.format("%H:%M").to_string())
                    .collect()
            }
            fn formatted_light_points(&self) -> Vec<String> {
                self.light_points()
                    .iter()
                    .map(|dt| dt.format("%H:%M").to_string())
                    .collect()
            }
        }

        #[test]
        fn start_on_bold_line_no_offset() {
            let start = Local.with_ymd_and_hms(1984, 12, 9, 22, 0, 0).unwrap();
            let end = Local.with_ymd_and_hms(1984, 12, 10, 4, 0, 0).unwrap();
            let interval = Duration::hours(1);
            let offset = Duration::zero();

            let x_spec = make_x_spec(start, end, interval, offset);

            assert_eq!(
                x_spec.formatted_bold_points(),
                vec!["22:00", "23:00", "00:00", "01:00", "02:00", "03:00", "04:00"]
            );
            assert_eq!(
                x_spec.formatted_light_points(),
                vec!["22:30", "23:30", "00:30", "01:30", "02:30", "03:30"]
            );
        }

        #[test]
        fn start_on_light_line_no_offset() {
            let start = Local.with_ymd_and_hms(1984, 12, 9, 22, 30, 0).unwrap();
            let end = Local.with_ymd_and_hms(1984, 12, 10, 4, 30, 0).unwrap();
            let interval = Duration::hours(1);
            let offset = Duration::zero();

            let x_spec = make_x_spec(start, end, interval, offset);

            assert_eq!(
                x_spec.formatted_bold_points(),
                vec!["23:00", "00:00", "01:00", "02:00", "03:00", "04:00"]
            );
            assert_eq!(
                x_spec.formatted_light_points(),
                vec!["22:30", "23:30", "00:30", "01:30", "02:30", "03:30", "04:30"]
            );
        }

        #[test]
        fn start_after_bold_line_no_offset() {
            let start = Local.with_ymd_and_hms(1984, 12, 9, 22, 5, 0).unwrap();
            let end = Local.with_ymd_and_hms(1984, 12, 10, 4, 5, 0).unwrap();
            let interval = Duration::hours(1);
            let offset = Duration::zero();

            let x_spec = make_x_spec(start, end, interval, offset);

            assert_eq!(
                x_spec.formatted_bold_points(),
                vec!["23:00", "00:00", "01:00", "02:00", "03:00", "04:00"]
            );
            assert_eq!(
                x_spec.formatted_light_points(),
                vec!["22:30", "23:30", "00:30", "01:30", "02:30", "03:30"]
            );
        }

        #[test]
        fn start_after_light_line_no_offset() {
            let start = Local.with_ymd_and_hms(1984, 12, 9, 22, 35, 0).unwrap();
            let end = Local.with_ymd_and_hms(1984, 12, 10, 4, 35, 0).unwrap();
            let interval = Duration::hours(1);
            let offset = Duration::zero();

            let x_spec = make_x_spec(start, end, interval, offset);

            assert_eq!(
                x_spec.formatted_bold_points(),
                vec!["23:00", "00:00", "01:00", "02:00", "03:00", "04:00"]
            );
            assert_eq!(
                x_spec.formatted_light_points(),
                vec!["23:30", "00:30", "01:30", "02:30", "03:30", "04:30"]
            );
        }

        #[test]
        fn with_offset() {
            let start = Local.with_ymd_and_hms(1984, 12, 9, 22, 0, 0).unwrap();
            let end = Local.with_ymd_and_hms(1984, 12, 10, 4, 0, 0).unwrap();
            let interval = Duration::hours(1);
            let offset = Duration::minutes(30);

            let x_spec = make_x_spec(start, end, interval, offset);

            assert_eq!(
                x_spec.formatted_bold_points(),
                vec!["22:30", "23:30", "00:30", "01:30", "02:30", "03:30"]
            );
            assert_eq!(
                x_spec.formatted_light_points(),
                vec!["22:00", "23:00", "00:00", "01:00", "02:00", "03:00", "04:00"]
            );
        }

        #[test]
        fn playground() {
            let start = Local.with_ymd_and_hms(1984, 12, 9, 22, 0, 0).unwrap();
            let end = Local.with_ymd_and_hms(1984, 12, 10, 4, 0, 0).unwrap();
            let ranged = RangedDateTime::from(start..end);
            dbg!(ranged.key_points(BoldPoints(8)));
            dbg!(ranged.key_points(LightPoints::new(8, 1)));
        }
    }
}
