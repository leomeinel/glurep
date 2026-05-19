pub(crate) mod prelude {
    pub(crate) use super::{SVG_SIZE, SvgData};
}

use std::{cmp::Ordering, collections::HashSet, ops::Range};

#[cfg(feature = "cli")]
use clap::ArgMatches;
use jiff::{
    Span,
    civil::{Date, Time},
};
use plotters::{
    prelude::*,
    style::full_palette::{CYAN_300, ORANGE_300, RED_300},
};

use crate::{deserialize::prelude::*, log::prelude::*, utils::prelude::*};

/// Size of the svg `(x, y)` in pixels.
///
/// This can be approximated to `mm` of the output pdf.
pub(crate) const SVG_SIZE: (u32, u32) = (640, 640);

/// Plotting font.
const PLOTTING_FONT: (&str, u32) = ("Helvetica", 14);
/// Spec of `x` axis in seconds.
const X_SPEC: Range<u32> = 0..(24 * 3600);
/// Description for the y axis.
const Y_DESC: &str = "mg/dL";

/// Config for plotting [`GlucoseReadingsMap`] to svg.
#[derive(Clone, Debug)]
pub struct PlotConfig {
    /// `y` axis spec in `mg/dL`.
    pub y_spec: Range<u32>,
    /// Number of labels for axes `(x, y)`.
    pub num_labels: (usize, usize),
    /// Label size for axes `(x, y)` in pixels.
    ///
    /// This can be approximated to `mm` of the output pdf.
    pub label_size: (u32, u32),

    /// Radius of a single plotted point in pixels.
    ///
    /// This can be approximated to `mm` of the output pdf.
    pub point_radius: u32,

    /// Glucose target range.
    ///
    /// ## Note
    ///
    /// - anything in range is displayed in [`CYAN_300`].
    /// - anything below the range is displayed in [`RED_300`].
    /// - anything above the range is displayed in [`ORANGE_300`].
    pub glucose_threshold: Range<u32>,
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
            y_spec: 40..310,
            num_labels: (6, 8),
            label_size: (20, 40),

            point_radius: 4,
            glucose_threshold: 80..180,
        }
    }
}
#[cfg(feature = "cli")]
impl From<&ArgMatches> for PlotConfig {
    fn from(matches: &ArgMatches) -> Self {
        let mut config = PlotConfig::default();
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

/// Relevant data for svg.
#[derive(PartialEq, Eq, PartialOrd)]
pub struct SvgData {
    pub date: Date,
    pub contents: String,
}
impl SvgData {
    fn new(date: &Date, contents: String) -> Self {
        Self {
            date: *date,
            contents,
        }
    }
}
impl Ord for SvgData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}

/// Plot [`GlucoseReadingsMap`] to a sorted [`Vec<SvgData>`].
pub fn plot_to_strings(
    readings_map: &GlucoseReadingsMap,
    config: &PlotConfig,
) -> Result<Vec<SvgData>, anyhow::Error> {
    let mut svgs = Vec::new();
    for (date, readings) in &readings_map.0 {
        let mut svg = String::new();

        {
            let backend = SVGBackend::with_string(&mut svg, SVG_SIZE);
            let root = backend.into_drawing_area();
            root.fill(&WHITE)?;

            let mut chart = ChartBuilder::on(&root)
                .x_label_area_size(config.label_size.0)
                .y_label_area_size(config.label_size.1)
                .build_cartesian_2d(X_SPEC, config.y_spec.clone())?;
            chart
                .configure_mesh()
                .label_style(PLOTTING_FONT)
                // FIXME: We should use fixed positions/deltas for labels. I however haven't found a way to do that.
                .x_labels(config.num_labels.0)
                // FIXME: Returning a Result here would be ideal, but `ChartContext::format_x` can only be modified with this.
                .x_label_formatter(&|x| {
                    Time::midnight()
                        .checked_add(Span::new().seconds(*x))
                        .expect(PLOT_ERR_INVALID_TIME)
                        .strftime("%H:%M")
                        .to_string()
                })
                .y_labels(config.num_labels.1)
                .y_desc(Y_DESC)
                .axis_desc_style(PLOTTING_FONT)
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

            root.present()?
        }

        svgs.push(SvgData::new(date, svg));
    }
    if svgs.is_empty() {
        return Err(PlotError::EmptySvgs.into());
    }
    svgs.sort();

    Ok(svgs)
}
