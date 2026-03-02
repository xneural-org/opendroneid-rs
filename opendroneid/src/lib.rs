use bytes::{Buf, BufMut};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use opendroneid_sys::{self as sys};

mod macros;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EncodeError {
    #[error("Can't encode {0} to wire format")]
    Unknown(String),
    #[error("Encode buffer too small for {message}: remaining {remaining}, required {required}")]
    BufferTooSmall {
        message: String,
        remaining: usize,
        required: usize,
    },
    #[error("Invalid value for {0}: {1}")]
    InvalidValue(&'static str, String),
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DecodeError {
    #[error("Can't decode {0} from wire format")]
    Unknown(String),
    #[error(
        "Decode buffer too small for {message}: remaining {remaining}, expected at least {expected}"
    )]
    BufferTooSmall {
        message: String,
        remaining: usize,
        expected: usize,
    },
    #[error("Enum mapping error: {0} has invalid value {1}")]
    EnumMappingError(&'static str, u32),
    #[error("Invalid value for {0}: {1}")]
    InvalidValue(&'static str, String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum MessageType {
    BasicId = sys::ODID_messagetype_ODID_MESSAGETYPE_BASIC_ID,
    Location = sys::ODID_messagetype_ODID_MESSAGETYPE_LOCATION,
    Auth = sys::ODID_messagetype_ODID_MESSAGETYPE_AUTH,
    SelfId = sys::ODID_messagetype_ODID_MESSAGETYPE_SELF_ID,
    System = sys::ODID_messagetype_ODID_MESSAGETYPE_SYSTEM,
    OperatorId = sys::ODID_messagetype_ODID_MESSAGETYPE_OPERATOR_ID,
    MessagePack = sys::ODID_messagetype_ODID_MESSAGETYPE_PACKED,
    Invalid = sys::ODID_messagetype_ODID_MESSAGETYPE_INVALID,
}

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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum HeightReference {
    #[default]
    Takeoff = sys::ODID_Height_reference_ODID_HEIGHT_REF_OVER_TAKEOFF,
    Ground = sys::ODID_Height_reference_ODID_HEIGHT_REF_OVER_GROUND,
}

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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum TimestampAccuracy {
    #[default]
    Unknown = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_UNKNOWN,
    LessThan0_1Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_0_1_SECOND,
    LessThan0_2Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_0_2_SECOND,
    LessThan0_3Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_0_3_SECOND,
    LessThan0_4Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_0_4_SECOND,
    LessThan0_5Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_0_5_SECOND,
    LessThan0_6Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_0_6_SECOND,
    LessThan0_7Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_0_7_SECOND,
    LessThan0_8Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_0_8_SECOND,
    LessThan0_9Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_0_9_SECOND,
    LessThan1_0Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_1_0_SECOND,
    LessThan1_1Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_1_1_SECOND,
    LessThan1_2Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_1_2_SECOND,
    LessThan1_3Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_1_3_SECOND,
    LessThan1_4Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_1_4_SECOND,
    LessThan1_5Second = sys::ODID_Timestamp_accuracy_ODID_TIME_ACC_1_5_SECOND,
}

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

/// Common trait for all Open Drone ID messages, providing encoding and decoding functionality.
pub trait Message: Sized {
    type Data;
    type Encoded;

    /// Creates a new message with default values.
    /// The values are the default values defined by the underlying C library,
    /// which may not be valid.
    fn new() -> Self;
    /// Returns the length of the encoded message in bytes.
    fn encoded_len() -> usize;
    /// Encodes the message to a buffer.
    ///
    /// An error will be returned if the buffer does not have sufficient capacity.
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError>;
    /// Encodes the message to a newly allocated buffer.
    fn encode_to_vec(&self) -> Result<Vec<u8>, EncodeError>;
    /// Decodes the message from a buffer.
    fn decode(buf: impl Buf) -> Result<Self, DecodeError>;
}

/// Internal trait for encoding and decoding messages using the underlying C library functions.
trait MessageInternal: Sized {
    type Data;
    type Encoded;

    /// Initializes the message data structure to default values.
    fn init_data(data: *mut Self::Data);
    /// Encodes the message data to the encoded format using the underlying C library function.
    fn encode_message(
        out_encoded: *mut Self::Encoded,
        in_data: *const Self::Data,
    ) -> Result<(), EncodeError>;
    /// Decodes the message data from the encoded format using the underlying C library function.
    fn decode_message(
        out_data: *mut Self::Data,
        in_encoded: *const Self::Encoded,
    ) -> Result<(), DecodeError>;
}

impl_message!(
    BasicId,
    sys::ODID_BasicID_data,
    sys::ODID_BasicID_encoded,
    sys::odid_initBasicIDData,
    sys::encodeBasicIDMessage,
    sys::decodeBasicIDMessage
);

impl BasicId {
    pub fn ua_type(&self) -> Result<UaType, DecodeError> {
        UaType::from_u32(self.data.UAType)
            .ok_or(DecodeError::EnumMappingError("UAType", self.data.UAType))
    }

    pub fn with_ua_type(&mut self, ua_type: UaType) {
        self.data.UAType = ua_type as u32;
    }

    pub fn id_type(&self) -> Result<IdType, DecodeError> {
        IdType::from_u32(self.data.IDType)
            .ok_or(DecodeError::EnumMappingError("IDType", self.data.IDType))
    }

    pub fn with_id_type(&mut self, id_type: IdType) {
        self.data.IDType = id_type as u32;
    }

    pub fn uas_id(&self) -> &[i8; 21] {
        &self.data.UASID
    }

    pub fn with_uas_id(&mut self, uas_id: [i8; 21]) {
        self.data.UASID = uas_id;
    }
}

impl_message!(
    Location,
    sys::ODID_Location_data,
    sys::ODID_Location_encoded,
    sys::odid_initLocationData,
    sys::encodeLocationMessage,
    sys::decodeLocationMessage
);

impl Location {
    pub fn status(&self) -> Result<Status, DecodeError> {
        Status::from_u32(self.data.Status)
            .ok_or(DecodeError::EnumMappingError("Status", self.data.Status))
    }

    pub fn with_status(&mut self, status: Status) {
        self.data.Status = status as u32;
    }

    pub fn direction(&self) -> Option<f32> {
        if self.data.Direction == sys::INV_DIR as f32 {
            None
        } else {
            Some(self.data.Direction)
        }
    }

    pub fn with_direction(&mut self, direction: f32) -> Result<(), EncodeError> {
        if direction < sys::MIN_DIR as f32 || direction > sys::MAX_DIR as f32 {
            return Err(EncodeError::InvalidValue(
                "direction",
                direction.to_string(),
            ));
        }
        self.data.Direction = direction;
        Ok(())
    }

    pub fn speed_horizontal(&self) -> Option<f32> {
        if self.data.SpeedHorizontal == sys::INV_SPEED_H as f32 {
            None
        } else {
            Some(self.data.SpeedHorizontal)
        }
    }

    pub fn with_speed_horizontal(&mut self, speed: f32) -> Result<(), EncodeError> {
        if speed < sys::MIN_SPEED_H as f32 || speed > sys::MAX_SPEED_H as f32 {
            return Err(EncodeError::InvalidValue(
                "speed_horizontal",
                speed.to_string(),
            ));
        }
        self.data.SpeedHorizontal = speed;
        Ok(())
    }

    pub fn speed_vertical(&self) -> Option<f32> {
        if self.data.SpeedVertical == sys::INV_SPEED_V as f32 {
            None
        } else {
            Some(self.data.SpeedVertical)
        }
    }

    pub fn with_speed_vertical(&mut self, speed: f32) -> Result<(), EncodeError> {
        if speed < sys::MIN_SPEED_V as f32 || speed > sys::MAX_SPEED_V as f32 {
            return Err(EncodeError::InvalidValue(
                "speed_vertical",
                speed.to_string(),
            ));
        }
        self.data.SpeedVertical = speed;
        Ok(())
    }

    pub fn latitude(&self) -> f64 {
        self.data.Latitude
    }

    pub fn with_latitude(&mut self, latitude: f64) -> Result<(), EncodeError> {
        if latitude < sys::MIN_LAT as f64 || latitude > sys::MAX_LAT as f64 {
            return Err(EncodeError::InvalidValue("latitude", latitude.to_string()));
        }
        self.data.Latitude = latitude;
        Ok(())
    }

    pub fn longitude(&self) -> f64 {
        self.data.Longitude
    }
    pub fn with_longitude(&mut self, longitude: f64) -> Result<(), EncodeError> {
        if longitude < sys::MIN_LON as f64 || longitude > sys::MAX_LON as f64 {
            return Err(EncodeError::InvalidValue(
                "longitude",
                longitude.to_string(),
            ));
        }
        self.data.Longitude = longitude;
        Ok(())
    }
    pub fn altitude_barometric(&self) -> Option<f32> {
        if self.data.AltitudeBaro == sys::INV_ALT as f32 {
            None
        } else {
            Some(self.data.AltitudeBaro)
        }
    }
    pub fn with_altitude_barometric(&mut self, altitude: f32) -> Result<(), EncodeError> {
        if altitude < sys::MIN_ALT as f32 || altitude > sys::MAX_ALT as f32 {
            return Err(EncodeError::InvalidValue(
                "altitude_barometric",
                altitude.to_string(),
            ));
        }
        self.data.AltitudeBaro = altitude;
        Ok(())
    }
    pub fn altitude_geodetic(&self) -> Option<f32> {
        if self.data.AltitudeGeo == sys::INV_ALT as f32 {
            None
        } else {
            Some(self.data.AltitudeGeo)
        }
    }

    pub fn with_altitude_geodetic(&mut self, altitude: f32) -> Result<(), EncodeError> {
        if altitude < sys::MIN_ALT as f32 || altitude > sys::MAX_ALT as f32 {
            return Err(EncodeError::InvalidValue(
                "altitude_geodetic",
                altitude.to_string(),
            ));
        }
        self.data.AltitudeGeo = altitude;
        Ok(())
    }

    pub fn height_type(&self) -> Result<HeightReference, DecodeError> {
        HeightReference::from_u32(self.data.HeightType).ok_or(DecodeError::EnumMappingError(
            "HeightType",
            self.data.HeightType,
        ))
    }
    pub fn with_height_type(&mut self, height_type: HeightReference) {
        self.data.HeightType = height_type as u32;
    }

    pub fn height(&self) -> Option<f32> {
        if self.data.Height == sys::INV_ALT as f32 {
            None
        } else {
            Some(self.data.Height)
        }
    }
    pub fn with_height(&mut self, height: f32) -> Result<(), EncodeError> {
        if height < sys::MIN_ALT as f32 || height > sys::MAX_ALT as f32 {
            return Err(EncodeError::InvalidValue("height", height.to_string()));
        }
        self.data.Height = height;
        Ok(())
    }

    pub fn horizontal_accuracy(&self) -> Result<HorizontalAccuracy, DecodeError> {
        HorizontalAccuracy::from_u32(self.data.HorizAccuracy).ok_or(DecodeError::EnumMappingError(
            "HorizontalAccuracy",
            self.data.HorizAccuracy,
        ))
    }

    pub fn with_horizontal_accuracy(&mut self, accuracy: HorizontalAccuracy) {
        self.data.HorizAccuracy = accuracy as u32;
    }

    pub fn vertical_accuracy(&self) -> Result<VerticalAccuracy, DecodeError> {
        VerticalAccuracy::from_u32(self.data.VertAccuracy).ok_or(DecodeError::EnumMappingError(
            "VerticalAccuracy",
            self.data.VertAccuracy,
        ))
    }

    pub fn with_vertical_accuracy(&mut self, accuracy: VerticalAccuracy) {
        self.data.VertAccuracy = accuracy as u32;
    }

    pub fn barometric_accuracy(&self) -> Result<VerticalAccuracy, DecodeError> {
        VerticalAccuracy::from_u32(self.data.BaroAccuracy).ok_or(DecodeError::EnumMappingError(
            "BarometricAccuracy",
            self.data.BaroAccuracy,
        ))
    }

    pub fn with_barometric_accuracy(&mut self, accuracy: VerticalAccuracy) {
        self.data.BaroAccuracy = accuracy as u32;
    }

    pub fn speed_accuracy(&self) -> Result<SpeedAccuracy, DecodeError> {
        SpeedAccuracy::from_u32(self.data.SpeedAccuracy).ok_or(DecodeError::EnumMappingError(
            "SpeedAccuracy",
            self.data.SpeedAccuracy,
        ))
    }
    pub fn with_speed_accuracy(&mut self, accuracy: SpeedAccuracy) {
        self.data.SpeedAccuracy = accuracy as u32;
    }
    pub fn timestamp_accuracy(&self) -> Result<TimestampAccuracy, DecodeError> {
        TimestampAccuracy::from_u32(self.data.TSAccuracy).ok_or(DecodeError::EnumMappingError(
            "TimestampAccuracy",
            self.data.TSAccuracy,
        ))
    }
    pub fn with_timestamp_accuracy(&mut self, accuracy: TimestampAccuracy) {
        self.data.TSAccuracy = accuracy as u32;
    }

    /// Returns the timestamp as a floating point number of seconds since the start of the hour
    pub fn timestamp(&self) -> Option<f32> {
        if self.data.TimeStamp == sys::INV_TIMESTAMP as f32 {
            None
        } else {
            Some(self.data.TimeStamp)
        }
    }

    /// Set the timestamp as a floating point number of seconds since the start of the hour
    pub fn with_timestamp(&mut self, timestamp: f32) -> Result<(), EncodeError> {
        if timestamp < 0 as f32 || timestamp > sys::MAX_TIMESTAMP as f32 {
            return Err(EncodeError::InvalidValue(
                "timestamp",
                timestamp.to_string(),
            ));
        }
        self.data.TimeStamp = timestamp;
        Ok(())
    }

    #[cfg(feature = "chrono")]
    pub fn chrono_timestamp(&self) -> Option<Result<chrono::DateTime<chrono::Utc>, DecodeError>> {
        if self.data.TimeStamp == sys::INV_TIMESTAMP as f32 {
            None
        } else {
            Some(decode_timestamp(self.data.TimeStamp))
        }
    }

    #[cfg(feature = "chrono")]
    pub fn with_chrono_timestamp(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
        self.data.TimeStamp = encode_timestamp(timestamp);
    }
}

#[cfg(feature = "chrono")]
fn decode_timestamp(value: f32) -> Result<chrono::DateTime<chrono::Utc>, DecodeError> {
    use chrono::Timelike;
    let now = chrono::Utc::now();

    let this_hour_start = now
        .with_minute(0)
        .ok_or(DecodeError::InvalidValue(
            "timestamp.minute",
            "0".to_string(),
        ))?
        .with_second(0)
        .ok_or(DecodeError::InvalidValue(
            "timestamp.second",
            "0".to_string(),
        ))?
        .with_nanosecond(0)
        .ok_or(DecodeError::InvalidValue(
            "timestamp.nanosecond",
            "0".to_string(),
        ))?;
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
        .ok_or(DecodeError::InvalidValue(
            "timestamp.minute",
            value.to_string(),
        ))?
        .with_second((value % 60.0) as u32)
        .ok_or(DecodeError::InvalidValue(
            "timestamp.second",
            value.to_string(),
        ))?
        .with_nanosecond(((value % 1.0) * 1_000_000_000.0) as u32)
        .ok_or(DecodeError::InvalidValue(
            "timestamp.nanosecond",
            value.to_string(),
        ))
}

#[cfg(feature = "chrono")]
fn encode_timestamp(value: chrono::DateTime<chrono::Utc>) -> f32 {
    use chrono::Timelike;
    let mins = value.minute();
    let secs = value.second();
    let nanos = value.nanosecond();
    (mins as f32 * 60.0) + (secs as f32) + (nanos as f32 / 1_000_000_000.0)
}
