/*
 * Heavily inspired by:
 * - https://docs.rs/csv/latest/csv/tutorial/index.html
 * - https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html
 */

pub(crate) mod prelude {
    pub(crate) use super::GlucoseReadingsMap;
}

use std::{
    collections::{HashMap, HashSet},
    error::Error,
    path::PathBuf,
};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use csv::ReaderBuilder;
use serde::Deserialize;

/// Map of [`NaiveDate`] to [`GlucoseReading`]s of that day.
#[derive(Default, Debug)]
pub(crate) struct GlucoseReadingsMap(pub(crate) HashMap<NaiveDate, HashSet<GlucoseReading>>);
impl GlucoseReadingsMap {
    /// Date format used while deserializing [`SiDiaryRecord`].
    ///
    /// See the [`format::strftime` module](chrono::format::strftime) for supported format
    /// sequences.
    const DATE_TIME_FMT: &str = "%d.%m.%YT%H:%M";

    /// Deserialize file at `file_path` with `time` used to determine timezone and construct [`GlucoseReadingsMap`].
    pub(crate) fn from_file_path(
        input_path: &PathBuf,
    ) -> Result<GlucoseReadingsMap, Box<dyn Error>> {
        let mut readings: GlucoseReadingsMap = GlucoseReadingsMap::default();

        let mut reader = ReaderBuilder::new().delimiter(b';').from_path(input_path)?;
        for result in reader.deserialize() {
            let record: SiDiaryRecord = result?;
            let date_time = format!("{}T{}", record.day, record.time);
            let date_time = NaiveDateTime::parse_from_str(date_time.as_str(), Self::DATE_TIME_FMT)?;

            if let Some(measurement) = record.udt_cgms {
                readings
                    .0
                    .entry(date_time.date())
                    .or_default()
                    .insert(GlucoseReading::from(date_time).with_measurement(measurement));
            }
        }

        Ok(readings)
    }
}

/// A glucose reading.
#[derive(Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct GlucoseReading {
    /// Time of day the reading was taken.
    pub(crate) time: NaiveTime,
    /// Glucose measurement in `mg/dL`.
    pub(crate) measurement: u32,
}
impl From<NaiveDateTime> for GlucoseReading {
    fn from(date_time: NaiveDateTime) -> Self {
        Self {
            time: date_time.time(),
            ..Default::default()
        }
    }
}
impl GlucoseReading {
    pub(crate) fn with_measurement(self, measurement: u32) -> Self {
        Self {
            measurement,
            ..self
        }
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
