# Glurep

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/leomeinel/glurep)
[![Crates.io](https://img.shields.io/crates/v/glurep.svg)](https://crates.io/crates/glurep)
[![Downloads](https://img.shields.io/crates/d/glurep.svg)](https://crates.io/crates/glurep)
[![Docs](https://docs.rs/glurep/badge.svg)](https://docs.rs/glurep/latest/glurep/)

Generate pdf reports from glucose readings in supported csv formats.

## Supported csv formats

Currently this is only compatible with the [SiDiary CSV format](https://www.sidiary.org/help/en/_35.htm) used by [xDrip+](https://github.com/nightscoutfoundation/xdrip) for exports.

The format expects the following:

```csv
DAY;TIME;UDT_CGMS;BG_LEVEL;CH_GR;BOLUS;REMARK
14.05.2026;19:20;115;;;;
```

Only `DAY`, `TIME` and `UDT_CGMS` are processed.

## Usage

```
Usage: glurep [OPTIONS] <INPUT_FILE> <OUTPUT_PATH>

Arguments:
  <INPUT_FILE>   Input file (csv)
  <OUTPUT_PATH>  Output file (pdf) [default] or directory if using `--svg`

Options:
  -s, --svg                              Output svgs instead of pdf
  -f, --force                            Force overwrite files
  -n, --name [<patient_name>]            Patient name [default: Patient]
      --width [<width>]                  Width of the output pdf in `mm`
      --height [<height>]                Height of the output pdf in `mm`
  -m, --margin [<margin>]                Margin of the output pdf in `mm`
      --hfs [<header_font_size>]         Header font size in `pt`
      --min-y [<max_y>]                  Minimum y value in `mg/dL`
      --max-y [<max_y>]                  Maximum y value in `mg/dL`
      --num-x [<num_labels_x>]           Maximum number of x labels
      --num-y [<num_labels_y>]           Maximum number of y labels
      --size-x [<label_size_x>]          X label size approximately in `mm`
      --size-y [<label_size_y>]          Y label size approximately in `mm`
  -r, --radius [<radius>]                Radius of a single point approximately in `mm`
      --low [<low_glucose_threshold>]    Low glucose threshold
      --high [<high_glucose_threshold>]  High glucose threshold
  -h, --help                             Print help
  -V, --version                          Print version
```

### Showcase

The following showcase has been generated with:

```sh
glurep --svg ./input.csv ./output/
```

<img src="https://github.com/leomeinel/glurep/blob/main/static/showcase.svg?raw=true" width="400" alt="glucose levels in a colored 2d chart">
