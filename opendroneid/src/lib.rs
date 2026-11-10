//! This crate defines Rust abstractions for the structs and enums exported by [`opendroneid_sys`]
//! from the C-library.
//!
//! * Message types are implemented as a wrapper around the `ODID_*_data` structs from [opendroneid_sys`]
//! * `const` values exported by [`opendroneid_sys`] are defined as enums or as constants in [`constants`]
//!
//! Each message type implements the [`Message`] trait that allows decoding and encoding of messages.
//! A special case is the [`UasData`] struct, this represents a group of messages which can be
//! together in a MessagePack message.

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
pub use uuid::Uuid;

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

/// Type of the [`BasicId::uas_id`]. Accessible through [`BasicId::id_type`] and [`UasId::id_type`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum IdType {
    #[default]
    None = sys::ODID_idtype_ODID_IDTYPE_NONE,
    SerialNumber = sys::ODID_idtype_ODID_IDTYPE_SERIAL_NUMBER,
    CaaRegistrationId = sys::ODID_idtype_ODID_IDTYPE_CAA_REGISTRATION_ID,
    UtmAssignedUuid = sys::ODID_idtype_ODID_IDTYPE_UTM_ASSIGNED_UUID,
    SpecificSessionId = sys::ODID_idtype_ODID_IDTYPE_SPECIFIC_SESSION_ID,
}

/// Type of the UA in the [`BasicId`] message. Accessible through [`BasicId::ua_type`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
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

/// Type of authentication used in the [`Auth`] message.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum DescriptionType {
    #[default]
    Text = sys::ODID_desctype_ODID_DESC_TYPE_TEXT,
    Emergency = sys::ODID_desctype_ODID_DESC_TYPE_EMERGENCY,
    ExtendedStatus = sys::ODID_desctype_ODID_DESC_TYPE_EXTENDED_STATUS,
}

/// Type of the Operator ID
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum OperatorIdType {
    #[default]
    OperatorId = sys::ODID_operatorIdType_ODID_OPERATOR_ID,
}

/// Type of the [`System::operator_location`]. Accessible through [`OperatorLocation::location_type`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum OperatorLocationType {
    #[default]
    Takeoff = sys::ODID_operator_location_type_ODID_OPERATOR_LOCATION_TYPE_TAKEOFF,
    LiveGnss = sys::ODID_operator_location_type_ODID_OPERATOR_LOCATION_TYPE_LIVE_GNSS,
    Fixed = sys::ODID_operator_location_type_ODID_OPERATOR_LOCATION_TYPE_FIXED,
}

/// Type of the [`System::classification`]. Accessible through [`Classification::classification_type`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum ClassificationType {
    #[default]
    Undeclared = sys::ODID_classification_type_ODID_CLASSIFICATION_TYPE_UNDECLARED,
    EuropeanUnion = sys::ODID_classification_type_ODID_CLASSIFICATION_TYPE_EU,
}

/// EU category of the UA in a [`Classification::EuropeanUnion`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum Category {
    #[default]
    Undeclared = sys::ODID_category_EU_ODID_CATEGORY_EU_UNDECLARED,
    Open = sys::ODID_category_EU_ODID_CATEGORY_EU_OPEN,
    Specific = sys::ODID_category_EU_ODID_CATEGORY_EU_SPECIFIC,
    Certified = sys::ODID_category_EU_ODID_CATEGORY_EU_CERTIFIED,
}

/// EU class of the UA in a [`Classification::EuropeanUnion`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
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

/// UAS ID of the [`BasicId`] message. The variant is the [`IdType`] of the ID.
///
/// The text ID types hold a [`String`], the binary ID types hold the bytes as broadcast.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UasId {
    /// ID type 0, none or not declared. The [`UAS_ID_SIZE`] bytes are kept as broadcast.
    None([u8; UAS_ID_SIZE]),
    /// ID type 1, manufacturer serial number in ANSI/CTA-2063-A format.
    SerialNumber(String),
    /// ID type 2, CAA issued registration ID, `<ICAO Nationality Mark>.<CAA Assigned ID>`.
    CaaRegistrationId(String),
    /// ID type 3, UTM assigned UUID, broadcast in the first 16 bytes.
    UtmAssignedUuid(Uuid),
    /// ID type 4, specific session ID.
    SpecificSessionId(SpecificSessionId),
}

impl UasId {
    /// Returns the [`IdType`] of the UAS ID.
    pub fn id_type(&self) -> IdType {
        match self {
            UasId::None(_) => IdType::None,
            UasId::SerialNumber(_) => IdType::SerialNumber,
            UasId::CaaRegistrationId(_) => IdType::CaaRegistrationId,
            UasId::UtmAssignedUuid(_) => IdType::UtmAssignedUuid,
            UasId::SpecificSessionId(_) => IdType::SpecificSessionId,
        }
    }

    /// Returns the [`UAS_ID_SIZE`] bytes of the UAS ID as they are broadcast.
    ///
    /// Returns an error if a text ID does not fit, see [`BasicId::with_uas_id`].
    pub fn to_bytes(&self) -> Result<[u8; UAS_ID_SIZE], Error> {
        let mut bytes = [0u8; UAS_ID_SIZE];
        match self {
            UasId::None(id) => bytes = *id,
            UasId::SerialNumber(id) | UasId::CaaRegistrationId(id) => {
                if id.len() > UAS_ID_SIZE {
                    return Err(Error::InvalidValue {
                        field: "UASID",
                        value: id.clone(),
                    });
                }
                bytes[..id.len()].copy_from_slice(id.as_bytes());
            }
            UasId::UtmAssignedUuid(uuid) => bytes[..UUID_SIZE].copy_from_slice(uuid.as_bytes()),
            UasId::SpecificSessionId(session_id) => {
                bytes[0] = session_id.id_type;
                bytes[1..].copy_from_slice(&session_id.id);
            }
        }
        Ok(bytes)
    }

