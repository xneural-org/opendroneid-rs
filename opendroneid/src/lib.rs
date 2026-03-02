pub use opendroneid_sys as sys;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error {
    pub code: i32,
}

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

pub type BasicIdData = sys::ODID_BasicID_data;
pub type LocationData = sys::ODID_Location_data;
pub type AuthData = sys::ODID_Auth_data;
pub type SelfIdData = sys::ODID_SelfID_data;
pub type SystemData = sys::ODID_System_data;
pub type OperatorIdData = sys::ODID_OperatorID_data;
pub type UasData = sys::ODID_UAS_Data;
pub type MessagePackData = sys::ODID_MessagePack_data;

pub type BasicIdEncoded = sys::ODID_BasicID_encoded;
pub type LocationEncoded = sys::ODID_Location_encoded;
pub type AuthEncoded = sys::ODID_Auth_encoded;
pub type SelfIdEncoded = sys::ODID_SelfID_encoded;
pub type SystemEncoded = sys::ODID_System_encoded;
pub type OperatorIdEncoded = sys::ODID_OperatorID_encoded;
pub type MessagePackEncoded = sys::ODID_MessagePack_encoded;
