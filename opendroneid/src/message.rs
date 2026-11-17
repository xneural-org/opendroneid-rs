//! Message trait shared by all Open Drone ID message types.

use bytes::{Buf, BufMut};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use opendroneid_sys as sys;

use crate::error::Error;

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
pub(crate) trait MessageInternal: Sized {
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
