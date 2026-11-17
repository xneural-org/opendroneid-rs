//! Operator ID message.

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

/// Type of the Operator ID
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum OperatorIdType {
    #[default]
    OperatorId = sys::ODID_operatorIdType_ODID_OPERATOR_ID,
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