    /// Decodes a UAS ID of the given [`IdType`] from the [`UAS_ID_SIZE`] bytes as broadcast.
    ///
    /// Text IDs end at the first `0x00` byte, bytes that are not valid UTF-8 are replaced.
    /// The bytes of the binary ID types are kept as they are.
    pub fn from_bytes(id_type: IdType, bytes: [u8; UAS_ID_SIZE]) -> Self {
        match id_type {
            IdType::None => UasId::None(bytes),
            IdType::SerialNumber => UasId::SerialNumber(utils::bytes_to_string(&bytes)),
            IdType::CaaRegistrationId => UasId::CaaRegistrationId(utils::bytes_to_string(&bytes)),
            IdType::UtmAssignedUuid => {
                let mut uuid = [0u8; UUID_SIZE];
                uuid.copy_from_slice(&bytes[..UUID_SIZE]);
                UasId::UtmAssignedUuid(Uuid::from_bytes(uuid))
            }
            IdType::SpecificSessionId => {
                let mut id = [0u8; SPECIFIC_SESSION_ID_SIZE];
                id.copy_from_slice(&bytes[1..]);
                UasId::SpecificSessionId(SpecificSessionId {
                    id_type: bytes[0],
                    id,
                })
            }
        }
    }
}

/// Specific session ID, a [`UasId`] that identifies one flight session.
///
/// It provides a greater level of privacy to the operator than the other ID types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpecificSessionId {
    /// Type of the session ID from the specific session ID type registry, the first broadcast byte.
    ///
    /// `1` is an IETF DRIP entity ID, `2` is an IEEE 1609.2-2016 HashedID8. `0` is reserved.
    pub id_type: u8,
    /// Session ID in accordance with the specification of its type, the remaining broadcast bytes.
    pub id: [u8; SPECIFIC_SESSION_ID_SIZE],
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

/// Location of the operator in the [`System`] message. The variant is the [`OperatorLocationType`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperatorLocation {
    /// The operator is at the takeoff location.
    Takeoff(OperatorPosition),
    /// Dynamic location of the operator from a live GNSS.
    LiveGnss(OperatorPosition),
    /// Fixed location of the operator.
    Fixed(OperatorPosition),
}

impl OperatorLocation {
    /// Returns the [`OperatorLocationType`] of the location.
    pub fn location_type(&self) -> OperatorLocationType {
        match self {
            OperatorLocation::Takeoff(_) => OperatorLocationType::Takeoff,
            OperatorLocation::LiveGnss(_) => OperatorLocationType::LiveGnss,
            OperatorLocation::Fixed(_) => OperatorLocationType::Fixed,
        }
    }

    /// Returns the position of the operator.
    pub fn position(&self) -> &OperatorPosition {
        match self {
            OperatorLocation::Takeoff(position)
            | OperatorLocation::LiveGnss(position)
            | OperatorLocation::Fixed(position) => position,
        }
    }
}

/// Position of the operator in an [`OperatorLocation`].
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct OperatorPosition {
    /// Coordinates of the operator, `None` if they are unknown.
    pub coordinates: Option<Coordinates>,
    /// Geodetic altitude above the WGS-84 ellipsoid in meters, `None` if it is unknown.
    pub altitude_geodetic: Option<f32>,
}

