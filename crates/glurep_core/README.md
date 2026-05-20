# Glurep Core

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/leomeinel/glurep)
[![Crates.io](https://img.shields.io/crates/v/glurep_core.svg)](https://crates.io/crates/glurep_core)
[![Downloads](https://img.shields.io/crates/d/glurep_core.svg)](https://crates.io/crates/glurep_core)
[![Docs](https://docs.rs/glurep_core/badge.svg)](https://docs.rs/glurep/latest/glurep_core)

Core library for generating pdf reports from glucose readings in supported csv formats.

## Supported csv formats

Currently this is only compatible with the [SiDiary CSV format](https://www.sidiary.org/help/en/_35.htm) used by [xDrip+](https://github.com/nightscoutfoundation/xdrip) for exports.

The format expects the following:

```csv
DAY;TIME;UDT_CGMS;BG_LEVEL;CH_GR;BOLUS;REMARK
14.05.2026;19:20;115;;;;
```

Only `DAY`, `TIME` and `UDT_CGMS` are processed.

### Showcase

<img src="https://github.com/leomeinel/glurep/blob/main/static/showcase.svg?raw=true" width="400" alt="glucose levels in a colored 2d chart">
