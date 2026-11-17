//! Self ID message.

use bytes::{Buf, BufMut};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use opendroneid_sys as sys;

use crate::{
    error::Error,
    macros,
    message::{Message, MessageInternal},
    utils,
};

/// Type of the description in the [`SelfId`] message. Accessible through [`SelfId::desc_type`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum DescriptionType {
    #[default]
    Text = sys::ODID_desctype_ODID_DESC_TYPE_TEXT,
    Emergency = sys::ODID_desctype_ODID_DESC_TYPE_EMERGENCY,
    ExtendedStatus = sys::ODID_desctype_ODID_DESC_TYPE_EXTENDED_STATUS,
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