/// WGS-84 coordinates in degrees, in a [`Location`] or an [`OperatorPosition`].
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
    fn known(latitude: f64, longitude: f64) -> Option<Self> {
        (latitude != 0.0 || longitude != 0.0).then_some(Coordinates {
            latitude,
            longitude,
        })
    }

    /// Checks the coordinates and returns the latitude and longitude to store, 0°/0° for `None`.
    fn checked(coordinates: Option<Self>) -> Result<(f64, f64), Error> {
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

/// Returns `value`, or `None` if it is the `unknown` marker of its field.
fn known<T: PartialEq>(value: T, unknown: T) -> Option<T> {
    (value != unknown).then_some(value)
}

/// Checks that `value` is in `range` and returns the value to store, `unknown` for `None`.
fn checked<T: PartialOrd + ToString>(
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
fn checked_altitude(field: &'static str, altitude: Option<f32>) -> Result<f32, Error> {
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

/// Classification of the UA in the [`System`] message. The variant is the [`ClassificationType`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Classification {
    /// The classification is not declared.
    #[default]
    Undeclared,
    /// Classification of the European Union.
    EuropeanUnion {
        /// EU category of the UA.
        category: Category,
        /// EU class of the UA.
        class: ClassEu,
    },
}

impl Classification {
    /// Returns the [`ClassificationType`] of the classification.
    pub fn classification_type(&self) -> ClassificationType {
        match self {
            Classification::Undeclared => ClassificationType::Undeclared,
            Classification::EuropeanUnion { .. } => ClassificationType::EuropeanUnion,
        }
    }
}

/// Page of the authentication data in an [`Auth`] message. The variant is the page number.
///
/// The first page holds the fields that describe the whole authentication data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthPage {
    /// Page 0.
    First {
        /// Index of the last page, below [`AUTH_MAX_PAGES`].
        last_page_index: u8,
        /// Length in bytes of the authentication data across all pages.
        length: u8,
        /// Seconds since 2019-01-01 00:00:00 UTC.
        timestamp: u32,
        /// First bytes of the authentication data.
        data: [u8; AUTH_FIRST_PAGE_DATA_SIZE],
    },
    /// Pages 1 to `AUTH_MAX_PAGES - 1`.
    Next {
        /// Page number.
        index: u8,
        /// Following bytes of the authentication data.
        data: [u8; AUTH_NEXT_PAGE_DATA_SIZE],
    },
}

impl AuthPage {
    /// Returns the page number.
    pub fn index(&self) -> u8 {
        match self {
            AuthPage::First { .. } => 0,
            AuthPage::Next { index, .. } => *index,
        }
    }

    /// Returns the bytes of the authentication data on this page.
    pub fn data(&self) -> &[u8] {
        match self {
            AuthPage::First { data, .. } => data,
            AuthPage::Next { data, .. } => data,
        }
    }

    /// Returns how many bytes of authentication data fit in pages 0 to `last_page_index`.
    fn capacity(last_page_index: u8) -> usize {
        AUTH_FIRST_PAGE_DATA_SIZE + last_page_index as usize * AUTH_NEXT_PAGE_DATA_SIZE
    }
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
    ///     .with_uas_id(opendroneid::UasId::SerialNumber("TEST1234567890".into()))
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
    /// Returns the UA type of the message, or an [`Error`] if the value is invalid.
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
    /// Returns the ID type of the message, or an [`Error`] if the value is invalid.
    pub fn id_type(&self) -> Result<IdType, Error> {
        IdType::from_u32(self.data.IDType).ok_or(Error::EnumMappingError {
            field: "IDType",
            value: self.data.IDType,
        })
    }
    /// Returns the UAS ID of the message, or an [`Error`] if the ID type is invalid.
    ///
    /// The variant of the [`UasId`] is the ID type of the message.
    pub fn uas_id(&self) -> Result<UasId, Error> {
        let mut bytes = [0u8; UAS_ID_SIZE];
        for (dest, &src) in bytes.iter_mut().zip(&self.data.UASID) {
            *dest = src as u8;
        }
        Ok(UasId::from_bytes(self.id_type()?, bytes))
    }
    /// Sets the UAS ID and the ID type of the message.
    ///
    /// Returns an error if a [`UasId::SerialNumber`] or [`UasId::CaaRegistrationId`] is longer
    /// than [`UAS_ID_SIZE`] bytes in UTF-8 encoding.
    pub fn with_uas_id(mut self, uas_id: UasId) -> Result<BasicId, Error> {
        let bytes = uas_id.to_bytes()?;
        self.data.UASID.fill(0);
        for (dest, &src) in self.data.UASID.iter_mut().zip(&bytes) {
            *dest = src as core::ffi::c_char;
        }
        self.data.IDType = uas_id.id_type() as u32;
        Ok(self)
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
        known(self.data.Direction, sys::INV_DIR as f32)
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
        known(self.data.SpeedHorizontal, sys::INV_SPEED_H as f32)
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
        known(self.data.SpeedVertical, sys::INV_SPEED_V as f32)
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
        known(self.data.AltitudeBaro, sys::INV_ALT as f32)
    }
    /// Sets the barometric altitude of the UA in meters, `None` if it is unknown.
    /// Returns an error if the altitude is out of the valid range defined by the protocol, or for
    /// [`MIN_ALTITUDE`], which marks an unknown altitude.
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
        known(self.data.AltitudeGeo, sys::INV_ALT as f32)
    }
    /// Sets the geodetic altitude of the UA in meters, `None` if it is unknown.
    /// Returns an error if the altitude is out of the valid range defined by the protocol, or for
    /// [`MIN_ALTITUDE`], which marks an unknown altitude.
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
            known(self.data.Height, sys::INV_ALT as f32).map(|height| match reference {
                HeightReference::Takeoff => Height::AboveTakeoff(height),
                HeightReference::Ground => Height::AboveGroundLevel(height),
            }),
        )
    }
    /// Sets the height of the UA and its reference, `None` if it is unknown.
    /// Returns an error if the height is out of the valid range defined by the protocol, or for
    /// [`MIN_ALTITUDE`], which marks an unknown height.
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
        known(self.data.TimeStamp, sys::INV_TIMESTAMP as f32)
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

    /// Sets the timestamp to the seconds of `time` since the start of its hour.
    #[cfg(feature = "chrono")]
    pub fn with_chrono_timestamp(mut self, time: chrono::DateTime<chrono::Utc>) -> Self {
        self.data.TimeStamp = time.timestamp_millis().rem_euclid(HOUR_MS) as f32 / 1000.0;
        self
    }
}

/// Number of milliseconds in an hour, the range of the [`Location`] timestamp.
#[cfg(feature = "chrono")]
const HOUR_MS: i64 = 3_600_000;

/// Number of seconds from 1970-01-01 to 2019-01-01, the epoch of the Auth and System timestamps.
#[cfg(feature = "chrono")]
const ODID_EPOCH_OFFSET: i64 = 1546300800;

/// Converts seconds since 2019-01-01 00:00:00 UTC, the timestamp of an [`AuthPage::First`] and of
/// a [`System`] message, to a `chrono::DateTime<Utc>`.
#[cfg(feature = "chrono")]
pub fn from_odid_epoch(seconds: u32) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::from_timestamp(seconds as i64 + ODID_EPOCH_OFFSET, 0)
        .expect("every u32 offset from 2019 is a valid date")
}

