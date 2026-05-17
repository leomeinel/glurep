use jiff::civil::Time;

pub(crate) mod prelude {
    pub(crate) use super::num_seconds_from_midnight;
}

/// Number of seconds from midnight for [`Time`].
pub(crate) fn num_seconds_from_midnight(time: &Time) -> u32 {
    time.hour() as u32 * 3600 + time.minute() as u32 * 60 + time.second() as u32
}
