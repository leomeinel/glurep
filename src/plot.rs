pub(crate) mod prelude {
    pub(crate) use super::{PlotConfig, plot_to_svg};
}

use std::{error::Error, ops::Range, path::PathBuf};

use chrono::{NaiveTime, Timelike};
use plotters::prelude::*;

use crate::deserialize::prelude::*;

/// Error if an invalid time is processed from the csv.
const ERR_INVALID_TIME: &str = "Invalid time. Aborting.";

/// Config for plotting [`GlucoseReadingsMap`] to svg.
#[derive(Clone, Debug)]
pub(crate) struct PlotConfig {
    /// Margin of the drawing area.
    margin: u32,
    /// Size of the drawing area `(x, y)`.
    ///
    /// This is equivalent to the size of the resulting svg.
    size: (u32, u32),

    /// Ranges of axes `(x, y)`.
    ranges: (Range<f32>, Range<u32>),
    /// Number of labels for axes `(x, y)`.
    num_labels: (usize, usize),
    /// Size of labels for axes `(x, y)` in pixels.
    label_size: (u32, u32),

    /// Font size for the caption.
    caption_font_size: u32,

    /// Radius of a single plotted point in pixels.
    point_radius: u32,

    /// Glucose limits `(low, high)`.
    ///
    /// ## Note
    ///
    /// - `low` will be displayed in [`RED`].
    /// - `high` will be displayed in [`YELLOW`].
    /// - anything else will be displayed in [`BLUE`].
    limits: (u32, u32),
}
impl Default for PlotConfig {
    fn default() -> Self {
        Self {
            margin: 10,
            size: (640, 480),

            ranges: (0_f32..24_f32, 0..300),
            num_labels: (6, 8),
            label_size: (20, 30),

            caption_font_size: 30,

            point_radius: 4,
            limits: (80, 180),
        }
    }
}

/// Plot [`GlucoseReadingsMap`] to svg.
pub(crate) fn plot_to_svg(
    readings: GlucoseReadingsMap,
    config: PlotConfig,
    output_dir: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    for (date, readings) in readings.0 {
        let output_path = output_dir.join(format!("{}.svg", date));
        let root = SVGBackend::new(&output_path, config.size).into_drawing_area();
        root.margin(config.margin, config.margin, config.margin, config.margin);

        let caption = date.to_string();
        let mut chart = ChartBuilder::on(&root)
            .caption(
                caption,
                ("sans-serif", config.caption_font_size).into_font(),
            )
            .x_label_area_size(config.label_size.0)
            .y_label_area_size(config.label_size.1)
            .build_cartesian_2d(config.ranges.0.clone(), config.ranges.1.clone())?;
        chart
            .configure_mesh()
            .x_labels(config.num_labels.0)
            .y_labels(config.num_labels.1)
            .x_label_formatter(&|x| {
                let seconds = (x * 3600.) as u32;
                NaiveTime::from_num_seconds_from_midnight_opt(seconds, 0)
                    .expect(ERR_INVALID_TIME)
                    .format("%H:%M")
                    .to_string()
            })
            .draw()?;

        let readings: Vec<_> = readings
            .iter()
            .map(|r| {
                let hours = r.time.num_seconds_from_midnight() as f32 / 3600.;
                (hours, r.measurement)
            })
            .collect();
        chart.draw_series(readings.iter().map(|(x, y)| {
            let (low, high) = config.limits;
            let color = if *y <= low {
                RED
            } else if *y <= high {
                BLUE
            } else {
                YELLOW
            };
            return EmptyElement::at((*x, *y))
                + Circle::new((0, 0), config.point_radius, color.filled());
        }))?;

        root.present()?;
    }

    Ok(())
}
