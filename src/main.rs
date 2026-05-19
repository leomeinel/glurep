/*
 * Heavily inspired by:
 * - https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
 */

use std::{env, fs, path::PathBuf};

use anyhow::Context;
use clap::{ArgMatches, arg, command, value_parser};

use crate::{deserialize::prelude::*, log::prelude::*, pdf::prelude::*, plot::prelude::*};

mod deserialize;
mod log;
mod pdf;
mod plot;
mod utils;

fn main() -> Result<(), anyhow::Error> {
    let args = arg_matches();
    let use_svg = *args.get_one::<bool>("use_svg").unwrap();
    let use_force = *args.get_one::<bool>("use_force").unwrap();
    let input_path = args.get_one::<PathBuf>("INPUT_FILE").unwrap().clone();
    let output_path = args.get_one::<PathBuf>("OUTPUT_PATH").unwrap().clone();
    let patient_name = args.get_one::<String>("patient_name").unwrap().as_str();
    let plot_config = PlotConfig::from(&args);
    let page_config = PageConfig::from(&args);

    let readings_map = readings_map(&input_path)
        .context(format!("Failed to deserialize `{}`", input_path.display()))?;
    let svgs = plot_to_strings(&readings_map, &plot_config)
        .context(format!("Failed to plot `{}`", input_path.display()))?;

    if use_svg {
        if !output_path.is_dir() {
            return Err(FlagError::IoErrorNotADirectory(output_path).into());
        }

        for svg in svgs {
            let file_name = format!("{}.svg", svg.date);
            let output_path = output_path.join(file_name);
            if !use_force && output_path.try_exists()? {
                return Err(FlagError::IoErrorAlreadyExists(output_path).into());
            }

            fs::write(output_path, svg.contents)?;
        }
    } else {
        if !use_force && output_path.try_exists()? {
            return Err(FlagError::IoErrorAlreadyExists(output_path).into());
        } else if output_path.is_dir() {
            return Err(FlagError::IoErrorIsADirectory(output_path).into());
        }

        let pdf_bytes = svgs_to_pdf_bytes(svgs, page_config, patient_name)
            .context(format!("Failed to create pdf `{}`", output_path.display()))?;
        fs::write(output_path, pdf_bytes)?;
    }

    Ok(())
}

/// All [`ArgMatches`] from arguments that were supplied to the program at runtime by the user.
fn arg_matches() -> ArgMatches {
    command!()
        .arg(
            arg!(-s --svg "Output svgs instead of pdf")
                .value_parser(value_parser!(bool))
                .id("use_svg"),
        )
        .arg(
            arg!(-f --force "Force overwrite files")
                .value_parser(value_parser!(bool))
                .id("use_force"),
        )
        .arg(
            arg!(-n --name [patient_name] "Patient name")
                .default_value("Patient")
                .id("patient_name"),
        )
        .arg(
            arg!(--width [width] "Width of the output pdf in `mm`")
                .value_parser(value_parser!(u32))
                .id("size_x"),
        )
        .arg(
            arg!(--height [height] "Height of the output pdf in `mm`")
                .value_parser(value_parser!(u32))
                .id("size_y"),
        )
        .arg(
            arg!(-m --margin [margin] "Margin of the output pdf in `mm`")
                .value_parser(value_parser!(u32))
                .id("margin"),
        )
        .arg(
            arg!(--hfs [header_font_size] "Header font size in `pt`")
                .value_parser(value_parser!(f32))
                .id("header_font_size"),
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
            arg!(--sx --"size-x" [label_size_x] "X label size approximately in `mm`")
                .value_parser(value_parser!(u32))
                .id("label_size_x"),
        )
        .arg(
            arg!(--sy --"size-y" [label_size_y] "Y label size approximately in `mm`")
                .value_parser(value_parser!(u32))
                .id("label_size_y"),
        )
        .arg(
            arg!(-r --radius [radius] "Radius of a single point approximately in `mm`")
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
                <OUTPUT_PATH> "Output file (pdf) [default] or directory if using `--svg`"
            )
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches()
}
