//! Group of messages of one UAS, decoded from and encoded to message packs.

use bytes::Buf;
use opendroneid_sys as sys;

use crate::{
    Auth, BasicId, Location, Message, MessageId, OperatorId, SelfId, System, error::Error,
};

/// UAS data is a collection of Open Drone ID messages that together represent the state and
/// identity of a UAS.
///
/// It may contain several Basic ID and Auth messages, but at most one of each of the other message
/// types. Decode it from a single message or a message pack with [`UasData::decode`] and encode it
/// as a message pack with [`UasData::encode_to_vec`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct UasData {
    /// The Basic ID messages associated with this UAS data, at most one per [`IdType`](crate::IdType).
    pub basic_id: Vec<BasicId>,
    /// The Location message associated with this UAS data, if available.
    pub location: Option<Location>,
    /// The Auth messages associated with this UAS data, one per [`AuthPage`](crate::AuthPage).
    pub auth: Vec<Auth>,
    /// The Self ID message associated with this UAS data, if available.
    pub self_id: Option<SelfId>,
    /// The System message associated with this UAS data, if available.
    pub system: Option<System>,
    /// The Operator ID message associated with this UAS data, if available.
    pub operator_id: Option<OperatorId>,
}

impl UasData {
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

    /// Encodes the messages as a message pack.
    ///
    /// Returns an error if there is no message, if there are more than `ODID_PACK_MAX_MESSAGES`
    /// messages, or if a message cannot be encoded.
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
    use crate::testing::*;
    use crate::{Height, UasId, Uuid};

    const MESSAGE_SIZE: usize = sys::ODID_MESSAGE_SIZE as usize;

    fn uas_data() -> UasData {
        UasData {
            basic_id: vec![basic_id(UasId::UtmAssignedUuid(Uuid::from_bytes(UUID)))],
            location: Some(
                Location::default()
                    .with_height(Height::AboveTakeoff(42.0))
                    .unwrap(),
            ),
            ..UasData::default()
        }
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

        assert_eq!(decoded.basic_id, [message]);
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
}
