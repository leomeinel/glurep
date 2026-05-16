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

use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveTime};
use serde::Deserialize;

/// Map of [`NaiveDate`] to [`GlucoseReading`]s of that day.
#[derive(Default, Debug)]
pub(crate) struct GlucoseReadingsMap(pub(crate) HashMap<NaiveDate, HashSet<GlucoseReading>>);
impl GlucoseReadingsMap {
    /// Date format used while deserializing [`SiDiaryRecord`].
    ///
    /// See the [`format::strftime` module](chrono::format::strftime) for supported format
    /// sequences.
    const RAW_DATE_FMT: &str = "%d.%m.%YT%H:%M%z";

    /// Deserialize file at `file_path` with `time` used to determine timezone and construct [`GlucoseReadingsMap`].
    pub(crate) fn from_file_path(
        file_path: &PathBuf,
        time: &DateTime<Local>,
    ) -> Result<GlucoseReadingsMap, Box<dyn Error>> {
        let mut readings: GlucoseReadingsMap = GlucoseReadingsMap::default();

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_path(file_path)?;
        for result in reader.deserialize() {
            let record: SiDiaryRecord = result?;
            let date_time = format!("{}T{}{}", record.day, record.time, time.offset());
            let date_time = DateTime::parse_from_str(date_time.as_str(), Self::RAW_DATE_FMT)?;

            if let Some(measurement) = record.udt_cgms {
                readings
                    .0
                    .entry(date_time.date_naive())
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
impl From<DateTime<FixedOffset>> for GlucoseReading {
    fn from(date_time: DateTime<FixedOffset>) -> Self {
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
