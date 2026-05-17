pub(crate) mod prelude {
    pub(crate) use super::{PlotConfig, plot_to_svg};
}

use std::{collections::HashSet, error::Error, ops::Range, path::PathBuf};

use clap::ArgMatches;
use jiff::{Span, civil::Time};
use plotters::{prelude::*, style::full_palette::*};

use crate::{deserialize::prelude::*, utils::prelude::*};

/// Error if an invalid time is processed from the csv.
const ERR_INVALID_TIME: &str = "Invalid time. Aborting.";

/// Chart margin in pixels.
const CHART_MARGIN: u32 = 10;
/// Spec of `x` axis in seconds.
const X_SPEC: Range<u32> = 0..(24 * 3600);
/// Description for the y axis.
const Y_DESC: &str = "mg/dL";

/// Config for plotting [`GlucoseReadingsMap`] to svg.
#[derive(Clone, Debug)]
pub(crate) struct PlotConfig {
    /// Size of the drawing area `(x, y)` in pixels.
    ///
    /// This is equivalent to the size of the resulting svg.
    pub(crate) size: (u32, u32),

    /// Spec of `y` axis in `mg/dL`.
    pub(crate) y_spec: Range<u32>,
    /// Number of labels for axes `(x, y)`.
    pub(crate) num_labels: (usize, usize),
    /// Size of labels for axes `(x, y)` in pixels.
    pub(crate) label_size: (u32, u32),

    /// Caption font size.
    pub(crate) caption_font_size: u32,

    /// Radius of a single plotted point in pixels.
    pub(crate) point_radius: u32,

    /// Glucose target range.
    ///
    /// ## Note
    ///
    /// - anything in range is displayed in [`CYAN_300`].
    /// - anything below the range is displayed in [`RED_300`].
    /// - anything above the range is displayed in [`ORANGE_300`].
    pub(crate) glucose_threshold: Range<u32>,
}
impl PlotConfig {
    fn measurement_color(&self, measurement: u32) -> RGBColor {
        if measurement <= self.glucose_threshold.start {
            return RED_300;
        } else if measurement < self.glucose_threshold.end {
            return CYAN_300;
        } else {
            return ORANGE_300;
        }
    }
}
impl Default for PlotConfig {
    fn default() -> Self {
        Self {
            size: (640, 480),

            y_spec: 0..300,
            num_labels: (6, 8),
            label_size: (20, 40),

            caption_font_size: 18,

            point_radius: 4,
            glucose_threshold: 80..180,
        }
    }
}
impl From<&ArgMatches> for PlotConfig {
    fn from(matches: &ArgMatches) -> Self {
        let mut config = PlotConfig::default();
        matches.get_one::<u32>("size_x").map(|&x| config.size.0 = x);
        matches.get_one::<u32>("size_y").map(|&y| config.size.1 = y);
        matches
            .get_one::<u32>("min_y_spec")
            .map(|&y| config.y_spec.start = y);
        matches
            .get_one::<u32>("max_y_spec")
            .map(|&y| config.y_spec.end = y);
        matches
            .get_one::<usize>("num_labels_x")
            .map(|&x| config.num_labels.0 = x);
        matches
            .get_one::<usize>("num_labels_y")
            .map(|&y| config.num_labels.1 = y);
        matches
            .get_one::<u32>("label_size_x")
            .map(|&x| config.label_size.0 = x);
        matches
            .get_one::<u32>("label_size_y")
            .map(|&y| config.label_size.1 = y);
        matches
            .get_one::<u32>("caption_font_size")
            .map(|&s| config.caption_font_size = s);
        matches
            .get_one::<u32>("point_radius")
            .map(|&r| config.point_radius = r);
        matches
            .get_one::<u32>("min_glucose_threshold")
            .map(|&l| config.glucose_threshold.start = l);
        matches
            .get_one::<u32>("max_glucose_threshold")
            .map(|&h| config.glucose_threshold.end = h);

        config
    }
}

/// Plot [`GlucoseReadingsMap`] to svg.
pub(crate) fn plot_to_svg(
    readings: GlucoseReadingsMap,
    output_path: &PathBuf,
    config: PlotConfig,
) -> Result<(), Box<dyn Error>> {
    for (date, readings) in readings.0 {
        let output_path = output_path.join(format!("{}.svg", date));
        let root = SVGBackend::new(&output_path, config.size).into_drawing_area();

        let caption = date.to_string();
        let mut chart = ChartBuilder::on(&root)
            .caption(
                caption,
                ("sans-serif", config.caption_font_size).into_font(),
            )
            .x_label_area_size(config.label_size.0)
            .y_label_area_size(config.label_size.1)
            .margin(CHART_MARGIN)
            .build_cartesian_2d(X_SPEC, config.y_spec.clone())?;
        chart
            .configure_mesh()
            // FIXME: We should use fixed positions/deltas for labels. I however haven't found a way to do that.
            .x_labels(config.num_labels.0)
            .x_label_formatter(&|x| {
                Time::midnight()
                    .checked_add(Span::new().seconds(*x))
                    .expect(ERR_INVALID_TIME)
                    .strftime("%H:%M")
                    .to_string()
            })
            .y_labels(config.num_labels.1)
            .y_desc(Y_DESC)
            .draw()?;

        let glucose_threshold = config.glucose_threshold.clone();
        let x_points = &[X_SPEC.start, X_SPEC.end];
        chart.draw_series(LineSeries::new(
            x_points.map(|x| (x, glucose_threshold.end)),
            config.measurement_color(glucose_threshold.end),
        ))?;
        chart.draw_series(LineSeries::new(
            x_points.map(|x| (x, glucose_threshold.start)),
            config.measurement_color(glucose_threshold.start),
        ))?;

        let readings: HashSet<_> = readings
            .iter()
            .map(|r| (num_seconds_from_midnight(&r.time), r.measurement))
            .collect();
        chart.draw_series(readings.iter().map(|(x, y)| {
            let color = config.measurement_color(*y);
            return EmptyElement::at((*x, *y))
                + Circle::new((0, 0), config.point_radius, color.filled());
        }))?;

        root.present()?;
    }

    Ok(())
}
