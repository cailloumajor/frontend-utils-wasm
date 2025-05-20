use std::iter::{successors, zip};

use chrono::serde::ts_seconds;
use chrono::{DateTime, Duration, DurationRound, Local, Utc};
use colorsys::Rgb;
use plotters::coord::combinators::WithKeyPoints;
use plotters::prelude::*;
use plotters::style::RelativeSize;
use plotters_canvas::CanvasBackend;
use serde::Deserialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Event, HtmlCanvasElement};

use crate::errors::TimelineError;

#[derive(Deserialize)]
struct Slot {
    #[serde(with = "ts_seconds")]
    start_time: DateTime<Utc>,
    color_index: Option<usize>,
}

#[derive(Deserialize, Tsify)]
#[tsify(from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    palette: Vec<String>,
    font_family: String,
    opacity: f64,
    x_interval_minutes: u16,
    x_offset_minutes: u16,
    emphasis_labels: Vec<String>,
}

#[wasm_bindgen]
pub struct Timeline {
    canvas: HtmlCanvasElement,
    config: Config,
    palette: Vec<RGBAColor>,
}

#[wasm_bindgen]
impl Timeline {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement, config: Config) -> Result<Timeline, JsError> {
        let palette = config
            .palette
            .iter()
            .map(|hex| {
                let rgb = Rgb::from_hex_str(hex)
                    .map_err(|err| TimelineError::PaletteColorParsing(hex.to_string(), err))?;
                let (r, g, b) = rgb.into();
                Ok(RGBColor(r, g, b).mix(config.opacity))
            })
            .collect::<Result<_, TimelineError>>()?;

        Ok(Self {
            canvas,
            config,
            palette,
        })
    }

    pub async fn draw(&self, data: &[u8]) -> Result<(), JsError> {
        let axis_color = {
            let rgb: Rgb = web_sys::window()
                .unwrap_throw()
                .get_computed_style(&self.canvas)
                .unwrap_throw()
                .unwrap_throw()
                .get_property_value("color")
                .unwrap_throw()
                .parse()
                .map_err(TimelineError::ParsingCanvasColor)?;
            let (r, g, b) = rgb.into();
            RGBColor(r, g, b)
        };

        self.canvas
            .get_context("2d")
            .unwrap_throw()
            .ok_or(TimelineError::GetCanvasContext)?
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap_throw()
            .clear_rect(
                0.0,
                0.0,
                self.canvas.width().into(),
                self.canvas.height().into(),
            );

        let slots: Vec<Slot> = rmp_serde::from_slice(data).map_err(TimelineError::MsgPackDecode)?;

        let backend = CanvasBackend::with_canvas_object(self.canvas.clone())
            .ok_or(TimelineError::BackendCreation)?;
        let root = backend.into_drawing_area();

        let (Some(first), Some(last)) = (slots.first(), slots.last()) else {
            return Err(TimelineError::EmptyDataset.into());
        };
        let x_range = make_x_spec(
            first.start_time.into(),
            last.start_time.into(),
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
            .x_label_formatter(&|label| {
                let formatted = format!("{}", label.format("%H:%M"));
                if self.config.emphasis_labels.contains(&formatted) {
                    format!("<{formatted}>")
                } else {
                    formatted
                }
            })
            .disable_y_mesh()
            .disable_y_axis()
            .draw()?;

        let series: Vec<Rectangle<(DateTime<Local>, usize)>> = slots
            .windows(2)
            .filter_map(|window| {
                let start = &window[0];
                let end = &window[1];
                if let Some(index) = start.color_index {
                    let Some(color) = self.palette.get(index).cloned() else {
                        return Some(Err(TimelineError::ColorIndexNotInPalette(index)));
                    };
                    let style = ShapeStyle {
                        color,
                        filled: true,
                        stroke_width: 0,
                    };
                    Some(Ok(Rectangle::new(
                        [(start.start_time.into(), 1), (end.start_time.into(), 9)],
                        style,
                    )))
                } else {
                    None
                }
            })
            .collect::<Result<_, _>>()?;

        chart.draw_series(series)?;
        root.present()?;

        let drawed_event = Event::new("drawed").unwrap_throw();
        self.canvas.dispatch_event(&drawed_event).unwrap_throw();
        Ok(())
    }
}

fn make_x_spec(
    start: DateTime<Local>,
    end: DateTime<Local>,
    bold_interval: Duration,
    offset: Duration,
) -> WithKeyPoints<RangedDateTime<DateTime<Local>>> {
    let initial = start.duration_trunc(bold_interval).unwrap_throw() + offset - bold_interval;
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
                vec![
                    "22:00", "23:00", "00:00", "01:00", "02:00", "03:00", "04:00"
                ]
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
                vec![
                    "22:30", "23:30", "00:30", "01:30", "02:30", "03:30", "04:30"
                ]
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
                vec![
                    "22:00", "23:00", "00:00", "01:00", "02:00", "03:00", "04:00"
                ]
            );
        }
    }
}
