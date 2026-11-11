//! Limits and sizes of the fields of Open Drone ID messages.

use opendroneid_sys as sys;

/// Maximum altitude in meters that can be encoded in Open Drone ID messages
pub use sys::MAX_ALT as MAX_ALTITUDE;
/// Maximum heading direction in degrees that can be encoded in Open Drone ID messages
pub use sys::MAX_DIR as MAX_DIRECTION;
/// Maximum latitude in degrees that can be encoded in Open Drone ID messages
pub use sys::MAX_LAT as MAX_LATITUDE;
/// Maximum longitude in degrees that can be encoded in Open Drone ID messages
pub use sys::MAX_LON as MAX_LONGITUDE;
/// Maximum horizontal speed in m/s that can be encoded in Open Drone ID messages
pub use sys::MAX_SPEED_H as MAX_SPEED_HORIZONTAL;
/// Maximum vertical speed in m/s that can be encoded in Open Drone ID messages
pub use sys::MAX_SPEED_V as MAX_SPEED_VERTICAL;
/// Minimum altitude in meters that can be encoded in Open Drone ID messages, which also marks an
/// unknown altitude
pub use sys::MIN_ALT as MIN_ALTITUDE;
/// Minimum heading direction in degrees that can be encoded in Open Drone ID messages
pub use sys::MIN_DIR as MIN_DIRECTION;
/// Minimum latitude in degrees that can be encoded in Open Drone ID messages
pub use sys::MIN_LAT as MIN_LATITUDE;
/// Minimum longitude in degrees that can be encoded in Open Drone ID messages
pub use sys::MIN_LON as MIN_LONGITUDE;
/// Minimum horizontal speed in m/s that can be encoded in Open Drone ID messages
pub use sys::MIN_SPEED_H as MIN_SPEED_HORIZONTAL;
/// Minimum vertical speed in m/s that can be encoded in Open Drone ID messages
pub use sys::MIN_SPEED_V as MIN_SPEED_VERTICAL;
/// Size in bytes of the UAS ID field of a [`BasicId`](crate::BasicId) message
pub const UAS_ID_SIZE: usize = sys::ODID_ID_SIZE as usize;
/// Size in bytes of the [`Uuid`](crate::Uuid) of a [`UasId::UtmAssignedUuid`](crate::UasId::UtmAssignedUuid)
pub const UUID_SIZE: usize = 16;
/// Size in bytes of the [`SpecificSessionId::id`](crate::SpecificSessionId::id)
pub const SPECIFIC_SESSION_ID_SIZE: usize = UAS_ID_SIZE - 1;
/// Maximum number of pages of an [`Auth`](crate::Auth) message
pub const AUTH_MAX_PAGES: usize = sys::ODID_AUTH_MAX_PAGES as usize;
/// Size in bytes of the authentication data on the first [`AuthPage`](crate::AuthPage)
pub const AUTH_FIRST_PAGE_DATA_SIZE: usize = sys::ODID_AUTH_PAGE_ZERO_DATA_SIZE as usize;
/// Size in bytes of the authentication data on each later [`AuthPage`](crate::AuthPage)
pub const AUTH_NEXT_PAGE_DATA_SIZE: usize = sys::ODID_AUTH_PAGE_NONZERO_DATA_SIZE as usize;
