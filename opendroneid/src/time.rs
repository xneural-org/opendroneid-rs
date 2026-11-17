//! Conversion of the Auth and System timestamps to and from `chrono` times.

use crate::error::Error;

/// Number of seconds from 1970-01-01 to 2019-01-01, the epoch of the Auth and System timestamps.
const ODID_EPOCH_OFFSET: i64 = 1546300800;

/// Converts seconds since 2019-01-01 00:00:00 UTC, the timestamp of an [`AuthPage::First`](crate::AuthPage::First) and of
/// a [`System`](crate::System) message, to a `chrono::DateTime<Utc>`.
pub fn from_odid_epoch(seconds: u32) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::from_timestamp(seconds as i64 + ODID_EPOCH_OFFSET, 0)
        .expect("every u32 offset from 2019 is a valid date")
}

/// Converts a `chrono::DateTime<Utc>` to whole seconds since 2019-01-01 00:00:00 UTC, the timestamp
/// of an [`AuthPage::First`](crate::AuthPage::First) and of a [`System`](crate::System) message.
///
/// Returns an error for a time before 2019 or after the range of a `u32`.
pub fn to_odid_epoch(time: chrono::DateTime<chrono::Utc>) -> Result<u32, Error> {
    u32::try_from(time.timestamp() - ODID_EPOCH_OFFSET).map_err(|_| Error::InvalidValue {
        field: "timestamp",
        value: time.to_rfc3339(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::*;

    #[cfg(feature = "chrono")]
    #[test]
    fn odid_epoch_conversions() {
        let time = utc("2026-11-10T15:00:00Z");
        let seconds = to_odid_epoch(time).unwrap();

        assert_eq!(from_odid_epoch(seconds), time);
        assert_eq!(from_odid_epoch(0).to_rfc3339(), "2019-01-01T00:00:00+00:00");
        assert!(to_odid_epoch(utc("2018-12-31T23:59:59Z")).is_err());
    }
}
