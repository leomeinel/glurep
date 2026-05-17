use std::{env, error::Error, path::PathBuf};

use clap::{arg, command, value_parser};

use crate::{deserialize::prelude::*, plot::prelude::*};

mod deserialize;
mod plot;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = command!()
        .arg(
            arg!(
                -i <INPUT_FILE> "Input file (csv)"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -o <OUTPUT_DIR> "Output directory"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let input_path = matches.get_one::<PathBuf>("INPUT_FILE").unwrap();
    let readings = GlucoseReadingsMap::from_file_path(input_path)?;

    let output_path = matches.get_one::<PathBuf>("OUTPUT_DIR").unwrap();
    plot_to_svg(readings, output_path, PlotConfig::default())
}
