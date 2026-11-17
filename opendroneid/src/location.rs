//! Location/Vector message, its accuracies, heights and coordinates.

use bytes::{Buf, BufMut};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use opendroneid_sys as sys;

use crate::{
    constants::{
        MAX_DIRECTION, MAX_LATITUDE, MAX_LONGITUDE, MAX_SPEED_HORIZONTAL, MAX_SPEED_VERTICAL,
        MIN_DIRECTION, MIN_LATITUDE, MIN_LONGITUDE, MIN_SPEED_HORIZONTAL, MIN_SPEED_VERTICAL,
    },
    error::Error,
    macros,
    message::{Message, MessageInternal},
    values::{checked, checked_altitude, none_if},
};

/// Status of the UA reported in the [`Location`] message. Accessible through [`Location::status`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum Status {
    #[default]
    Undeclared = sys::ODID_status_ODID_STATUS_UNDECLARED,
    Ground = sys::ODID_status_ODID_STATUS_GROUND,
    Airborne = sys::ODID_status_ODID_STATUS_AIRBORNE,
    Emergency = sys::ODID_status_ODID_STATUS_EMERGENCY,
    RemoteIdSystemFailure = sys::ODID_status_ODID_STATUS_REMOTE_ID_SYSTEM_FAILURE,
}

/// Reference of the [`Location::height`] of the UA. Accessible through [`Height::reference`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum HeightReference {
    /// Indicates that the height is relative to the takeoff location of the UA.
    #[default]
    Takeoff = sys::ODID_Height_reference_ODID_HEIGHT_REF_OVER_TAKEOFF,
    /// Indicates that the height is relative to the ground level at the current location of the UA.
    Ground = sys::ODID_Height_reference_ODID_HEIGHT_REF_OVER_GROUND,
}

/// Accuracy of horizontal position information in the [`Location`] message.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum HorizontalAccuracy {
    #[default]
    Unknown = sys::ODID_Horizontal_accuracy_ODID_HOR_ACC_UNKNOWN,
    LessThan10NM = sys::ODID_Horizontal_accuracy_ODID_HOR_ACC_10NM,
    LessThan4NM = sys::ODID_Horizontal_accuracy_ODID_HOR_ACC_4NM,
    LessThan2NM = sys::ODID_Horizontal_accuracy_ODID_HOR_ACC_2NM,
    LessThan1NM = sys::ODID_Horizontal_accuracy_ODID_HOR_ACC_1NM,
    LessThan0_5NM = sys::ODID_Horizontal_accuracy_ODID_HOR_ACC_0_5NM,
    LessThan0_3NM = sys::ODID_Horizontal_accuracy_ODID_HOR_ACC_0_3NM,
    LessThan0_1NM = sys::ODID_Horizontal_accuracy_ODID_HOR_ACC_0_1NM,
    LessThan0_05NM = sys::ODID_Horizontal_accuracy_ODID_HOR_ACC_0_05NM,
    LessThan30Meter = sys::ODID_Horizontal_accuracy_ODID_HOR_ACC_30_METER,
    LessThan10Meter = sys::ODID_Horizontal_accuracy_ODID_HOR_ACC_10_METER,
    LessThan3Meter = sys::ODID_Horizontal_accuracy_ODID_HOR_ACC_3_METER,
    LessThan1Meter = sys::ODID_Horizontal_accuracy_ODID_HOR_ACC_1_METER,
}

/// Accuracy of the [`Location::altitude_barometric`] and [`Location::altitude_geodetic`] fields in the [`Location`] message.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum VerticalAccuracy {
    #[default]
    Unknown = sys::ODID_Vertical_accuracy_ODID_VER_ACC_UNKNOWN,
    LessThan150Meter = sys::ODID_Vertical_accuracy_ODID_VER_ACC_150_METER,
    LessThan45Meter = sys::ODID_Vertical_accuracy_ODID_VER_ACC_45_METER,
    LessThan25Meter = sys::ODID_Vertical_accuracy_ODID_VER_ACC_25_METER,
    LessThan10Meter = sys::ODID_Vertical_accuracy_ODID_VER_ACC_10_METER,
    LessThan3Meter = sys::ODID_Vertical_accuracy_ODID_VER_ACC_3_METER,
    LessThan1Meter = sys::ODID_Vertical_accuracy_ODID_VER_ACC_1_METER,
}