/// Converts a `chrono::DateTime<Utc>` to whole seconds since 2019-01-01 00:00:00 UTC, the timestamp
/// of an [`AuthPage::First`] and of a [`System`] message.
///
/// Returns an error for a time before 2019 or after the range of a `u32`.
#[cfg(feature = "chrono")]
pub fn to_odid_epoch(time: chrono::DateTime<chrono::Utc>) -> Result<u32, Error> {
    u32::try_from(time.timestamp() - ODID_EPOCH_OFFSET).map_err(|_| Error::InvalidValue {
        field: "timestamp",
        value: time.to_rfc3339(),
    })
}

macros::impl_message!(
    /// The Authentication Message defines a field that provides a means for authenticating the
    /// identity of the UA sending the message.
    ///
    /// The authentication data is split across up to [`AUTH_MAX_PAGES`] messages, one per
    /// [`AuthPage`].
    ///
    /// ```rust
    /// use opendroneid::{Auth, AuthPage, AUTH_FIRST_PAGE_DATA_SIZE};
    ///
    /// // Create the first page of an Auth message.
    /// let auth = Auth::default()
    ///     // Set fields of the Auth message using the builder pattern.
    ///     .with_auth_type(opendroneid::AuthenticationType::UasIdSignature)
    ///     .with_page(AuthPage::First {
    ///         last_page_index: 0,
    ///         length: 4,
    ///         timestamp: 12345,
    ///         data: [0xde, 0xad, 0xbe, 0xef, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ///     })
    ///     .expect("Invalid Auth page");
    ///
    /// assert_eq!(auth.page().data().len(), AUTH_FIRST_PAGE_DATA_SIZE);
    /// ```
    Auth,
    sys::ODID_Auth_data,
    sys::ODID_Auth_encoded,
    sys::odid_initAuthData,
    sys::encodeAuthMessage,
    sys::decodeAuthMessage
);

impl Auth {
    /// Returns the authentication type of the message, or an [`Error`] if the value is invalid.
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

    /// Returns the page of the authentication data in the message.
    ///
    /// The variant of the [`AuthPage`] is the page number of the message.
    pub fn page(&self) -> AuthPage {
        if self.data.DataPage == 0 {
            let mut data = [0u8; AUTH_FIRST_PAGE_DATA_SIZE];
            data.copy_from_slice(&self.data.AuthData[..AUTH_FIRST_PAGE_DATA_SIZE]);
            AuthPage::First {
                last_page_index: self.data.LastPageIndex,
                length: self.data.Length,
                timestamp: self.data.Timestamp,
                data,
            }
        } else {
            let mut data = [0u8; AUTH_NEXT_PAGE_DATA_SIZE];
            data.copy_from_slice(&self.data.AuthData[..AUTH_NEXT_PAGE_DATA_SIZE]);
            AuthPage::Next {
                index: self.data.DataPage,
                data,
            }
        }
    }

    /// Sets the page of the authentication data and its page number.
    ///
    /// Returns an error if a page index is out of range, or if the `length` of the first page
    /// exceeds the data that fits in the pages up to its `last_page_index`.
    pub fn with_page(mut self, page: AuthPage) -> Result<Self, Error> {
        self.data.AuthData.fill(0);
        match page {
            AuthPage::First {
                last_page_index,
                length,
                timestamp,
                data,
            } => {
                if last_page_index as usize >= AUTH_MAX_PAGES {
                    return Err(Error::InvalidValue {
                        field: "last_page_index",
                        value: last_page_index.to_string(),
                    });
                }
                if length as usize > AuthPage::capacity(last_page_index) {
                    return Err(Error::InvalidValue {
                        field: "length",
                        value: length.to_string(),
                    });
                }
                self.data.DataPage = 0;
                self.data.LastPageIndex = last_page_index;
                self.data.Length = length;
                self.data.Timestamp = timestamp;
                self.data.AuthData[..AUTH_FIRST_PAGE_DATA_SIZE].copy_from_slice(&data);
            }
            AuthPage::Next { index, data } => {
                if index == 0 || index as usize >= AUTH_MAX_PAGES {
                    return Err(Error::InvalidValue {
                        field: "index",
                        value: index.to_string(),
                    });
                }
                // Only the first page carries these fields.
                self.data.DataPage = index;
                self.data.LastPageIndex = 0;
                self.data.Length = 0;
                self.data.Timestamp = 0;
                self.data.AuthData[..AUTH_NEXT_PAGE_DATA_SIZE].copy_from_slice(&data);
            }
        }
        Ok(self)
    }

