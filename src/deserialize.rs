/*
 * Heavily inspired by:
 * - https://docs.rs/csv/latest/csv/tutorial/index.html
 * - https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html
 */

pub(crate) mod prelude {
    pub(crate) use super::{GlucoseReadingsMap, readings_map};
}

use std::{
    collections::{HashMap, HashSet},
    error::Error,
    path::PathBuf,
};

use csv::ReaderBuilder;
use jiff::civil::{Date, Time};
use serde::Deserialize;

/// Map of [`Date`] to [`GlucoseReading`]s of that day.
#[derive(Default, Debug)]
pub(crate) struct GlucoseReadingsMap(pub(crate) HashMap<Date, HashSet<GlucoseReading>>);

/// A glucose reading.
#[derive(Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct GlucoseReading {
    /// Time of day the reading was taken.
    pub(crate) time: Time,
    /// Glucose measurement in `mg/dL`.
    pub(crate) measurement: u32,
}
impl GlucoseReading {
    pub(crate) fn new(time: Time, measurement: u32) -> Self {
        Self { time, measurement }
    }
}

/// A record in SiDiary CSV format.
///
/// This CSV format is used by [xDrip+](https://github.com/nightscoutfoundation/xdrip) for exports.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
struct SiDiaryRecord {
    day: String,
    time: String,
    udt_cgms: Option<u32>,
}

/// Deserialize csv at `input_path` and construct [`GlucoseReadingsMap`].
pub(crate) fn readings_map(input_path: &PathBuf) -> Result<GlucoseReadingsMap, Box<dyn Error>> {
    let mut readings_map: GlucoseReadingsMap = GlucoseReadingsMap::default();

    let mut reader = ReaderBuilder::new().delimiter(b';').from_path(input_path)?;
    for result in reader.deserialize() {
        let record: SiDiaryRecord = result?;
        let date = Date::strptime("%d.%m.%Y", record.day)?;
        let time = Time::strptime("%H:%M", record.time)?;

        if let Some(measurement) = record.udt_cgms {
            readings_map
                .0
                .entry(date)
                .or_default()
                .insert(GlucoseReading::new(time, measurement));
        }
    }

    Ok(readings_map)
}