/// Accuracy of the [`Location::speed_horizontal`] and [`Location::speed_vertical`] fields in the
/// [`Location`] message.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum SpeedAccuracy {
    #[default]
    Unknown = sys::ODID_Speed_accuracy_ODID_SPEED_ACC_UNKNOWN,
    LessThan10MetersPerSecond = sys::ODID_Speed_accuracy_ODID_SPEED_ACC_10_METERS_PER_SECOND,
    LessThan3MetersPerSecond = sys::ODID_Speed_accuracy_ODID_SPEED_ACC_3_METERS_PER_SECOND,
    LessThan1MetersPerSecond = sys::ODID_Speed_accuracy_ODID_SPEED_ACC_1_METERS_PER_SECOND,
    LessThan0_3MetersPerSecond = sys::ODID_Speed_accuracy_ODID_SPEED_ACC_0_3_METERS_PER_SECOND,
}

/// Accuracy of the [`Location::timestamp`] information in the [`Location`] message.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum TimestampAccuracy {
    #[default]
    Unknown = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_UNKNOWN,
    /// Indicates that the timestamp is accurate to within 0.1 seconds.
    LessThan0_1Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_0_1_SECOND,
    /// Indicates that the timestamp is accurate to within 0.2 seconds.
    LessThan0_2Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_0_2_SECOND,
    /// Indicates that the timestamp is accurate to within 0.3 seconds.
    LessThan0_3Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_0_3_SECOND,
    /// Indicates that the timestamp is accurate to within 0.4 seconds.
    LessThan0_4Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_0_4_SECOND,
    /// Indicates that the timestamp is accurate to within 0.5 seconds.
    LessThan0_5Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_0_5_SECOND,
    /// Indicates that the timestamp is accurate to within 0.6 seconds.
    LessThan0_6Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_0_6_SECOND,
    /// Indicates that the timestamp is accurate to within 0.7 seconds.
    LessThan0_7Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_0_7_SECOND,
    /// Indicates that the timestamp is accurate to within 0.8 seconds.
    LessThan0_8Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_0_8_SECOND,
    /// Indicates that the timestamp is accurate to within 0.9 seconds.
    LessThan0_9Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_0_9_SECOND,
    /// Indicates that the timestamp is accurate to within 1.0 seconds.
    LessThan1_0Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_1_0_SECOND,
    /// Indicates that the timestamp is accurate to within 1.1 seconds.
    LessThan1_1Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_1_1_SECOND,
    /// Indicates that the timestamp is accurate to within 1.2 seconds.
    LessThan1_2Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_1_2_SECOND,
    /// Indicates that the timestamp is accurate to within 1.3 seconds.
    LessThan1_3Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_1_3_SECOND,
    /// Indicates that the timestamp is accurate to within 1.4 seconds.
    LessThan1_4Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_1_4_SECOND,
    /// Indicates that the timestamp is accurate to within 1.5 seconds.
    LessThan1_5Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_1_5_SECOND,
}

/// Height of the UA in meters in the [`Location`] message. The variant is the [`HeightReference`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Height {
    /// Height above the takeoff location of the UA.
    AboveTakeoff(f32),
    /// Height above the ground level at the current location of the UA.
    AboveGroundLevel(f32),
}

impl Height {
    /// Returns the [`HeightReference`] of the height.
    pub fn reference(&self) -> HeightReference {
        match self {
            Height::AboveTakeoff(_) => HeightReference::Takeoff,
            Height::AboveGroundLevel(_) => HeightReference::Ground,
        }
    }

    /// Returns the height in meters.
    pub fn value(&self) -> f32 {
        match self {
            Height::AboveTakeoff(height) | Height::AboveGroundLevel(height) => *height,
        }
    }
}

/// WGS-84 coordinates in degrees, in a [`Location`] or an [`OperatorPosition`](crate::OperatorPosition).
///
/// The protocol sends unknown coordinates as 0°/0°, so they cannot be set as known coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinates {
    /// Latitude in degrees, [`MIN_LATITUDE`] to [`MAX_LATITUDE`].
    pub latitude: f64,
    /// Longitude in degrees, [`MIN_LONGITUDE`] to [`MAX_LONGITUDE`].
    pub longitude: f64,
}

