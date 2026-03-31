//! Rust bindings for the [opendroneid](https://github.com/opendroneid/opendroneid-core-c) library.

use bytes::{Buf, BufMut};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use opendroneid_sys as sys;

pub mod constants;
pub mod error;
mod macros;
mod utils;
pub use constants::*;
pub use error::*;

/// Message IDs used in the Open Drone ID messages as defined in the underlying C library.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum MessageId {
    /// Message ID used for the Basic ID message
    BasicId = sys::ODID_messagetype_ODID_MESSAGETYPE_BASIC_ID,
    /// Message ID used for the Location message
    Location = sys::ODID_messagetype_ODID_MESSAGETYPE_LOCATION,
    /// Message ID used for the Authentication message
    Auth = sys::ODID_messagetype_ODID_MESSAGETYPE_AUTH,
    /// Message ID used for the Self ID message
    SelfId = sys::ODID_messagetype_ODID_MESSAGETYPE_SELF_ID,
    /// Message ID used for the System message
    System = sys::ODID_messagetype_ODID_MESSAGETYPE_SYSTEM,
    /// Message ID used for the Operator ID message
    OperatorId = sys::ODID_messagetype_ODID_MESSAGETYPE_OPERATOR_ID,
    /// Message ID used for the Message Pack message
    MessagePack = sys::ODID_messagetype_ODID_MESSAGETYPE_PACKED,
}

impl TryFrom<u8> for MessageId {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let t = unsafe { sys::decodeMessageType(value) };
        if t == sys::ODID_messagetype_ODID_MESSAGETYPE_INVALID {
            return Err(Error::EnumMappingError {
                field: "MessageId",
                value: value as u32,
            });
        }
        MessageId::from_u32(t).ok_or(Error::EnumMappingError {
            field: "MessageId",
            value: t as u32,
        })
    }
}

/// Type of the [BasicId::uas_id]. Accessible through [BasicId::id_type].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum IdType {
    #[default]
    None = sys::ODID_idtype_ODID_IDTYPE_NONE,
    SerialNumber = sys::ODID_idtype_ODID_IDTYPE_SERIAL_NUMBER,
    CaaRegistrationId = sys::ODID_idtype_ODID_IDTYPE_CAA_REGISTRATION_ID,
    UtmAssignedUuid = sys::ODID_idtype_ODID_IDTYPE_UTM_ASSIGNED_UUID,
    SpecificSessionId = sys::ODID_idtype_ODID_IDTYPE_SPECIFIC_SESSION_ID,
}

/// Type of the UA in the [BasicId] message. Accessible through [BasicId::ua_type].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum UaType {
    #[default]
    None = sys::ODID_uatype_ODID_UATYPE_NONE,
    Aeroplane = sys::ODID_uatype_ODID_UATYPE_AEROPLANE,
    HelicopterOrMultirotor = sys::ODID_uatype_ODID_UATYPE_HELICOPTER_OR_MULTIROTOR,
    Gyroplane = sys::ODID_uatype_ODID_UATYPE_GYROPLANE,
    HybridLift = sys::ODID_uatype_ODID_UATYPE_HYBRID_LIFT,
    Ornithopter = sys::ODID_uatype_ODID_UATYPE_ORNITHOPTER,
    Glider = sys::ODID_uatype_ODID_UATYPE_GLIDER,
    Kite = sys::ODID_uatype_ODID_UATYPE_KITE,
    FreeBalloon = sys::ODID_uatype_ODID_UATYPE_FREE_BALLOON,
    CaptiveBalloon = sys::ODID_uatype_ODID_UATYPE_CAPTIVE_BALLOON,
    Airship = sys::ODID_uatype_ODID_UATYPE_AIRSHIP,
    FreeFallParachute = sys::ODID_uatype_ODID_UATYPE_FREE_FALL_PARACHUTE,
    Rocket = sys::ODID_uatype_ODID_UATYPE_ROCKET,
    TetheredPoweredAircraft = sys::ODID_uatype_ODID_UATYPE_TETHERED_POWERED_AIRCRAFT,
    GroundObstacle = sys::ODID_uatype_ODID_UATYPE_GROUND_OBSTACLE,
    Other = sys::ODID_uatype_ODID_UATYPE_OTHER,
}

/// Status of the UA reported in the [Location] message. Accessible through [Location::status].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum Status {
    #[default]
    Undeclared = sys::ODID_status_ODID_STATUS_UNDECLARED,
    Ground = sys::ODID_status_ODID_STATUS_GROUND,
    Airborne = sys::ODID_status_ODID_STATUS_AIRBORNE,
    Emergency = sys::ODID_status_ODID_STATUS_EMERGENCY,
    RemoteIdSystemFailure = sys::ODID_status_ODID_STATUS_REMOTE_ID_SYSTEM_FAILURE,
}

/// Reference for the [Location::height] of the UA provided by the [Location] message.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum HeightReference {
    /// Indicates that the height is relative to the takeoff location of the UA.
    #[default]
    Takeoff = sys::ODID_Height_reference_ODID_HEIGHT_REF_OVER_TAKEOFF,
    /// Indicates that the height is relative to the ground level at the current location of the UA.
    Ground = sys::ODID_Height_reference_ODID_HEIGHT_REF_OVER_GROUND,
}

/// Accuracy of horizontal position information in the [Location] message.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
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

/// Accuracy of the [Location::altitude_barometric] and [Location::altitude_geodetic] fields in the [Location] message.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
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

/// Accuracy of the [Location::speed_horizontal] and [Location::speed_vertical] fields in the
/// [Location] message.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum SpeedAccuracy {
    #[default]
    Unknown = sys::ODID_Speed_accuracy_ODID_SPEED_ACC_UNKNOWN,
    LessThan10MetersPerSecond = sys::ODID_Speed_accuracy_ODID_SPEED_ACC_10_METERS_PER_SECOND,
    LessThan3MetersPerSecond = sys::ODID_Speed_accuracy_ODID_SPEED_ACC_3_METERS_PER_SECOND,
    LessThan1MetersPerSecond = sys::ODID_Speed_accuracy_ODID_SPEED_ACC_1_METERS_PER_SECOND,
    LessThan0_3MetersPerSecond = sys::ODID_Speed_accuracy_ODID_SPEED_ACC_0_3_METERS_PER_SECOND,
}

