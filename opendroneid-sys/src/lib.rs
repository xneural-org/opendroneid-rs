//! sys crate for the [`opendroneid-core-c`](https://github.com/opendroneid/opendroneid-core-c) library
//!
//! Do not use this crate, use [opendroneid](https://crates.io/opendroneid) instead!
//!
//! All message structs are exposed as types.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(clippy::all)]

use std::hash::{Hash, Hasher};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Eq for ODID_Location_data {}

impl Hash for ODID_Location_data {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.Status.hash(state);
        self.Direction.to_bits().hash(state);
        self.SpeedHorizontal.to_bits().hash(state);
        self.SpeedVertical.to_bits().hash(state);
        self.Latitude.to_bits().hash(state);
        self.Longitude.to_bits().hash(state);
        self.AltitudeBaro.to_bits().hash(state);
        self.AltitudeGeo.to_bits().hash(state);
        self.HeightType.hash(state);
        self.Height.to_bits().hash(state);
        self.HorizAccuracy.hash(state);
        self.VertAccuracy.hash(state);
        self.BaroAccuracy.hash(state);
        self.SpeedAccuracy.hash(state);
        self.TSAccuracy.hash(state);
        self.TimeStamp.to_bits().hash(state);
    }
}

impl Eq for ODID_System_data {}

impl Hash for ODID_System_data {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.OperatorLocationType.hash(state);
        self.ClassificationType.hash(state);
        self.OperatorLatitude.to_bits().hash(state);
        self.OperatorLongitude.to_bits().hash(state);
        self.AreaCount.hash(state);
        self.AreaRadius.hash(state);
        self.AreaCeiling.to_bits().hash(state);
        self.AreaFloor.to_bits().hash(state);
        self.CategoryEU.hash(state);
        self.ClassEU.hash(state);
        self.OperatorAltitudeGeo.to_bits().hash(state);
        self.Timestamp.hash(state);
    }
}