impl Coordinates {
    /// Returns the coordinates, or `None` for 0°/0°, which marks unknown coordinates.
    pub(crate) fn known(latitude: f64, longitude: f64) -> Option<Self> {
        (latitude != 0.0 || longitude != 0.0).then_some(Coordinates {
            latitude,
            longitude,
        })
    }

    /// Checks the coordinates and returns the latitude and longitude to store, 0°/0° for `None`.
    pub(crate) fn checked(coordinates: Option<Self>) -> Result<(f64, f64), Error> {
        let Some(Coordinates {
            latitude,
            longitude,
        }) = coordinates
        else {
            return Ok((0.0, 0.0));
        };
        let latitude = checked(
            "latitude",
            Some(latitude),
            MIN_LATITUDE as f64..=MAX_LATITUDE as f64,
            0.0,
        )?;
        let longitude = checked(
            "longitude",
            Some(longitude),
            MIN_LONGITUDE as f64..=MAX_LONGITUDE as f64,
            0.0,
        )?;
        if latitude == 0.0 && longitude == 0.0 {
            return Err(Error::InvalidValue {
                field: "coordinates",
                value: "0°/0° marks unknown coordinates".into(),
            });
        }
        Ok((latitude, longitude))
    }
}

macros::impl_message!(
    /// Location message as defined in the underlying C library.
    ///
    /// Values the UA does not know are `None`. A new message has every such value unknown.
    ///
    /// ```rust
    /// use opendroneid::{Coordinates, Location};
    ///
    /// // Create a new Location message with default values.
    /// let location = Location::default()
    ///     // Set fields of the Location message using the builder pattern.
    ///     .with_status(opendroneid::Status::Airborne)
    ///     .with_coordinates(Coordinates { latitude: 37.7749, longitude: -77.4194 })
    ///     .expect("Invalid coordinates")
    ///     .with_direction(90.0)
    ///     .expect("Invalid direction value")
    ///     .with_speed_horizontal(5.0)
    ///     .expect("Invalid speed value");
    ///
    /// assert_eq!(location.direction(), Some(90.0));
    /// assert_eq!(location.altitude_geodetic(), None);
    /// ```
    Location,
    sys::ODID_Location_data,
    sys::ODID_Location_encoded,
    init_location_data,
    sys::encodeLocationMessage,
    sys::decodeLocationMessage
);

/// Initializes the Location data like `odid_initLocationData` and also marks the timestamp as
/// unknown, which the C library leaves at 0 seconds.
///
/// # Safety
///
/// `data` must be a valid pointer to an `ODID_Location_data`.
unsafe fn init_location_data(data: *mut sys::ODID_Location_data) {
    unsafe {
        sys::odid_initLocationData(data);
        (*data).TimeStamp = sys::INV_TIMESTAMP as f32;
    }
}