/// Accuracy of the [Location::timestamp] information in the [Location] message.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
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

/// Type of authentication used in the [Auth] message.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum AuthenticationType {
    #[default]
    None = sys::ODID_authtype_ODID_AUTH_NONE,
    UasIdSignature = sys::ODID_authtype_ODID_AUTH_UAS_ID_SIGNATURE,
    OperatorIdSignature = sys::ODID_authtype_ODID_AUTH_OPERATOR_ID_SIGNATURE,
    MessageSetSignature = sys::ODID_authtype_ODID_AUTH_MESSAGE_SET_SIGNATURE,
    NetworkRemoteId = sys::ODID_authtype_ODID_AUTH_NETWORK_REMOTE_ID,
    SpecificAuthentication = sys::ODID_authtype_ODID_AUTH_SPECIFIC_AUTHENTICATION,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum DescriptionType {
    #[default]
    Text = sys::ODID_desctype_ODID_DESC_TYPE_TEXT,
    Emergency = sys::ODID_desctype_ODID_DESC_TYPE_EMERGENCY,
    ExtendedStatus = sys::ODID_desctype_ODID_DESC_TYPE_EXTENDED_STATUS,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum OperatorIdType {
    #[default]
    OperatorId = sys::ODID_operatorIdType_ODID_OPERATOR_ID,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum OperatorLocationType {
    #[default]
    Takeoff = sys::ODID_operator_location_type_ODID_OPERATOR_LOCATION_TYPE_TAKEOFF,
    LiveGnss = sys::ODID_operator_location_type_ODID_OPERATOR_LOCATION_TYPE_LIVE_GNSS,
    Fixed = sys::ODID_operator_location_type_ODID_OPERATOR_LOCATION_TYPE_FIXED,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum ClassificationType {
    #[default]
    Undeclared = sys::ODID_classification_type_ODID_CLASSIFICATION_TYPE_UNDECLARED,
    EuropeanUnion = sys::ODID_classification_type_ODID_CLASSIFICATION_TYPE_EU,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum Category {
    #[default]
    Undeclared = sys::ODID_category_EU_ODID_CATEGORY_EU_UNDECLARED,
    Open = sys::ODID_category_EU_ODID_CATEGORY_EU_OPEN,
    Specific = sys::ODID_category_EU_ODID_CATEGORY_EU_SPECIFIC,
    Certified = sys::ODID_category_EU_ODID_CATEGORY_EU_CERTIFIED,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum ClassEu {
    #[default]
    Undeclared = sys::ODID_class_EU_ODID_CLASS_EU_UNDECLARED,
    Class0 = sys::ODID_class_EU_ODID_CLASS_EU_CLASS_0,
    Class1 = sys::ODID_class_EU_ODID_CLASS_EU_CLASS_1,
    Class2 = sys::ODID_class_EU_ODID_CLASS_EU_CLASS_2,
    Class3 = sys::ODID_class_EU_ODID_CLASS_EU_CLASS_3,
    Class4 = sys::ODID_class_EU_ODID_CLASS_EU_CLASS_4,
    Class5 = sys::ODID_class_EU_ODID_CLASS_EU_CLASS_5,
    Class6 = sys::ODID_class_EU_ODID_CLASS_EU_CLASS_6,
}

/// Common trait for basic Open Drone ID messages, providing encoding and decoding functionality.
pub trait Message: Sized {
    /// Type of the decoded message data structure
    type Data;
    /// Type of the encoded message format
    type Encoded;

    /// Returns the length of the encoded message in bytes.
    #[inline]
    fn encoded_len(&self) -> usize {
        std::mem::size_of::<Self::Encoded>()
    }

    /// Encodes the message to a buffer.
    ///
    /// An error will be returned if the buffer does not have sufficient capacity.
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), Error>;

    /// Encodes the message to a newly allocated buffer.
    fn encode_to_vec(&self) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;
        Ok(buf)
    }

    /// Decodes the message from a buffer.
    fn decode(buf: impl Buf) -> Result<Self, Error>;
}

/// Internal trait for encoding and decoding messages using the underlying C library functions.
trait MessageInternal: Sized {
    /// Type of the decoded message data structure
    type Data;
    /// Type of the encoded message format
    type Encoded;

    /// Initializes the message data structure to default values.
    fn init_data(data: *mut Self::Data);

    /// Encodes the message data to the encoded format using the underlying C library function.
    fn encode_message(
        out_encoded: *mut Self::Encoded,
        in_data: *const Self::Data,
    ) -> Result<(), Error>;

    /// Decodes the message data from the encoded format using the underlying C library function.
    fn decode_message(
        out_data: *mut Self::Data,
        in_encoded: *const Self::Encoded,
    ) -> Result<(), Error>;
}

macros::impl_message!(
    /// Basic ID message for UA identification.
    ///
    /// ``` rust
    /// use opendroneid::BasicId;
    ///
    /// // Create a new Basic ID message with default values.
    /// // The default values are defined by the underlying C library and may not be valid.
    /// let basic_id = BasicId::default()
    ///     // Set fields of the Basic ID message using the builder pattern.
    ///     .with_ua_type(opendroneid::UaType::Aeroplane)
    ///     .with_id_type(opendroneid::IdType::SerialNumber)
    ///     .with_uas_id("TEST1234567890")
    ///     .expect("Invalid UAS ID value");
    /// ```
    BasicId,
    sys::ODID_BasicID_data,
    sys::ODID_BasicID_encoded,
    sys::odid_initBasicIDData,
    sys::encodeBasicIDMessage,
    sys::decodeBasicIDMessage
);

impl BasicId {
    /// Returns the UA type of the message, or an error if the value is invalid.
    pub fn ua_type(&self) -> Result<UaType, Error> {
        UaType::from_u32(self.data.UAType).ok_or(Error::EnumMappingError {
            field: "UAType",
            value: self.data.UAType,
        })
    }
    /// Sets the UA type of the message.
    pub fn with_ua_type(mut self, ua_type: UaType) -> Self {
        self.data.UAType = ua_type as u32;
        self
    }
    /// Returns the ID type of the message, or an error if the value is invalid.
    pub fn id_type(&self) -> Result<IdType, Error> {
        IdType::from_u32(self.data.IDType).ok_or(Error::EnumMappingError {
            field: "IDType",
            value: self.data.IDType,
        })
    }
    /// Sets the ID type of the message.
    pub fn with_id_type(mut self, id_type: IdType) -> Self {
        self.data.IDType = id_type as u32;
        self
    }
    /// Returns the UAS ID of the message as a string.
    pub fn uas_id(&self) -> String {
        utils::c_string_to_rust(&self.data.UASID)
    }
    /// Sets the UAS ID of the message.
    /// The string must be encodable to less than 21 bytes in UTF-8 encoding.
    pub fn with_uas_id(mut self, uas_id: &str) -> Result<BasicId, Error> {
        utils::set_c_string(uas_id, &mut self.data.UASID).map(|_| self)
    }
}

macros::impl_message!(
    /// Location message as defined in the underlying C library.
    ///
    /// ```rust
    /// use opendroneid::Location;
    ///
    /// // Create a new Location message with default values.
    /// let location = Location::default()
    ///     // Set fields of the Location message using the builder pattern.
    ///     .with_status(opendroneid::Status::Airborne)
    ///     .with_latitude(37.7749)
    ///     .expect("Invalid latitude value")
    ///     .with_longitude(-77.4194)
    ///     .expect("Invalid longitude value")
    ///     .with_direction(90.0)
    ///     .expect("Invalid direction value")
    ///     .with_speed_horizontal(5.0);
    /// ```
    Location,
    sys::ODID_Location_data,
    sys::ODID_Location_encoded,
    sys::odid_initLocationData,
    sys::encodeLocationMessage,
    sys::decodeLocationMessage
);

impl Location {
    /// Returns the status of the message, or an error if the value is invalid.
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
    /// Returns the direction of the UA in degrees from north, or None if the value is invalid.
    pub fn direction(&self) -> Option<f32> {
        if self.data.Direction == sys::INV_DIR as f32 {
            None
        } else {
            Some(self.data.Direction)
        }
    }
    /// Sets the direction of the UA in degrees from north.
    pub fn with_direction(mut self, direction: f32) -> Result<Self, Error> {
        if direction < MIN_DIRECTION as f32 || direction > MAX_DIRECTION as f32 {
            return Err(Error::InvalidValue {
                field: "direction",
                value: direction.to_string(),
            });
        }
        self.data.Direction = direction;
        Ok(self)
    }
    /// Returns the horizontal speed of the UA in m/s, or None if the value is invalid.
    pub fn speed_horizontal(&self) -> Option<f32> {
        if self.data.SpeedHorizontal == sys::INV_SPEED_H as f32 {
            None
        } else {
            Some(self.data.SpeedHorizontal)
        }
    }
    /// Sets the horizontal speed of the UA in m/s.
    /// Returns an error if the speed is out of the valid range defined by the protocol.
    pub fn with_speed_horizontal(mut self, speed: f32) -> Result<Self, Error> {
        if speed < MIN_SPEED_HORIZONTAL as f32 || speed > MAX_SPEED_HORIZONTAL as f32 {
            return Err(Error::InvalidValue {
                field: "speed_horizontal",
                value: speed.to_string(),
            });
        }
        self.data.SpeedHorizontal = speed;
        Ok(self)
    }
    /// Returns the vertical speed of the UA in m/s, or None if the value is invalid.
    pub fn speed_vertical(&self) -> Option<f32> {
        if self.data.SpeedVertical == sys::INV_SPEED_V as f32 {
            None
        } else {
            Some(self.data.SpeedVertical)
        }
    }
    /// Sets the vertical speed of the UA in m/s.
    pub fn with_speed_vertical(mut self, speed: f32) -> Result<Self, Error> {
        if speed < MIN_SPEED_VERTICAL as f32 || speed > MAX_SPEED_VERTICAL as f32 {
            return Err(Error::InvalidValue {
                field: "speed_vertical",
                value: speed.to_string(),
            });
        }
        self.data.SpeedVertical = speed;
        Ok(self)
    }
    /// Returns the latitude of the UA in degrees.
    pub fn latitude(&self) -> f64 {
        self.data.Latitude
    }
    /// Sets the latitude of the UA in degrees.
    /// Returns an error if the latitude is out of the valid range defined by the protocol.
    pub fn with_latitude(mut self, latitude: f64) -> Result<Self, Error> {
        if latitude < MIN_LATITUDE as f64 || latitude > MAX_LATITUDE as f64 {
            return Err(Error::InvalidValue {
                field: "latitude",
                value: latitude.to_string(),
            });
        }
        self.data.Latitude = latitude;
        Ok(self)
    }
    /// Returns the longitude of the UA in degrees.
    pub fn longitude(&self) -> f64 {
        self.data.Longitude
    }
    /// Sets the longitude of the UA in degrees.
    /// Returns an error if the longitude is out of the valid range defined by the protocol.
    pub fn with_longitude(mut self, longitude: f64) -> Result<Self, Error> {
        if longitude < MIN_LONGITUDE as f64 || longitude > MAX_LONGITUDE as f64 {
            return Err(Error::InvalidValue {
                field: "longitude",
                value: longitude.to_string(),
            });
        }
        self.data.Longitude = longitude;
        Ok(self)
    }
    /// Returns the barometric altitude of the UA in meters, or None if the value is invalid.
    ///
    /// The barometric altitude is the uncorrected barometric pressure altitude
    /// (based on reference standard 29.92 inHg, 1013.25 mb).
    pub fn altitude_barometric(&self) -> Option<f32> {
        if self.data.AltitudeBaro == sys::INV_ALT as f32 {
            None
        } else {
            Some(self.data.AltitudeBaro)
        }
    }
    /// Sets the barometric altitude of the UA in meters.
    /// Returns an error if the altitude is out of the valid range defined by the protocol.
    pub fn with_altitude_barometric(mut self, altitude: f32) -> Result<Self, Error> {
        if altitude < MIN_ALTITUDE as f32 || altitude > MAX_ALTITUDE as f32 {
            return Err(Error::InvalidValue {
                field: "altitude_barometric",
                value: altitude.to_string(),
            });
        }
        self.data.AltitudeBaro = altitude;
        Ok(self)
    }
    /// Returns the geodetic altitude of the UA in meters, or None if the value is invalid.
    ///
    /// The geodetic altitude is the distance above or below the surface of the WGS-84 ellipsoid.
    pub fn altitude_geodetic(&self) -> Option<f32> {
        if self.data.AltitudeGeo == sys::INV_ALT as f32 {
            None
        } else {
            Some(self.data.AltitudeGeo)
        }
    }
    /// Sets the geodetic altitude of the UA in meters.
    /// Returns an error if the altitude is out of the valid range defined by the protocol.
    ///
    /// The geodetic altitude is the distance above or below the surface of the WGS-84 ellipsoid.
    pub fn with_altitude_geodetic(mut self, altitude: f32) -> Result<Self, Error> {
        if altitude < sys::MIN_ALT as f32 || altitude > sys::MAX_ALT as f32 {
            return Err(Error::InvalidValue {
                field: "altitude_geodetic",
                value: altitude.to_string(),
            });
        }
        self.data.AltitudeGeo = altitude;
        Ok(self)
    }
    /// Returns the height reference of the UA, or an error if the value is invalid.
    pub fn height_type(&self) -> Result<HeightReference, Error> {
        HeightReference::from_u32(self.data.HeightType).ok_or(Error::EnumMappingError {
            field: "HeightType",
            value: self.data.HeightType,
        })
    }
    /// Sets the height reference of the UA.
    pub fn with_height_type(mut self, height_type: HeightReference) -> Self {
        self.data.HeightType = height_type as u32;
        self
    }
    /// Returns the height of the UA in meters, or None if the value is invalid.
    pub fn height(&self) -> Option<f32> {
        if self.data.Height == sys::INV_ALT as f32 {
            None
        } else {
            Some(self.data.Height)
        }
    }
    /// Sets the height of the UA in meters.
    /// Returns an error if the height is out of the valid range defined by the protocol.
    pub fn with_height(mut self, height: f32) -> Result<Self, Error> {
        if height < MIN_ALTITUDE as f32 || height > MAX_ALTITUDE as f32 {
            return Err(Error::InvalidValue {
                field: "height",
                value: height.to_string(),
            });
        }
        self.data.Height = height;
        Ok(self)
    }
    /// Returns the horizontal accuracy of the location information, or an error if the value is invalid.
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
    /// Returns the vertical accuracy of the location information, or an error if the value is invalid.
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
    /// Returns the barometric accuracy of the location information, or an error if the value is invalid.
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
    /// Returns the speed accuracy of the location information, or an error if the value is invalid.
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
    /// Returns the timestamp accuracy of the location information, or an error if the value is invalid.
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
        if self.data.TimeStamp == sys::INV_TIMESTAMP as f32 {
            None
        } else {
            Some(self.data.TimeStamp)
        }
    }

    /// Sets the timestamp as a floating point number of seconds since the start of the hour.
    /// Returns an error if the timestamp is out of the valid range defined by the protocol.
    pub fn with_timestamp(mut self, timestamp: f32) -> Result<Self, Error> {
        if timestamp < 0 as f32 || timestamp > sys::MAX_TIMESTAMP as f32 {
            return Err(Error::InvalidValue {
                field: "timestamp",
                value: timestamp.to_string(),
            });
        }
        self.data.TimeStamp = timestamp;
        Ok(self)
    }

    /// Returns the timestamp as a `chrono::DateTime<chrono::Utc>`, or None if the value is invalid.
    ///
    /// <div class="warning">
    /// The value of the timestamp is only valid within one hour of timestamp creation.
    /// The internally stored value is relative to the start of the current hour, so, if more than
    /// one hour has passed since the timestamp was created, the returned value may be inaccurate.
    /// </div>
    ///
    #[cfg(feature = "chrono")]
    pub fn chrono_timestamp(&self) -> Option<Result<chrono::DateTime<chrono::Utc>, Error>> {
        if self.data.TimeStamp == sys::INV_TIMESTAMP as f32 {
            None
        } else {
            Some(decode_timestamp(self.data.TimeStamp))
        }
    }
    /// Sets the timestamp using a `chrono::DateTime<chrono::Utc>`.
    #[cfg(feature = "chrono")]
    pub fn with_chrono_timestamp(mut self, timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        self.data.TimeStamp = encode_timestamp(timestamp);
        self
    }
}

#[cfg(feature = "chrono")]
fn decode_timestamp(value: f32) -> Result<chrono::DateTime<chrono::Utc>, Error> {
    use chrono::Timelike;
    let now = chrono::Utc::now();

    let this_hour_start = now
        .with_minute(0)
        .ok_or(Error::InvalidValue {
            field: "timestamp.minute",
            value: "0".to_string(),
        })?
        .with_second(0)
        .ok_or(Error::InvalidValue {
            field: "timestamp.second",
            value: "0".to_string(),
        })?
        .with_nanosecond(0)
        .ok_or(Error::InvalidValue {
            field: "timestamp.nanosecond",
            value: "0".to_string(),
        })?;
    let mins = now.minute();
    let secs = now.second();
    let nanos = now.nanosecond();
    let now_secs = (mins as f32 * 60.0) + (secs as f32) + (nanos as f32 / 1_000_000_000.0);

    let base_hour = if value > now_secs {
        this_hour_start - chrono::Duration::hours(1)
    } else {
        this_hour_start
    };

    base_hour
        .with_minute((value / 60.0) as u32)
        .ok_or(Error::InvalidValue {
            field: "timestamp.minute",
            value: value.to_string(),
        })?
        .with_second((value % 60.0) as u32)
        .ok_or(Error::InvalidValue {
            field: "timestamp.second",
            value: value.to_string(),
        })?
        .with_nanosecond(((value % 1.0) * 1_000_000_000.0) as u32)
        .ok_or(Error::InvalidValue {
            field: "timestamp.nanosecond",
            value: value.to_string(),
        })
}

#[cfg(feature = "chrono")]
fn encode_timestamp(value: chrono::DateTime<chrono::Utc>) -> f32 {
    use chrono::Timelike;
    let mins = value.minute();
    let secs = value.second();
    let nanos = value.nanosecond();
    (mins as f32 * 60.0) + (secs as f32) + (nanos as f32 / 1_000_000_000.0)
}

/// Number of seconds from 1970-01-01 to 2019-01-01
#[cfg(feature = "chrono")]
const EPOCH_OFFSET: i64 = 1546300800;

macros::impl_message!(
    /// The Authentication Message defines a field that provides a means for authenticating the
    /// identity of the UA sending the message.
    ///
    /// ```rust
    /// use opendroneid::Auth;
    ///
    /// // Create a new Auth message with default values.
    /// let auth = Auth::default()
    ///     // Set fields of the Auth message using the builder pattern.
    ///     .with_auth_type(opendroneid::AuthenticationType::UasIdSignature)
    ///     .with_timestamp(12345);
    /// ```
    Auth,
    sys::ODID_Auth_data,
    sys::ODID_Auth_encoded,
    sys::odid_initAuthData,
    sys::encodeAuthMessage,
    sys::decodeAuthMessage
);

impl Auth {
    /// Returns the authentication data page index.
    pub fn data_page(&self) -> u8 {
        self.data.DataPage
    }
    /// Sets the authentication data page index.
    pub fn with_data_page(mut self, data_page: u8) -> Result<Self, Error> {
        if data_page >= sys::ODID_AUTH_MAX_PAGES as u8 {
            return Err(Error::InvalidValue {
                field: "data_page",
                value: data_page.to_string(),
            });
        }
        self.data.DataPage = data_page;
        Ok(self)
    }
    /// Returns the authentication type of the message, or an error if the value is invalid.
    pub fn auth_type(&self) -> Result<AuthenticationType, Error> {
        AuthenticationType::from_u32(self.data.AuthType).ok_or(Error::EnumMappingError {
            field: "AuthType",
            value: self.data.AuthType,
        })
    }

    /// Sets the authentication type of the message.
    pub fn with_auth_type(mut self, auth_type: AuthenticationType) -> Self {
        self.data.AuthType = auth_type as u32;
        self
    }

    /// Returns the last authentication page index.
    pub fn last_page_index(&self) -> u8 {
        self.data.LastPageIndex
    }

    /// Sets the last authentication page index.
    pub fn with_last_page_index(mut self, last_page_index: u8) -> Result<Self, Error> {
        if last_page_index >= sys::ODID_AUTH_MAX_PAGES as u8 {
            return Err(Error::InvalidValue {
                field: "last_page_index",
                value: last_page_index.to_string(),
            });
        }

        if self.data.DataPage == 0 {
            let max_len = sys::ODID_AUTH_PAGE_ZERO_DATA_SIZE
                + last_page_index as u32 * sys::ODID_AUTH_PAGE_NONZERO_DATA_SIZE;
            if self.data.Length as u32 > max_len {
                return Err(Error::InvalidValue {
                    field: "length",
                    value: self.data.Length.to_string(),
                });
            }
        }

        self.data.LastPageIndex = last_page_index;
        Ok(self)
    }

    /// Returns the authentication data length in bytes.
    pub fn length(&self) -> u8 {
        self.data.Length
    }

    /// Sets the authentication data length in bytes.
    pub fn with_length(mut self, length: u8) -> Result<Self, Error> {
        if self.data.DataPage == 0 {
            let max_len = sys::ODID_AUTH_PAGE_ZERO_DATA_SIZE
                + self.data.LastPageIndex as u32 * sys::ODID_AUTH_PAGE_NONZERO_DATA_SIZE;
            if length as u32 > max_len {
                return Err(Error::InvalidValue {
                    field: "length",
                    value: length.to_string(),
                });
            }
        }

        self.data.Length = length;
        Ok(self)
    }

    /// Returns the authentication timestamp value.
    ///
    /// 32 bit Unix Timestamp (UTC) in seconds since (epoch) 00:00:00 01/01/2019 (to re-
    /// late back to standard Unix timestamp, add 1546300800 to the common epoch of
    /// 00:00:00 01/01/1970)
    pub fn timestamp(&self) -> u32 {
        self.data.Timestamp
    }

    /// Sets the authentication timestamp value.
    pub fn with_timestamp(mut self, timestamp: u32) -> Self {
        self.data.Timestamp = timestamp;
        self
    }

    #[cfg(feature = "chrono")]
    /// Returns the authentication timestamp as a `chrono::DateTime<Utc>`.
    /// Returns None if the timestamp value is invalid.
    pub fn chrono_timestamp(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        chrono::DateTime::<chrono::Utc>::from_timestamp(
            self.data.Timestamp as i64 + EPOCH_OFFSET,
            0,
        )
    }

    #[cfg(feature = "chrono")]
    /// Sets the authentication timestamp using a `chrono::DateTime<Utc>`.
    pub fn with_chrono_timestamp(mut self, timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        self.data.Timestamp = (timestamp.timestamp() - EPOCH_OFFSET) as u32;
        self
    }

    /// Returns the authentication data payload bytes.
    pub fn auth_data(&self) -> &[u8; 24] {
        &self.data.AuthData
    }

    /// Sets the authentication data payload bytes.
    pub fn with_auth_data(mut self, auth_data: [u8; 24]) -> Self {
        self.data.AuthData = auth_data;
        self
    }
}

macros::impl_message!(
    /// The Self-ID Message is an opportunity for the Remote Pilot to declare their identity
    /// or purpose (intent) of the flight. This message can serve to reduce the perceived threat
    /// of a UA flying in a particular area or manner.
    ///
    /// ```rust
    /// use opendroneid::SelfId;
    ///
    /// // Create a new Self-ID message with default values.
    /// let self_id = SelfId::default()
    ///     // Set fields of the Self-ID message using the builder pattern.
    ///     .with_desc_type(opendroneid::DescriptionType::Emergency)
    ///     .with_desc("OBSERVATION").expect("Invalid description value");
    /// ```
    SelfId,
    sys::ODID_SelfID_data,
    sys::ODID_SelfID_encoded,
    sys::odid_initSelfIDData,
    sys::encodeSelfIDMessage,
    sys::decodeSelfIDMessage
);

impl SelfId {
    /// Returns the description type of the message, or an error if the value is invalid.
    pub fn desc_type(&self) -> Result<DescriptionType, Error> {
        DescriptionType::from_u32(self.data.DescType).ok_or(Error::EnumMappingError {
            field: "DescType",
            value: self.data.DescType,
        })
    }

    /// Sets the description type of the message.
    pub fn with_desc_type(mut self, desc_type: DescriptionType) -> Self {
        self.data.DescType = desc_type as u32;
        self
    }

    /// Returns the description payload as a string.
    pub fn desc(&self) -> String {
        utils::c_string_to_rust(&self.data.Desc)
    }

    /// Sets the description payload as a string.
    /// The string must be encodable to less than 24 bytes in UTF-8 encoding.
    pub fn with_desc(mut self, desc: &str) -> Result<Self, Error> {
        utils::set_c_string(desc, &mut self.data.Desc).map(|_| self)
    }
}

macros::impl_message!(
    /// The System Message contains general system information including information about the
    /// Remote Pilot location and flight area. It can also include information about UAs flying
    /// in formation.
    System,
    sys::ODID_System_data,
    sys::ODID_System_encoded,
    sys::odid_initSystemData,
    sys::encodeSystemMessage,
    sys::decodeSystemMessage
);

impl System {
    /// Returns the operator location type of the message, or an error if the value is invalid.
    pub fn operator_location_type(&self) -> Result<OperatorLocationType, Error> {
        OperatorLocationType::from_u32(self.data.OperatorLocationType).ok_or(
            Error::EnumMappingError {
                field: "OperatorLocationType",
                value: self.data.OperatorLocationType,
            },
        )
    }

    /// Sets the operator location type of the message.
    pub fn with_operator_location_type(mut self, location_type: OperatorLocationType) -> Self {
        self.data.OperatorLocationType = location_type as u32;
        self
    }

    /// Returns the classification type of the message, or an error if the value is invalid.
    pub fn classification_type(&self) -> Result<ClassificationType, Error> {
        ClassificationType::from_u32(self.data.ClassificationType).ok_or(Error::EnumMappingError {
            field: "ClassificationType",
            value: self.data.ClassificationType,
        })
    }

    /// Sets the classification type of the message.
    pub fn with_classification_type(mut self, classification_type: ClassificationType) -> Self {
        self.data.ClassificationType = classification_type as u32;
        self
    }

    /// Returns the operator latitude in degrees.
    pub fn operator_latitude(&self) -> f64 {
        self.data.OperatorLatitude
    }

    /// Sets the operator latitude in degrees.
    pub fn with_operator_latitude(mut self, latitude: f64) -> Result<Self, Error> {
        if latitude < MIN_LATITUDE as f64 || latitude > MAX_LATITUDE as f64 {
            return Err(Error::InvalidValue {
                field: "operator_latitude",
                value: latitude.to_string(),
            });
        }
        self.data.OperatorLatitude = latitude;
        Ok(self)
    }

    /// Returns the operator longitude in degrees.
    pub fn operator_longitude(&self) -> f64 {
        self.data.OperatorLongitude
    }

    /// Sets the operator longitude in degrees.
    pub fn with_operator_longitude(mut self, longitude: f64) -> Result<Self, Error> {
        if longitude < MIN_LONGITUDE as f64 || longitude > MAX_LONGITUDE as f64 {
            return Err(Error::InvalidValue {
                field: "operator_longitude",
                value: longitude.to_string(),
            });
        }
        self.data.OperatorLongitude = longitude;
        Ok(self)
    }

    /// Returns the area count value.
    pub fn area_count(&self) -> u16 {
        self.data.AreaCount
    }

    /// Sets the area count value.
    pub fn with_area_count(mut self, area_count: u16) -> Self {
        self.data.AreaCount = area_count;
        self
    }

    /// Returns the area radius in meters.
    pub fn area_radius(&self) -> u16 {
        self.data.AreaRadius
    }

    /// Sets the area radius in meters.
    pub fn with_area_radius(mut self, area_radius: u16) -> Result<Self, Error> {
        if area_radius > sys::MAX_AREA_RADIUS as u16 {
            return Err(Error::InvalidValue {
                field: "area_radius",
                value: area_radius.to_string(),
            });
        }
        self.data.AreaRadius = area_radius;
        Ok(self)
    }

    /// Returns the area ceiling altitude in meters.
    pub fn area_ceiling(&self) -> f32 {
        self.data.AreaCeiling
    }

    /// Sets the area ceiling altitude in meters.
    pub fn with_area_ceiling(mut self, area_ceiling: f32) -> Result<Self, Error> {
        if area_ceiling < MIN_ALTITUDE as f32 || area_ceiling > MAX_ALTITUDE as f32 {
            return Err(Error::InvalidValue {
                field: "area_ceiling",
                value: area_ceiling.to_string(),
            });
        }
        self.data.AreaCeiling = area_ceiling;
        Ok(self)
    }

    /// Returns the area floor altitude in meters.
    pub fn area_floor(&self) -> f32 {
        self.data.AreaFloor
    }

    /// Sets the area floor altitude in meters.
    pub fn with_area_floor(mut self, area_floor: f32) -> Result<Self, Error> {
        if area_floor < MIN_ALTITUDE as f32 || area_floor > MAX_ALTITUDE as f32 {
            return Err(Error::InvalidValue {
                field: "area_floor",
                value: area_floor.to_string(),
            });
        }
        self.data.AreaFloor = area_floor;
        Ok(self)
    }

    /// Returns the EU category of the message, or an error if the value is invalid.
    pub fn category(&self) -> Result<Category, Error> {
        Category::from_u32(self.data.CategoryEU).ok_or(Error::EnumMappingError {
            field: "CategoryEU",
            value: self.data.CategoryEU,
        })
    }

    /// Sets the EU category of the message.
    pub fn with_category(mut self, category: Category) -> Self {
        self.data.CategoryEU = category as u32;
        self
    }

    /// Returns the EU class of the message, or an error if the value is invalid.
    pub fn class_eu(&self) -> Result<ClassEu, Error> {
        ClassEu::from_u32(self.data.ClassEU).ok_or(Error::EnumMappingError {
            field: "ClassEU",
            value: self.data.ClassEU,
        })
    }

    /// Sets the EU class of the message.
    pub fn with_class_eu(mut self, class_eu: ClassEu) -> Self {
        self.data.ClassEU = class_eu as u32;
        self
    }

    /// Returns the operator geodetic altitude in meters.
    pub fn operator_altitude_geo(&self) -> f32 {
        self.data.OperatorAltitudeGeo
    }

    /// Sets the operator geodetic altitude in meters.
    pub fn with_operator_altitude_geo(mut self, altitude: f32) -> Result<Self, Error> {
        if altitude < MIN_ALTITUDE as f32 || altitude > MAX_ALTITUDE as f32 {
            return Err(Error::InvalidValue {
                field: "operator_altitude_geo",
                value: altitude.to_string(),
            });
        }
        self.data.OperatorAltitudeGeo = altitude;
        Ok(self)
    }

    /// Returns the timestamp value.
    pub fn timestamp(&self) -> u32 {
        self.data.Timestamp
    }

    /// Sets the timestamp value.
    pub fn with_timestamp(mut self, timestamp: u32) -> Self {
        self.data.Timestamp = timestamp;
        self
    }
}

macros::impl_message!(
    /// The Operator ID Message contains the CAA issued Operator ID formatted as described
    /// in Operator ID.
    ///
    /// ```rust
    /// use opendroneid::OperatorId;
    ///
    /// // Create a new Operator ID message with default values.
    /// let operator_id = OperatorId::default()
    ///      // Set fields of the Operator ID message using the builder pattern.
    ///     .with_operator_id_type(opendroneid::OperatorIdType::OperatorId)
    ///     .with_operator_id("TEST").expect("Invalid operator ID value");
    /// ```
    OperatorId,
    sys::ODID_OperatorID_data,
    sys::ODID_OperatorID_encoded,
    sys::odid_initOperatorIDData,
    sys::encodeOperatorIDMessage,
    sys::decodeOperatorIDMessage
);

impl OperatorId {
    /// Returns the operator ID type of the message, or an error if the value is invalid.
    pub fn operator_id_type(&self) -> Result<OperatorIdType, Error> {
        OperatorIdType::from_u32(self.data.OperatorIdType).ok_or(Error::EnumMappingError {
            field: "OperatorIdType",
            value: self.data.OperatorIdType,
        })
    }

    /// Sets the operator ID type of the message.
    pub fn with_operator_id_type(mut self, operator_id_type: OperatorIdType) -> Self {
        self.data.OperatorIdType = operator_id_type as u32;
        self
    }

    /// Returns the operator ID payload as a byte array.
    pub fn operator_id(&self) -> String {
        utils::c_string_to_rust(&self.data.OperatorId)
    }

    /// Sets the operator ID payload as a string.
    /// The string must be encodable to less than 21 bytes in UTF-8 encoding.
    pub fn with_operator_id(mut self, operator_id: &str) -> Result<Self, Error> {
        utils::set_c_string(operator_id, &mut self.data.OperatorId).map(|_| self)
    }
}

/// UAS data is a collection of Open Drone ID messages that together represent the state and identity of a UAS.
/// It may contain multiple Basic ID and Auth messages, but at most one of each of the other message types.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct UasData {
    /// The Basic ID messages associated with this UAS data.
    /// There may be multiple Basic ID messages, but at least one is required.
    pub basic_id: Vec<BasicId>,
    /// The Location message associated with this UAS data, if available.
    pub location: Option<Location>,
    /// The Auth messages associated with this UAS data.
    pub auth: Vec<Auth>,
    /// The Self ID message associated with this UAS data, if available.
    pub self_id: Option<SelfId>,
    /// The System message associated with this UAS data, if available.
    pub system: Option<System>,
    /// The Operator ID message associated with this UAS data, if available.
    pub operator_id: Option<OperatorId>,
}

impl UasData {
    /// Get the Basic ID messages associated with this UAS data.
    pub fn basic_id(&self) -> &[BasicId] {
        &self.basic_id
    }

    /// Set the Basic ID messages associated with this UAS data.
    pub fn with_basic_id(mut self, basic_id: Vec<BasicId>) -> Self {
        self.basic_id = basic_id;
        self
    }

    /// Get the Location message associated with this UAS data, if available.
    pub fn location(&self) -> Option<&Location> {
        self.location.as_ref()
    }

    /// Set the Location message associated with this UAS data.
    pub fn with_location(mut self, location: Option<Location>) -> Self {
        self.location = location;
        self
    }

    /// Get the Auth messages associated with this UAS data.
    pub fn auth(&self) -> &[Auth] {
        &self.auth
    }

    /// Set the Auth messages associated with this UAS data.
    pub fn with_auth(mut self, auth: Vec<Auth>) -> Self {
        self.auth = auth;
        self
    }

    /// Get the Self ID message associated with this UAS data, if available.
    pub fn self_id(&self) -> Option<&SelfId> {
        self.self_id.as_ref()
    }
    /// Set the Self ID message associated with this UAS data.
    pub fn with_self_id(mut self, self_id: Option<SelfId>) -> Self {
        self.self_id = self_id;
        self
    }

    /// Get the System message associated with this UAS data, if available.
    pub fn system(&self) -> Option<&System> {
        self.system.as_ref()
    }

    /// Set the System message associated with this UAS data.
    pub fn with_system(mut self, system: Option<System>) -> Self {
        self.system = system;
        self
    }

    /// Get the Operator ID message associated with this UAS data, if available.
    pub fn operator_id(&self) -> Option<&OperatorId> {
        self.operator_id.as_ref()
    }

    /// Set the Operator ID message associated with this UAS data.
    pub fn with_operator_id(mut self, operator_id: Option<OperatorId>) -> Self {
        self.operator_id = operator_id;
        self
    }

    /// Decode UAS data from a buffer.
    pub fn decode(buf: impl Buf) -> Result<Self, Error> {
        let mut data = sys::ODID_UAS_Data::default();

        let r = unsafe {
            sys::decodeOpenDroneID(&mut data as *mut sys::ODID_UAS_Data, buf.chunk().as_ptr())
        };

        if r == sys::ODID_messagetype_ODID_MESSAGETYPE_INVALID {
            return Err(Error::EnumMappingError {
                field: "MessageType",
                value: buf.chunk()[0] as u32,
            });
        }
        let basic_id = data
            .BasicIDValid
            .iter()
            .enumerate()
            .filter_map(|(idx, valid)| {
                if *valid != 0 {
                    Some(BasicId {
                        data: data.BasicID[idx],
                    })
                } else {
                    None
                }
            })
            .collect();

        let auth = data
            .AuthValid
            .iter()
            .enumerate()
            .filter_map(|(idx, valid)| {
                if *valid != 0 {
                    Some(Auth {
                        data: data.Auth[idx],
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(Self {
            basic_id,
            location: (data.LocationValid != 0).then_some(Location {
                data: data.Location,
            }),
            auth,
            self_id: (data.SelfIDValid != 0).then_some(SelfId { data: data.SelfID }),
            system: (data.SystemValid != 0).then_some(System { data: data.System }),
            operator_id: (data.OperatorIDValid != 0).then_some(OperatorId {
                data: data.OperatorID,
            }),
        })
    }

    pub fn encode_to_vec(&self) -> Result<Vec<u8>, Error> {
        let mut message_pack_data = std::mem::MaybeUninit::<sys::ODID_MessagePack_data>::uninit();
        unsafe { sys::odid_initMessagePackData(message_pack_data.as_mut_ptr()) };
        let mut message_pack_data = unsafe { message_pack_data.assume_init() };

        let mut message_count = 0usize;

        let mut push_message = |message: Vec<u8>,
                                message_name: &'static str|
         -> Result<(), Error> {
            if message_count >= sys::ODID_PACK_MAX_MESSAGES as usize {
                return Err(Error::BufferTooSmall {
                    operation: "encode",
                    message: "MessagePack".into(),
                    remaining: (sys::ODID_PACK_MAX_MESSAGES as usize).saturating_sub(message_count),
                    required: message_count + 1,
                });
            }

            let required = sys::ODID_MESSAGE_SIZE as usize;
            if message.len() != required {
                return Err(Error::InvalidValue {
                    field: message_name,
                    value: format!("encoded length {} (expected {})", message.len(), required),
                });
            }

            let mut raw = [0u8; sys::ODID_MESSAGE_SIZE as usize];
            raw.copy_from_slice(&message);
            message_pack_data.Messages[message_count].rawData = raw;

            message_count += 1;
            Ok(())
        };

        for message in &self.basic_id {
            push_message(message.encode_to_vec()?, "basic_id")?;
        }

        if let Some(message) = &self.location {
            push_message(message.encode_to_vec()?, "location")?;
        }

        for message in &self.auth {
            push_message(message.encode_to_vec()?, "auth")?;
        }

        if let Some(message) = &self.self_id {
            push_message(message.encode_to_vec()?, "self_id")?;
        }

        if let Some(message) = &self.system {
            push_message(message.encode_to_vec()?, "system")?;
        }

        if let Some(message) = &self.operator_id {
            push_message(message.encode_to_vec()?, "operator_id")?;
        }

        if message_count == 0 {
            return Err(Error::InvalidValue {
                field: "MsgPackSize",
                value: "0".into(),
            });
        }

        message_pack_data.SingleMessageSize = sys::ODID_MESSAGE_SIZE as u8;
        message_pack_data.MsgPackSize = message_count as u8;

        let mut encoded_pack = std::mem::MaybeUninit::<sys::ODID_MessagePack_encoded>::uninit();
        if unsafe { sys::encodeMessagePack(encoded_pack.as_mut_ptr(), &message_pack_data) } as u32
            != sys::ODID_SUCCESS
        {
            return Err(Error::Unknown {
                operation: "encode",
                message: "MessagePack".into(),
            });
        }

        let encoded_pack = unsafe { encoded_pack.assume_init() };
        let encoded_len = 3 + (message_count * sys::ODID_MESSAGE_SIZE as usize);
        let bytes = unsafe {
            std::slice::from_raw_parts(
                (&encoded_pack as *const sys::ODID_MessagePack_encoded).cast::<u8>(),
                encoded_len,
            )
        };
        Ok(bytes.to_vec())
    }
}
