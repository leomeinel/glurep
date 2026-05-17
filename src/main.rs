use std::{env, error::Error, path::PathBuf};

use clap::{ArgMatches, arg, command, value_parser};

use crate::{deserialize::prelude::*, plot::prelude::*};

mod deserialize;
mod plot;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let args = arg_matches();

    let input_path = args.get_one::<PathBuf>("INPUT_FILE").unwrap();
    let readings = GlucoseReadingsMap::from_file_path(input_path)?;

    let output_path = args.get_one::<PathBuf>("OUTPUT_DIR").unwrap();
    let plot_config = PlotConfig::from(&args);
    plot_to_svg(readings, output_path, plot_config)
}

/// All [`ArgMatches`] from arguments that were supplied to the program at runtime by the user.
fn arg_matches() -> ArgMatches {
    command!()
        .arg(
            arg!(--width [width] "Width of the output svgs in pixels")
                .value_parser(value_parser!(u32))
                .id("size_x"),
        )
        .arg(
            arg!(--height [height] "Height of the output svgs in pixels")
                .value_parser(value_parser!(u32))
                .id("size_y"),
        )
        .arg(
            arg!(--"min-y" [max_y] "Minimum y value in `mg/dL`")
                .value_parser(value_parser!(u32))
                .id("min_y_spec"),
        )
        .arg(
            arg!(--"max-y" [max_y] "Maximum y value in `mg/dL`")
                .value_parser(value_parser!(u32))
                .id("max_y_spec"),
        )
        .arg(
            arg!(--nx --"num-x" [num_labels_x] "Maximum number of x labels")
                .value_parser(value_parser!(usize))
                .id("num_labels_x"),
        )
        .arg(
            arg!(--ny --"num-y" [num_labels_y] "Maximum number of y labels")
                .value_parser(value_parser!(usize))
                .id("num_labels_y"),
        )
        .arg(
            arg!(--sx --"size-x" [label_size_x] "Size of x labels in pixels")
                .value_parser(value_parser!(u32))
                .id("label_size_x"),
        )
        .arg(
            arg!(--sy --"size-y" [label_size_y] "Size of y labels in pixels")
                .value_parser(value_parser!(u32))
                .id("label_size_y"),
        )
        .arg(
            arg!(--cfs [font_size] "Caption font size")
                .value_parser(value_parser!(u32))
                .id("caption_font_size"),
        )
        .arg(
            arg!(-r --radius [radius] "Radius of a single point in pixels")
                .value_parser(value_parser!(u32))
                .id("point_radius"),
        )
        .arg(
            arg!(--low [low_glucose_threshold] "Low glucose threshold")
                .value_parser(value_parser!(u32))
                .id("min_glucose_threshold"),
        )
        .arg(
            arg!(--high [high_glucose_threshold] "High glucose threshold")
                .value_parser(value_parser!(u32))
                .id("max_glucose_threshold"),
        )
        .arg(
            arg!(
                <INPUT_FILE> "Input file (csv)"
            )
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                <OUTPUT_DIR> "Output directory"
            )
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches()
}
