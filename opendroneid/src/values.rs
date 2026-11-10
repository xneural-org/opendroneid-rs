//! Helpers for fields that mark an unknown value with a special value.

use crate::constants::{MAX_ALTITUDE, MIN_ALTITUDE};
use crate::error::Error;
use opendroneid_sys as sys;

/// Returns `value`, or `None` if it is the `unknown` marker of its field.
pub(crate) fn none_if<T: PartialEq>(value: T, unknown: T) -> Option<T> {
    (value != unknown).then_some(value)
}

/// Checks that `value` is in `range` and returns the value to store, `unknown` for `None`.
pub(crate) fn checked<T: PartialOrd + ToString>(
    field: &'static str,
    value: Option<T>,
    range: std::ops::RangeInclusive<T>,
    unknown: T,
) -> Result<T, Error> {
    match value {
        None => Ok(unknown),
        Some(value) if range.contains(&value) => Ok(value),
        Some(value) => Err(Error::InvalidValue {
            field,
            value: value.to_string(),
        }),
    }
}

/// Checks an altitude and returns the value to store, [`MIN_ALTITUDE`] for `None`.
///
/// [`MIN_ALTITUDE`] itself marks an unknown altitude, so it is not a valid known altitude.
pub(crate) fn checked_altitude(field: &'static str, altitude: Option<f32>) -> Result<f32, Error> {
    if altitude == Some(sys::INV_ALT as f32) {
        return Err(Error::InvalidValue {
            field,
            value: format!("{} marks an unknown altitude", sys::INV_ALT),
        });
    }
    checked(
        field,
        altitude,
        MIN_ALTITUDE as f32..=MAX_ALTITUDE as f32,
        sys::INV_ALT as f32,
    )
}