    /// Returns the timestamp of the first page as a `chrono::DateTime<Utc>`, or `None` for a
    /// later page.
    #[cfg(feature = "chrono")]
    pub fn chrono_timestamp(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        match self.page() {
            AuthPage::First { timestamp, .. } => Some(from_odid_epoch(timestamp)),
            AuthPage::Next { .. } => None,
        }
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
    /// Returns the description type of the message, or an [`Error`] if the value is invalid.
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
    /// Returns the location of the operator, or an [`Error`] if the location type is invalid.
    ///
    /// The variant of the [`OperatorLocation`] is the operator location type of the message.
    pub fn operator_location(&self) -> Result<OperatorLocation, Error> {
        let location_type = OperatorLocationType::from_u32(self.data.OperatorLocationType).ok_or(
            Error::EnumMappingError {
                field: "OperatorLocationType",
                value: self.data.OperatorLocationType,
            },
        )?;
        let position = OperatorPosition {
            coordinates: Coordinates::known(
                self.data.OperatorLatitude,
                self.data.OperatorLongitude,
            ),
            altitude_geodetic: known(self.data.OperatorAltitudeGeo, sys::INV_ALT as f32),
        };
        Ok(match location_type {
            OperatorLocationType::Takeoff => OperatorLocation::Takeoff(position),
            OperatorLocationType::LiveGnss => OperatorLocation::LiveGnss(position),
            OperatorLocationType::Fixed => OperatorLocation::Fixed(position),
        })
    }

    /// Sets the location of the operator and its type.
    /// Returns an error if a coordinate or the altitude is out of the valid range defined by the
    /// protocol, or for 0°/0° or [`MIN_ALTITUDE`], which mark unknown values.
    pub fn with_operator_location(mut self, location: OperatorLocation) -> Result<Self, Error> {
        let position = location.position();
        (self.data.OperatorLatitude, self.data.OperatorLongitude) =
            Coordinates::checked(position.coordinates)?;
        self.data.OperatorAltitudeGeo =
            checked_altitude("operator_altitude_geodetic", position.altitude_geodetic)?;
        self.data.OperatorLocationType = location.location_type() as u32;
        Ok(self)
    }

    /// Returns the classification of the UA, or an [`Error`] if the classification type, the
    /// category or the class is invalid.
    ///
    /// The variant of the [`Classification`] is the classification type of the message.
    pub fn classification(&self) -> Result<Classification, Error> {
        let classification_type = ClassificationType::from_u32(self.data.ClassificationType)
            .ok_or(Error::EnumMappingError {
                field: "ClassificationType",
                value: self.data.ClassificationType,
            })?;
        Ok(match classification_type {
            ClassificationType::Undeclared => Classification::Undeclared,
            ClassificationType::EuropeanUnion => Classification::EuropeanUnion {
                category: Category::from_u32(self.data.CategoryEU).ok_or(
                    Error::EnumMappingError {
                        field: "CategoryEU",
                        value: self.data.CategoryEU,
                    },
                )?,
                class: ClassEu::from_u32(self.data.ClassEU).ok_or(Error::EnumMappingError {
                    field: "ClassEU",
                    value: self.data.ClassEU,
                })?,
            },
        })
    }

    /// Sets the classification of the UA and its type.
    pub fn with_classification(mut self, classification: Classification) -> Self {
        self.data.ClassificationType = classification.classification_type() as u32;
        let (category, class) = match classification {
            Classification::Undeclared => (Category::Undeclared, ClassEu::Undeclared),
            Classification::EuropeanUnion { category, class } => (category, class),
        };
        self.data.CategoryEU = category as u32;
        self.data.ClassEU = class as u32;
        self
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

    /// Returns the geodetic altitude of the area ceiling in meters, or `None` if it is unknown.
    pub fn area_ceiling(&self) -> Option<f32> {
        known(self.data.AreaCeiling, sys::INV_ALT as f32)
    }

    /// Sets the geodetic altitude of the area ceiling in meters, `None` if it is unknown.
    /// Returns an error if the altitude is out of the valid range defined by the protocol, or for
    /// [`MIN_ALTITUDE`], which marks an unknown altitude.
    pub fn with_area_ceiling(mut self, altitude: impl Into<Option<f32>>) -> Result<Self, Error> {
        self.data.AreaCeiling = checked_altitude("area_ceiling", altitude.into())?;
        Ok(self)
    }

    /// Returns the geodetic altitude of the area floor in meters, or `None` if it is unknown.
    pub fn area_floor(&self) -> Option<f32> {
        known(self.data.AreaFloor, sys::INV_ALT as f32)
    }

    /// Sets the geodetic altitude of the area floor in meters, `None` if it is unknown.
    /// Returns an error if the altitude is out of the valid range defined by the protocol, or for
    /// [`MIN_ALTITUDE`], which marks an unknown altitude.
    pub fn with_area_floor(mut self, altitude: impl Into<Option<f32>>) -> Result<Self, Error> {
        self.data.AreaFloor = checked_altitude("area_floor", altitude.into())?;
        Ok(self)
    }

    /// Returns the timestamp in seconds since 2019-01-01 00:00:00 UTC.
    ///
    /// Convert it with `from_odid_epoch` (feature `chrono`).
    pub fn timestamp(&self) -> u32 {
        self.data.Timestamp
    }

    /// Sets the timestamp in seconds since 2019-01-01 00:00:00 UTC.
    ///
    /// Convert a time with `to_odid_epoch` (feature `chrono`).
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
    /// Returns the operator ID type of the message, or an [`Error`] if the value is invalid.
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

    /// Decode UAS data from a single Open Drone ID message or a message pack in a buffer.
    ///
    /// Consumes the bytes of the message from the buffer, which does not need to be contiguous.
    /// Returns an error if the message type is invalid or the buffer holds fewer bytes than the
    /// message. On an error, part of the buffer may have been consumed.
    pub fn decode(mut buf: impl Buf) -> Result<Self, Error> {
        const MESSAGE_SIZE: usize = sys::ODID_MESSAGE_SIZE as usize;
        /// Size of the message pack header: message type, single message size and pack size.
        const PACK_HEADER_SIZE: usize = 3;

        // Peek the type before consuming anything; the first chunk is not empty if bytes remain.
        let message_id = match buf.chunk().first() {
            Some(&byte) => MessageId::try_from(byte)?,
            None => return Err(Self::buffer_too_small(0, MESSAGE_SIZE)),
        };
        // Every message, including the smallest message pack, is at least one message long.
        if buf.remaining() < MESSAGE_SIZE {
            return Err(Self::buffer_too_small(buf.remaining(), MESSAGE_SIZE));
        }

        // Copy the message into a zeroed buffer of the maximum message size instead of passing a
        // pointer into `buf` to the C decoder:
        // * `decodeOpenDroneID` takes no length. It trusts the message type and pack size in the
        //   message and reads as many bytes as they declare, up to the size of this buffer.
        // * The length checks above follow how the C decoder reads today. The full-size buffer
        //   keeps every read in bounds even if a change in the C library reads more than that.
        // * A `Buf` does not need to be contiguous, so `buf.chunk()` may not hold the whole
        //   message. The copy also handles that.
        // The copy is at most 228 bytes, which is small next to zeroing and filling the 920 byte
        // `ODID_UAS_Data` below.
        let mut encoded = [0u8; std::mem::size_of::<sys::ODID_MessagePack_encoded>()];
        buf.copy_to_slice(&mut encoded[..MESSAGE_SIZE]);

        if message_id == MessageId::MessagePack {
            let pack_size = encoded[2] as usize;
            if pack_size == 0 || pack_size > sys::ODID_PACK_MAX_MESSAGES as usize {
                return Err(Error::InvalidValue {
                    field: "MsgPackSize",
                    value: pack_size.to_string(),
                });
            }
            let pack_len = PACK_HEADER_SIZE + pack_size * MESSAGE_SIZE;
            if MESSAGE_SIZE + buf.remaining() < pack_len {
                return Err(Self::buffer_too_small(
                    MESSAGE_SIZE + buf.remaining(),
                    pack_len,
                ));
            }
            buf.copy_to_slice(&mut encoded[MESSAGE_SIZE..pack_len]);
        }

        let mut data = sys::ODID_UAS_Data::default();
        let r = unsafe {
            sys::decodeOpenDroneID(&mut data as *mut sys::ODID_UAS_Data, encoded.as_ptr())
        };

        if r == sys::ODID_messagetype_ODID_MESSAGETYPE_INVALID {
            return Err(Error::Unknown {
                operation: "decode",
                message: format!("{message_id:?}").into(),
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

    fn buffer_too_small(remaining: usize, required: usize) -> Error {
        Error::BufferTooSmall {
            operation: "decode",
            message: "UasData".into(),
            remaining,
            required,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    /// A 16-byte UUID that contains `0x00` (also as first byte) and bytes `>= 0x80`.
    const UUID: [u8; UUID_SIZE] = [
        0x00, 0x9f, 0x3c, 0x80, 0xff, 0x00, 0x4a, 0x11, 0x81, 0x00, 0xc3, 0x28, 0x7e, 0xfe, 0x00,
        0x01,
    ];

    fn round_trip<M: Message>(message: &M) -> M {
        let encoded = message.encode_to_vec().expect("encoding should succeed");
        M::decode(encoded.as_slice()).expect("decoding should succeed")
    }

    fn basic_id(uas_id: UasId) -> BasicId {
        BasicId::default()
            .with_ua_type(UaType::HelicopterOrMultirotor)
            .with_uas_id(uas_id)
            .expect("UAS ID should fit")
    }

    #[test]
    fn utm_assigned_uuid_keeps_binary_bytes() {
        let uas_id = UasId::UtmAssignedUuid(Uuid::from_bytes(UUID));
        let message = basic_id(uas_id.clone());

        assert_eq!(message.id_type().unwrap(), IdType::UtmAssignedUuid);
        assert_eq!(message.uas_id().unwrap(), uas_id);
        assert_eq!(round_trip(&message).uas_id().unwrap(), uas_id);
    }

    #[test]
    fn specific_session_id_keeps_type_and_binary_bytes() {
        let mut id = [0u8; SPECIFIC_SESSION_ID_SIZE];
        for (i, byte) in id.iter_mut().enumerate() {
            *byte = UUID[i % UUID_SIZE] ^ (i as u8);
        }
        id[0] = 0x00;
        let uas_id = UasId::SpecificSessionId(SpecificSessionId { id_type: 1, id });

        let decoded = round_trip(&basic_id(uas_id.clone()));

        assert_eq!(decoded.id_type().unwrap(), IdType::SpecificSessionId);
        assert_eq!(decoded.uas_id().unwrap(), uas_id);
    }

    #[test]
    fn not_declared_id_keeps_bytes() {
        let mut bytes = [0u8; UAS_ID_SIZE];
        bytes[..UUID_SIZE].copy_from_slice(&UUID);
        let uas_id = UasId::None(bytes);

        assert_eq!(
            round_trip(&basic_id(uas_id.clone())).uas_id().unwrap(),
            uas_id
        );
    }

    #[test]
    fn text_ids_round_trip() {
        for uas_id in [
            UasId::SerialNumber("1596F123456789ABCDEF".into()),
            UasId::CaaRegistrationId("FIN.87astrdge12k8".into()),
        ] {
            assert_eq!(
                round_trip(&basic_id(uas_id.clone())).uas_id().unwrap(),
                uas_id
            );
        }
    }

    #[test]
    fn text_id_longer_than_field_is_rejected() {
        let result =
            BasicId::default().with_uas_id(UasId::SerialNumber("1596F123456789ABCDEFG".into()));

        assert!(matches!(
            result,
            Err(Error::InvalidValue { field: "UASID", .. })
        ));
    }

    #[test]
    fn with_uas_id_overwrites_previous_id() {
        let message = basic_id(UasId::SerialNumber("ABCDEFGHIJKLMNOPQRST".into()))
            .with_uas_id(UasId::CaaRegistrationId("FIN.1".into()))
            .unwrap();

        assert_eq!(
            message.uas_id().unwrap(),
            UasId::CaaRegistrationId("FIN.1".into())
        );
    }

    #[test]
    fn invalid_id_type_is_an_error() {
        let mut message = BasicId::default();
        message.data.IDType = 42;

        assert!(matches!(
            message.uas_id(),
            Err(Error::EnumMappingError {
                field: "IDType",
                ..
            })
        ));
    }

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

    const MUNICH: Coordinates = Coordinates {
        latitude: 48.137_154_3,
        longitude: 11.575_382_1,
    };

    fn assert_coordinates_eq(actual: Option<Coordinates>, expected: Option<Coordinates>) {
        match (actual, expected) {
            (Some(actual), Some(expected)) => {
                assert!((actual.latitude - expected.latitude).abs() < 1e-7);
                assert!((actual.longitude - expected.longitude).abs() < 1e-7);
            }
            (actual, expected) => assert_eq!(actual, expected),
        }
    }

    #[test]
    fn operator_location_round_trips_with_type() {
        let position = OperatorPosition {
            coordinates: Some(MUNICH),
            altitude_geodetic: Some(520.5),
        };
        for location in [
            OperatorLocation::Takeoff(position),
            OperatorLocation::LiveGnss(position),
            OperatorLocation::Fixed(OperatorPosition::default()),
        ] {
            let decoded = round_trip(&System::default().with_operator_location(location).unwrap())
                .operator_location()
                .unwrap();

            assert_eq!(decoded.location_type(), location.location_type());
            assert_coordinates_eq(
                decoded.position().coordinates,
                location.position().coordinates,
            );
            assert_eq!(
                decoded.position().altitude_geodetic,
                location.position().altitude_geodetic
            );
        }
    }

    #[test]
    fn operator_location_out_of_range_is_rejected() {
        let result =
            System::default().with_operator_location(OperatorLocation::Fixed(OperatorPosition {
                coordinates: Some(Coordinates {
                    latitude: 91.0,
                    longitude: 0.0,
                }),
                altitude_geodetic: None,
            }));

        assert!(matches!(
            result,
            Err(Error::InvalidValue {
                field: "latitude",
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

    #[test]
    fn area_ceiling_and_floor_round_trip() {
        let system = System::default()
            .with_area_ceiling(150.0)
            .unwrap()
            .with_area_floor(None)
            .unwrap();

        let decoded = round_trip(&system);

        assert_eq!(decoded.area_ceiling(), Some(150.0));
        assert_eq!(decoded.area_floor(), None);
        assert_eq!(System::default().area_ceiling(), None);
    }

    fn first_page(last_page_index: u8, length: u8) -> AuthPage {
        let mut data = [0u8; AUTH_FIRST_PAGE_DATA_SIZE];
        data[0] = 0x00;
        data[1] = 0xff;
        data[AUTH_FIRST_PAGE_DATA_SIZE - 1] = 0x80;
        AuthPage::First {
            last_page_index,
            length,
            timestamp: 12345,
            data,
        }
    }

    #[test]
    fn auth_pages_round_trip() {
        let mut data = [0xa5u8; AUTH_NEXT_PAGE_DATA_SIZE];
        data[0] = 0x00;
        for page in [
            first_page(2, 63),
            AuthPage::Next { index: 1, data },
            AuthPage::Next {
                index: AUTH_MAX_PAGES as u8 - 1,
                data,
            },
        ] {
            let auth = Auth::default()
                .with_auth_type(AuthenticationType::MessageSetSignature)
                .with_page(page)
                .unwrap();

            let decoded = round_trip(&auth);

            assert_eq!(
                decoded.auth_type().unwrap(),
                AuthenticationType::MessageSetSignature
            );
            assert_eq!(decoded.page(), page);
            assert_eq!(decoded.page().index(), page.index());
        }
    }

    #[test]
    fn auth_length_must_fit_in_the_pages() {
        let capacity = (AUTH_FIRST_PAGE_DATA_SIZE + AUTH_NEXT_PAGE_DATA_SIZE) as u8;

        assert!(Auth::default().with_page(first_page(1, capacity)).is_ok());
        assert!(matches!(
            Auth::default().with_page(first_page(1, capacity + 1)),
            Err(Error::InvalidValue {
                field: "length",
                ..
            })
        ));
    }

    #[test]
    fn auth_page_indices_are_checked() {
        let data = [0u8; AUTH_NEXT_PAGE_DATA_SIZE];
        for index in [0, AUTH_MAX_PAGES as u8] {
            assert!(matches!(
                Auth::default().with_page(AuthPage::Next { index, data }),
                Err(Error::InvalidValue { field: "index", .. })
            ));
        }
        assert!(matches!(
            Auth::default().with_page(first_page(AUTH_MAX_PAGES as u8, 0)),
            Err(Error::InvalidValue {
                field: "last_page_index",
                ..
            })
        ));
    }

    #[test]
    fn later_auth_page_clears_first_page_fields() {
        let data = [0x11u8; AUTH_NEXT_PAGE_DATA_SIZE];
        let auth = Auth::default()
            .with_page(first_page(3, 40))
            .unwrap()
            .with_page(AuthPage::Next { index: 2, data })
            .unwrap();

        let expected = Auth::default()
            .with_page(AuthPage::Next { index: 2, data })
            .unwrap();
        assert_eq!(auth, expected);
    }

    #[cfg(feature = "chrono")]
    fn utc(time: &str) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::parse_from_rfc3339(time).unwrap().to_utc()
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
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn odid_epoch_conversions() {
        let time = utc("2026-11-10T15:00:00Z");
        let seconds = to_odid_epoch(time).unwrap();

        assert_eq!(from_odid_epoch(seconds), time);
        assert_eq!(from_odid_epoch(0).to_rfc3339(), "2019-01-01T00:00:00+00:00");
        assert!(to_odid_epoch(utc("2018-12-31T23:59:59Z")).is_err());
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn auth_chrono_timestamp_is_only_on_the_first_page() {
        let first = Auth::default().with_page(first_page(0, 1)).unwrap();
        let next = Auth::default()
            .with_page(AuthPage::Next {
                index: 1,
                data: [0; AUTH_NEXT_PAGE_DATA_SIZE],
            })
            .unwrap();

        assert_eq!(first.chrono_timestamp(), Some(from_odid_epoch(12345)));
        assert_eq!(next.chrono_timestamp(), None);
    }

    #[test]
    fn classification_round_trips() {
        for classification in [
            Classification::Undeclared,
            Classification::EuropeanUnion {
                category: Category::Specific,
                class: ClassEu::Class2,
            },
        ] {
            let message = System::default().with_classification(classification);

            assert_eq!(
                round_trip(&message).classification().unwrap(),
                classification
            );
        }
    }

    const MESSAGE_SIZE: usize = sys::ODID_MESSAGE_SIZE as usize;

    fn uas_data() -> UasData {
        UasData::default()
            .with_basic_id(vec![basic_id(UasId::UtmAssignedUuid(Uuid::from_bytes(
                UUID,
            )))])
            .with_location(Some(
                Location::default()
                    .with_height(Height::AboveTakeoff(42.0))
                    .unwrap(),
            ))
    }

    fn assert_buffer_too_small(result: Result<UasData, Error>, remaining: usize, required: usize) {
        match result {
            Err(Error::BufferTooSmall {
                remaining: r,
                required: q,
                ..
            }) => assert_eq!((r, q), (remaining, required)),
            other => panic!("expected BufferTooSmall, got {other:?}"),
        }
    }

    #[test]
    fn uas_data_decode_rejects_empty_buffer() {
        assert_buffer_too_small(UasData::decode(&[][..]), 0, MESSAGE_SIZE);
    }

    #[test]
    fn uas_data_decode_rejects_truncated_message() {
        let encoded = basic_id(UasId::SerialNumber("1596F1".into()))
            .encode_to_vec()
            .unwrap();

        for len in [1, MESSAGE_SIZE - 1] {
            assert_buffer_too_small(UasData::decode(&encoded[..len]), len, MESSAGE_SIZE);
        }
    }

    #[test]
    fn uas_data_decode_rejects_invalid_message_type() {
        let encoded = [0x60u8; MESSAGE_SIZE];

        assert!(matches!(
            UasData::decode(&encoded[..]),
            Err(Error::EnumMappingError {
                field: "MessageId",
                value: 0x60
            })
        ));
    }

    #[test]
    fn uas_data_decode_rejects_truncated_message_pack() {
        let encoded = uas_data().encode_to_vec().unwrap();
        let pack_len = encoded.len();

        assert_buffer_too_small(
            UasData::decode(&encoded[..pack_len - 1]),
            pack_len - 1,
            pack_len,
        );
    }

    #[test]
    fn uas_data_decode_rejects_invalid_message_pack_size() {
        let mut encoded = [0u8; 3 + 10 * MESSAGE_SIZE];
        encoded[0] = 0xf2;
        encoded[1] = MESSAGE_SIZE as u8;
        for pack_size in [0, sys::ODID_PACK_MAX_MESSAGES as u8 + 1] {
            encoded[2] = pack_size;

            assert!(matches!(
                UasData::decode(&encoded[..]),
                Err(Error::InvalidValue {
                    field: "MsgPackSize",
                    ..
                })
            ));
        }
    }

    #[test]
    fn uas_data_decode_message_pack_consumes_only_the_pack() {
        let data = uas_data();
        let mut encoded = data.encode_to_vec().unwrap();
        let pack_len = encoded.len();
        encoded.extend_from_slice(&[0xaa; 7]);

        let mut buf = &encoded[..];
        let decoded = UasData::decode(&mut buf).unwrap();

        assert_eq!(decoded, data);
        assert_eq!(buf, &encoded[pack_len..]);
    }

    #[test]
    fn uas_data_decode_single_message() {
        let message = basic_id(UasId::SerialNumber("1596F1".into()));
        let mut encoded = message.encode_to_vec().unwrap();
        encoded.push(0xaa);

        let mut buf = &encoded[..];
        let decoded = UasData::decode(&mut buf).unwrap();

        assert_eq!(decoded.basic_id(), [message]);
        assert_eq!(buf, [0xaa]);
    }

    #[test]
    fn uas_data_decode_non_contiguous_buffer() {
        let data = uas_data();
        let encoded = data.encode_to_vec().unwrap();
        // Split inside the pack header so that the pack size is not in the first chunk.
        let buf = (&encoded[..2]).chain(&encoded[2..]);

        assert_eq!(UasData::decode(buf).unwrap(), data);
    }

    #[test]
    fn undeclared_classification_clears_category_and_class() {
        let message = System::default()
            .with_classification(Classification::EuropeanUnion {
                category: Category::Open,
                class: ClassEu::Class1,
            })
            .with_classification(Classification::Undeclared);

        assert_eq!(message.data.CategoryEU, Category::Undeclared as u32);
        assert_eq!(message.data.ClassEU, ClassEu::Undeclared as u32);
    }
}