impl Location {
    /// Returns the status of the message, or an [`Error`] if the value is invalid.
    pub fn status(&self) -> Result<Status, Error> {
        Status::from_u32(self.data.Status).ok_or(Error::EnumMappingError {
            field: "Status",
            value: self.data.Status,
        })
    }
    /// Sets the status of the message.
    pub fn with_status(mut self, status: Status) -> Self {
        self.data.Status = status as u32;
        self
    }
    /// Returns the direction of the UA in degrees clockwise from true north, or `None` if it is
    /// unknown.
    pub fn direction(&self) -> Option<f32> {
        none_if(self.data.Direction, sys::INV_DIR as f32)
    }
    /// Sets the direction of the UA in degrees clockwise from true north, `None` if it is unknown.
    /// Returns an error if the direction is out of the valid range defined by the protocol.
    pub fn with_direction(mut self, direction: impl Into<Option<f32>>) -> Result<Self, Error> {
        self.data.Direction = checked(
            "direction",
            direction.into(),
            MIN_DIRECTION as f32..=MAX_DIRECTION as f32,
            sys::INV_DIR as f32,
        )?;
        Ok(self)
    }
    /// Returns the horizontal speed of the UA in m/s, or `None` if it is unknown.
    pub fn speed_horizontal(&self) -> Option<f32> {
        none_if(self.data.SpeedHorizontal, sys::INV_SPEED_H as f32)
    }
    /// Sets the horizontal speed of the UA in m/s, `None` if it is unknown.
    /// Returns an error if the speed is out of the valid range defined by the protocol.
    pub fn with_speed_horizontal(mut self, speed: impl Into<Option<f32>>) -> Result<Self, Error> {
        self.data.SpeedHorizontal = checked(
            "speed_horizontal",
            speed.into(),
            MIN_SPEED_HORIZONTAL as f32..=MAX_SPEED_HORIZONTAL as f32,
            sys::INV_SPEED_H as f32,
        )?;
        Ok(self)
    }
    /// Returns the vertical speed of the UA in m/s, positive upwards, or `None` if it is unknown.
    pub fn speed_vertical(&self) -> Option<f32> {
        none_if(self.data.SpeedVertical, sys::INV_SPEED_V as f32)
    }
    /// Sets the vertical speed of the UA in m/s, positive upwards, `None` if it is unknown.
    /// Returns an error if the speed is out of the valid range defined by the protocol.
    pub fn with_speed_vertical(mut self, speed: impl Into<Option<f32>>) -> Result<Self, Error> {
        self.data.SpeedVertical = checked(
            "speed_vertical",
            speed.into(),
            MIN_SPEED_VERTICAL as f32..=MAX_SPEED_VERTICAL as f32,
            sys::INV_SPEED_V as f32,
        )?;
        Ok(self)
    }
    /// Returns the coordinates of the UA, or `None` if they are unknown.
    pub fn coordinates(&self) -> Option<Coordinates> {
        Coordinates::known(self.data.Latitude, self.data.Longitude)
    }
    /// Sets the coordinates of the UA, `None` if they are unknown.
    /// Returns an error if a coordinate is out of the valid range defined by the protocol, or for
    /// 0°/0°, which marks unknown coordinates.
    pub fn with_coordinates(
        mut self,
        coordinates: impl Into<Option<Coordinates>>,
    ) -> Result<Self, Error> {
        (self.data.Latitude, self.data.Longitude) = Coordinates::checked(coordinates.into())?;
        Ok(self)
    }
    /// Returns the barometric altitude of the UA in meters, or `None` if it is unknown.
    ///
    /// The barometric altitude is the uncorrected barometric pressure altitude
    /// (based on reference standard 29.92 inHg, 1013.25 mb).
    pub fn altitude_barometric(&self) -> Option<f32> {
        none_if(self.data.AltitudeBaro, sys::INV_ALT as f32)
    }
    /// Sets the barometric altitude of the UA in meters, `None` if it is unknown.
    /// Returns an error if the altitude is out of the valid range defined by the protocol, or for
    /// [`MIN_ALTITUDE`](crate::MIN_ALTITUDE), which marks an unknown altitude.
    pub fn with_altitude_barometric(
        mut self,
        altitude: impl Into<Option<f32>>,
    ) -> Result<Self, Error> {
        self.data.AltitudeBaro = checked_altitude("altitude_barometric", altitude.into())?;
        Ok(self)
    }
    /// Returns the geodetic altitude of the UA in meters, or `None` if it is unknown.
    ///
    /// The geodetic altitude is the distance above or below the surface of the WGS-84 ellipsoid.
    pub fn altitude_geodetic(&self) -> Option<f32> {
        none_if(self.data.AltitudeGeo, sys::INV_ALT as f32)
    }
    /// Sets the geodetic altitude of the UA in meters, `None` if it is unknown.
    /// Returns an error if the altitude is out of the valid range defined by the protocol, or for
    /// [`MIN_ALTITUDE`](crate::MIN_ALTITUDE), which marks an unknown altitude.
    ///
    /// The geodetic altitude is the distance above or below the surface of the WGS-84 ellipsoid.
    pub fn with_altitude_geodetic(
        mut self,
        altitude: impl Into<Option<f32>>,
    ) -> Result<Self, Error> {
        self.data.AltitudeGeo = checked_altitude("altitude_geodetic", altitude.into())?;
        Ok(self)
    }
    /// Returns the height of the UA, `None` if it is unknown, or an [`Error`] if the height
    /// reference is invalid.
    ///
    /// The variant of the [`Height`] is the height reference of the message.
    pub fn height(&self) -> Result<Option<Height>, Error> {
        let reference =
            HeightReference::from_u32(self.data.HeightType).ok_or(Error::EnumMappingError {
                field: "HeightType",
                value: self.data.HeightType,
            })?;
        Ok(
            none_if(self.data.Height, sys::INV_ALT as f32).map(|height| match reference {
                HeightReference::Takeoff => Height::AboveTakeoff(height),
                HeightReference::Ground => Height::AboveGroundLevel(height),
            }),
        )
    }
    /// Sets the height of the UA and its reference, `None` if it is unknown.
    /// Returns an error if the height is out of the valid range defined by the protocol, or for
    /// [`MIN_ALTITUDE`](crate::MIN_ALTITUDE), which marks an unknown height.
    pub fn with_height(mut self, height: impl Into<Option<Height>>) -> Result<Self, Error> {
        let height = height.into();
        self.data.Height = checked_altitude("height", height.map(|height| height.value()))?;
        // An unknown height has no reference, so store the default one.
        let reference = height.map_or(HeightReference::default(), |height| height.reference());
        self.data.HeightType = reference as u32;
        Ok(self)
    }
    /// Returns the horizontal accuracy of the location information, or an [`Error`] if the value is invalid.
    pub fn horizontal_accuracy(&self) -> Result<HorizontalAccuracy, Error> {
        HorizontalAccuracy::from_u32(self.data.HorizAccuracy).ok_or(Error::EnumMappingError {
            field: "HorizontalAccuracy",
            value: self.data.HorizAccuracy,
        })
    }
    /// Sets the horizontal accuracy of the location information.
    pub fn with_horizontal_accuracy(mut self, accuracy: HorizontalAccuracy) -> Self {
        self.data.HorizAccuracy = accuracy as u32;
        self
    }
    /// Returns the vertical accuracy of the location information, or an [`Error`] if the value is invalid.
    pub fn vertical_accuracy(&self) -> Result<VerticalAccuracy, Error> {
        VerticalAccuracy::from_u32(self.data.VertAccuracy).ok_or(Error::EnumMappingError {
            field: "VerticalAccuracy",
            value: self.data.VertAccuracy,
        })
    }
    /// Sets the vertical accuracy of the location information.
    pub fn with_vertical_accuracy(mut self, accuracy: VerticalAccuracy) -> Self {
        self.data.VertAccuracy = accuracy as u32;
        self
    }
    /// Returns the barometric accuracy of the location information, or an [`Error`] if the value is invalid.
    pub fn barometric_accuracy(&self) -> Result<VerticalAccuracy, Error> {
        VerticalAccuracy::from_u32(self.data.BaroAccuracy).ok_or(Error::EnumMappingError {
            field: "BarometricAccuracy",
            value: self.data.BaroAccuracy,
        })
    }
    /// Sets the barometric accuracy of the location information.
    pub fn with_barometric_accuracy(mut self, accuracy: VerticalAccuracy) -> Self {
        self.data.BaroAccuracy = accuracy as u32;
        self
    }
    /// Returns the speed accuracy of the location information, or an [`Error`] if the value is invalid.
    pub fn speed_accuracy(&self) -> Result<SpeedAccuracy, Error> {
        SpeedAccuracy::from_u32(self.data.SpeedAccuracy).ok_or(Error::EnumMappingError {
            field: "SpeedAccuracy",
            value: self.data.SpeedAccuracy,
        })
    }
    /// Sets the speed accuracy of the location information.
    pub fn with_speed_accuracy(mut self, accuracy: SpeedAccuracy) -> Self {
        self.data.SpeedAccuracy = accuracy as u32;
        self
    }
    /// Returns the timestamp accuracy of the location information, or an [`Error`] if the value is invalid.
    pub fn timestamp_accuracy(&self) -> Result<TimestampAccuracy, Error> {
        TimestampAccuracy::from_u32(self.data.TSAccuracy).ok_or(Error::EnumMappingError {
            field: "TimestampAccuracy",
            value: self.data.TSAccuracy,
        })
    }
    /// Sets the timestamp accuracy of the location information.
    pub fn with_timestamp_accuracy(mut self, accuracy: TimestampAccuracy) -> Self {
        self.data.TSAccuracy = accuracy as u32;
        self
    }

