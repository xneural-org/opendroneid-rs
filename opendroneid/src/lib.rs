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
pub enum IDType {
    #[default]
    None = sys::ODID_idtype_ODID_IDTYPE_NONE,
    SerialNumber = sys::ODID_idtype_ODID_IDTYPE_SERIAL_NUMBER,
    CAARegistrationId = sys::ODID_idtype_ODID_IDTYPE_CAA_REGISTRATION_ID,
    UTMAssignedUuid = sys::ODID_idtype_ODID_IDTYPE_UTM_ASSIGNED_UUID,
    SpecificSessionId = sys::ODID_idtype_ODID_IDTYPE_SPECIFIC_SESSION_ID,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum UAType {
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
    OperatorID = sys::ODID_operatorIdType_ODID_OPERATOR_ID,
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
    EU = sys::ODID_classification_type_ODID_CLASSIFICATION_TYPE_EU,
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
    pub fn ua_type(&self) -> Result<UAType, DecodeError> {
        UAType::from_u32(self.data.UAType)
            .ok_or(DecodeError::EnumMappingError("UAType", self.data.UAType))
    }

    pub fn with_ua_type(&mut self, ua_type: UAType) {
        self.data.UAType = ua_type as u32;
    }

    pub fn id_type(&self) -> Result<IDType, DecodeError> {
        IDType::from_u32(self.data.IDType)
            .ok_or(DecodeError::EnumMappingError("IDType", self.data.IDType))
    }

    pub fn with_id_type(&mut self, id_type: IDType) {
        self.data.IDType = id_type as u32;
    }

    pub fn uas_id(&self) -> &[i8; 21] {
        &self.data.UASID
    }

    pub fn with_uas_id(&mut self, uas_id: [i8; 21]) {
        self.data.UASID = uas_id;
    }
}
