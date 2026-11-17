//! Helpers shared by the tests of several modules.

use crate::*;

/// A 16-byte UUID that contains `0x00` (also as first byte) and bytes `>= 0x80`.
pub(crate) const UUID: [u8; UUID_SIZE] = [
    0x00, 0x9f, 0x3c, 0x80, 0xff, 0x00, 0x4a, 0x11, 0x81, 0x00, 0xc3, 0x28, 0x7e, 0xfe, 0x00, 0x01,
];

pub(crate) fn round_trip<M: Message>(message: &M) -> M {
    let encoded = message.encode_to_vec().expect("encoding should succeed");
    M::decode(encoded.as_slice()).expect("decoding should succeed")
}

pub(crate) fn basic_id(uas_id: UasId) -> BasicId {
    BasicId::default()
        .with_ua_type(UaType::HelicopterOrMultirotor)
        .with_uas_id(uas_id)
        .expect("UAS ID should fit")
}

pub(crate) const MUNICH: Coordinates = Coordinates {
    latitude: 48.137_154_3,
    longitude: 11.575_382_1,
};

pub(crate) fn assert_coordinates_eq(actual: Option<Coordinates>, expected: Option<Coordinates>) {
    match (actual, expected) {
        (Some(actual), Some(expected)) => {
            assert!((actual.latitude - expected.latitude).abs() < 1e-7);
            assert!((actual.longitude - expected.longitude).abs() < 1e-7);
        }
        (actual, expected) => assert_eq!(actual, expected),
    }
}

#[cfg(feature = "chrono")]
pub(crate) fn utc(time: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(time).unwrap().to_utc()
}