    /// Returns the timestamp as a floating point number of seconds since the start of the hour
    pub fn timestamp(&self) -> Option<f32> {
        none_if(self.data.TimeStamp, sys::INV_TIMESTAMP as f32)
    }

    /// Sets the timestamp as a floating point number of seconds since the start of the hour,
    /// `None` if it is unknown.
    /// Returns an error if the timestamp is out of the valid range defined by the protocol.
    pub fn with_timestamp(mut self, timestamp: impl Into<Option<f32>>) -> Result<Self, Error> {
        self.data.TimeStamp = checked(
            "timestamp",
            timestamp.into(),
            0.0..=sys::MAX_TIMESTAMP as f32,
            sys::INV_TIMESTAMP as f32,
        )?;
        Ok(self)
    }

    /// Returns the timestamp as the time nearest to `reference`, or `None` if it is unknown.
    ///
    /// The message only holds the seconds since the start of the hour. The hour is chosen so that
    /// the time is at most half an hour away from `reference`, usually the time the message was
    /// received. This also holds when the clock of the UA is slightly ahead of the receiver.
    #[cfg(feature = "chrono")]
    pub fn chrono_timestamp(
        &self,
        reference: chrono::DateTime<chrono::Utc>,
    ) -> Option<chrono::DateTime<chrono::Utc>> {
        let offset_ms = (f64::from(self.timestamp()?) * 1000.0).round() as i64;
        let reference_ms = reference.timestamp_millis();
        let mut time_ms = reference_ms - reference_ms.rem_euclid(HOUR_MS) + offset_ms;
        if time_ms - reference_ms > HOUR_MS / 2 {
            time_ms -= HOUR_MS;
        } else if reference_ms - time_ms > HOUR_MS / 2 {
            time_ms += HOUR_MS;
        }
        chrono::DateTime::from_timestamp_millis(time_ms)
    }

