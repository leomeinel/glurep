# Gluco Plot

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/leomeinel/gluco-plot)

Plot glucose readings in supported csv formats to pdf.

## Supported csv formats

Currently this is only compatible with the [SiDiary CSV format](https://www.sidiary.org/help/en/_35.htm) used by [xDrip+](https://github.com/nightscoutfoundation/xdrip) for exports.

The format expects the following:

```csv
DAY;TIME;UDT_CGMS;BG_LEVEL;CH_GR;BOLUS;REMARK
14.05.2026;19:20;115;;;;
```

Only `DAY`, `TIME` and `UDT_CGMS` are processed.
