//! Errors of encoding, decoding and building Open Drone ID messages.

use std::borrow;

/// Error of encoding, decoding or building an Open Drone ID message.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The C library failed to encode or decode a message.
    #[error("Can't {operation} {message} in wire format")]
    Unknown {
        /// `"encode"` or `"decode"`.
        operation: &'static str,
        /// Name of the message.
        message: borrow::Cow<'static, str>,
    },
    /// The buffer holds fewer bytes than the message needs.
    #[error(
        "{operation} buffer too small for {message}: remaining {remaining}, required {required}"
    )]
    BufferTooSmall {
        /// `"encode"` or `"decode"`.
        operation: &'static str,
        /// Name of the message.
        message: borrow::Cow<'static, str>,
        /// Bytes left in the buffer.
        remaining: usize,
        /// Bytes the message needs.
        required: usize,
    },
    /// A value is out of the range defined by the protocol.
    #[error("Invalid value for {field}: {value}")]
    InvalidValue {
        /// Name of the field.
        field: &'static str,
        /// The value, or why it is invalid.
        value: String,
    },
    /// A raw value does not map to a variant of its enum.
    #[error("Enum mapping error: {field} has invalid value {value}")]
    EnumMappingError {
        /// Name of the field.
        field: &'static str,
        /// The raw value.
        value: u32,
    },
}