    /// Sets the timestamp to the seconds of `time` since the start of its hour, `None` if it is
    /// unknown.
    ///
    /// The message holds tenths of a second, so the time is rounded when it is encoded.
    #[cfg(feature = "chrono")]
    pub fn with_chrono_timestamp(
        mut self,
        time: impl Into<Option<chrono::DateTime<chrono::Utc>>>,
    ) -> Self {
        self.data.TimeStamp = match time.into() {
            Some(time) => time.timestamp_millis().rem_euclid(HOUR_MS) as f32 / 1000.0,
            None => sys::INV_TIMESTAMP as f32,
        };
        self
    }
}

/// Number of milliseconds in an hour, the range of the [`Location`] timestamp.
#[cfg(feature = "chrono")]
const HOUR_MS: i64 = 3_600_000;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::System;
    use crate::constants::{MAX_ALTITUDE, MIN_ALTITUDE};
    use crate::testing::*;

    #[test]
    fn height_round_trips_with_reference() {
        for height in [Height::AboveTakeoff(12.5), Height::AboveGroundLevel(-3.0)] {
            let message = Location::default().with_height(height).unwrap();

            assert_eq!(message.height().unwrap(), Some(height));
            assert_eq!(round_trip(&message).height().unwrap(), Some(height));
        }
    }

    #[test]
    fn default_height_is_unknown() {
        assert_eq!(Location::default().height().unwrap(), None);
    }

    #[test]
    fn height_out_of_range_is_rejected() {
        let result =
            Location::default().with_height(Height::AboveTakeoff(MAX_ALTITUDE as f32 + 1.0));

        assert!(matches!(
            result,
            Err(Error::InvalidValue {
                field: "height",
                ..
            })
        ));
    }

    #[test]
    fn default_location_values_are_unknown() {
        let location = Location::default();

        assert_eq!(location.direction(), None);
        assert_eq!(location.speed_horizontal(), None);
        assert_eq!(location.speed_vertical(), None);
        assert_eq!(location.coordinates(), None);
        assert_eq!(location.altitude_barometric(), None);
        assert_eq!(location.altitude_geodetic(), None);
        assert_eq!(location.height().unwrap(), None);
        assert_eq!(location.timestamp(), None);
    }

    #[test]
    fn known_location_values_round_trip() {
        let decoded = round_trip(
            &Location::default()
                .with_direction(271.0)
                .unwrap()
                .with_speed_horizontal(12.25)
                .unwrap()
                .with_speed_vertical(-3.5)
                .unwrap()
                .with_coordinates(MUNICH)
                .unwrap()
                .with_altitude_barometric(512.5)
                .unwrap()
                .with_altitude_geodetic(560.0)
                .unwrap()
                .with_timestamp(1234.5)
                .unwrap(),
        );

        assert_eq!(decoded.direction(), Some(271.0));
        assert_eq!(decoded.speed_horizontal(), Some(12.25));
        assert_eq!(decoded.speed_vertical(), Some(-3.5));
        assert_coordinates_eq(decoded.coordinates(), Some(MUNICH));
        assert_eq!(decoded.altitude_barometric(), Some(512.5));
        assert_eq!(decoded.altitude_geodetic(), Some(560.0));
        assert_eq!(decoded.timestamp(), Some(1234.5));
    }

    #[test]
    fn location_values_can_be_set_to_unknown() {
        let decoded = round_trip(
            &Location::default()
                .with_direction(90.0)
                .and_then(|l| l.with_direction(None))
                .and_then(|l| l.with_speed_horizontal(1.0))
                .and_then(|l| l.with_speed_horizontal(None))
                .and_then(|l| l.with_speed_vertical(1.0))
                .and_then(|l| l.with_speed_vertical(None))
                .and_then(|l| l.with_coordinates(MUNICH))
                .and_then(|l| l.with_coordinates(None))
                .and_then(|l| l.with_altitude_barometric(1.0))
                .and_then(|l| l.with_altitude_barometric(None))
                .and_then(|l| l.with_altitude_geodetic(1.0))
                .and_then(|l| l.with_altitude_geodetic(None))
                .and_then(|l| l.with_height(Height::AboveGroundLevel(1.0)))
                .and_then(|l| l.with_height(None))
                .and_then(|l| l.with_timestamp(1.0))
                .and_then(|l| l.with_timestamp(None))
                .unwrap(),
        );

        assert_eq!(decoded, round_trip(&Location::default()));
    }

    #[test]
    fn unknown_markers_are_rejected_as_known_values() {
        assert!(matches!(
            Location::default().with_coordinates(Coordinates {
                latitude: 0.0,
                longitude: 0.0
            }),
            Err(Error::InvalidValue {
                field: "coordinates",
                ..
            })
        ));
        assert!(matches!(
            Location::default().with_altitude_geodetic(MIN_ALTITUDE as f32),
            Err(Error::InvalidValue {
                field: "altitude_geodetic",
                ..
            })
        ));
        assert!(matches!(
            System::default().with_area_floor(MIN_ALTITUDE as f32),
            Err(Error::InvalidValue {
                field: "area_floor",
                ..
            })
        ));
    }

    #[test]
    fn out_of_range_location_values_are_rejected() {
        let location = Location::default();

        assert!(location.clone().with_direction(361.0).is_err());
        assert!(location.clone().with_direction(f32::NAN).is_err());
        assert!(location.clone().with_speed_horizontal(255.0).is_err());
        assert!(location.clone().with_speed_vertical(63.0).is_err());
        assert!(location.clone().with_timestamp(3600.5).is_err());
        assert!(
            location
                .with_coordinates(Coordinates {
                    latitude: 0.0,
                    longitude: 180.5
                })
                .is_err()
        );
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn location_timestamp_is_nearest_to_the_reference() {
        for (seconds, reference, expected) in [
            (600.0, "2026-11-10T15:20:00Z", "2026-11-10T15:10:00Z"),
            // Sent just before the hour, received just after it.
            (3598.0, "2026-11-10T15:00:10Z", "2026-11-10T14:59:58Z"),
            // Clock of the UA slightly ahead of the receiver.
            (0.5, "2026-11-10T14:59:59Z", "2026-11-10T15:00:00.500Z"),
            (1800.0, "2026-11-10T15:00:00Z", "2026-11-10T15:30:00Z"),
        ] {
            let location = Location::default().with_timestamp(seconds).unwrap();

            assert_eq!(
                location.chrono_timestamp(utc(reference)),
                Some(utc(expected))
            );
        }
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn location_chrono_timestamp_round_trips() {
        let time = utc("2026-11-10T15:42:17.300Z");
        let decoded = round_trip(&Location::default().with_chrono_timestamp(time));

        assert_eq!(decoded.timestamp(), Some(2537.3));
        assert_eq!(
            decoded.chrono_timestamp(utc("2026-11-10T15:42:18Z")),
            Some(time)
        );
        assert_eq!(Location::default().chrono_timestamp(time), None);
        assert_eq!(
            Location::default()
                .with_chrono_timestamp(time)
                .with_chrono_timestamp(None)
                .timestamp(),
            None
        );
    }
}
