use std::borrow;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Can't {operation} {message} in wire format")]
    Unknown {
        operation: &'static str,
        message: borrow::Cow<'static, str>,
    },
    #[error(
        "{operation} buffer too small for {message}: remaining {remaining}, required {required}"
    )]
    BufferTooSmall {
        operation: &'static str,
        message: borrow::Cow<'static, str>,
        remaining: usize,
        required: usize,
    },
    #[error("Invalid value for {field}: {value}")]
    InvalidValue { field: &'static str, value: String },
    #[error("Enum mapping error: {field} has invalid value {value}")]
    EnumMappingError { field: &'static str, value: u32 },
    // Add more variants as needed
}
