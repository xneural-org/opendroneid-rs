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
    #[error("Invalid value for {0}: {1}")]
    InvalidValue(&'static str, String),
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
    #[error("Invalid value for {0}: {1}")]
    InvalidValue(&'static str, String),
}
