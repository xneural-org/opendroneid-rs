//! Basic ID message and the UAS ID.

use bytes::{Buf, BufMut};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use opendroneid_sys as sys;

use crate::{
    Uuid,
    constants::{SPECIFIC_SESSION_ID_SIZE, UAS_ID_SIZE, UUID_SIZE},
    error::Error,
    macros,
    message::{Message, MessageInternal},
    utils,
};

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
    /// Returns an error if a text ID is not valid, see [`BasicId::with_uas_id`].
    pub fn to_bytes(&self) -> Result<[u8; UAS_ID_SIZE], Error> {
        let mut bytes = [0u8; UAS_ID_SIZE];
        match self {
            UasId::None(id) => bytes = *id,
            UasId::SerialNumber(id) | UasId::CaaRegistrationId(id) => {
                // ASTM F3411-22a: text IDs are ASCII strings padded with nulls (BMG0060), and
                // CAA registration IDs only use uppercase letters, digits and dots.
                let valid_char = |c: char| match self {
                    UasId::CaaRegistrationId(_) => {
                        c.is_ascii_uppercase() || c.is_ascii_digit() || c == '.'
                    }
                    _ => c.is_ascii() && c != '\0',
                };
                if id.len() > UAS_ID_SIZE || !id.chars().all(valid_char) {
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
    /// than [`UAS_ID_SIZE`] bytes, or contains a character that ASTM F3411-22a does not allow:
    /// a serial number must be ASCII without `0x00` bytes, a CAA registration ID may only
    /// contain uppercase letters (`A-Z`), digits (`0-9`) and dots (`.`).
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::*;

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
            UasId::CaaRegistrationId("FIN.87ASTRDGE12K8".into()),
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
    fn text_id_with_invalid_characters_is_rejected() {
        for uas_id in [
            UasId::SerialNumber("AB\0CD".into()),
            UasId::SerialNumber("1596F1ÄB".into()),
            UasId::CaaRegistrationId("N.12\0X".into()),
            UasId::CaaRegistrationId("N.abc123".into()),
            UasId::CaaRegistrationId("N-123456".into()),
            UasId::CaaRegistrationId("N.123 456".into()),
        ] {
            assert!(matches!(
                BasicId::default().with_uas_id(uas_id),
                Err(Error::InvalidValue { field: "UASID", .. })
            ));
        }
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
}
