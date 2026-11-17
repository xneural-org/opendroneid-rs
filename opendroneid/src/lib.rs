//! This crate defines Rust abstractions for the structs and enums exported by [`opendroneid_sys`]
//! from the C-library.
//!
//! * Message types are implemented as a wrapper around the `ODID_*_data` structs from
//!   [`opendroneid_sys`]
//! * `const` values exported by [`opendroneid_sys`] are defined as enums or as constants in
//!   [`constants`]
//! * Values whose meaning depends on a type field are enums whose variant is that type, for example
//!   [`UasId`] and [`Height`]
//! * Values the UA does not know are `None`
//!
//! Each message type implements the [`Message`] trait that allows decoding and encoding of messages.
//! A special case is the [`UasData`] struct, which holds a group of messages that can be sent
//! together in a message pack.

pub mod constants;
pub mod error;

mod auth;
mod basic_id;
mod location;
mod macros;
mod message;
mod operator_id;
mod self_id;
mod system;
#[cfg(test)]
mod testing;
#[cfg(feature = "chrono")]
mod time;
mod uas_data;
mod utils;
mod values;

pub use auth::*;
pub use basic_id::*;
pub use constants::*;
pub use error::*;
pub use location::*;
pub use message::*;
pub use operator_id::*;
pub use self_id::*;
pub use system::*;
#[cfg(feature = "chrono")]
pub use time::*;
pub use uas_data::*;
pub use uuid::Uuid;
