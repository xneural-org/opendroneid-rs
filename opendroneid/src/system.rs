//! System message, the operator location and the UA classification.

use bytes::{Buf, BufMut};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use opendroneid_sys as sys;

use crate::{
    Coordinates,
    error::Error,
    macros,
    message::{Message, MessageInternal},
    values::{checked_altitude, none_if},
};

/// Type of the [`System::operator_location`]. Accessible through [`OperatorLocation::location_type`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum OperatorLocationType {
    #[default]
    Takeoff = sys::ODID_operator_location_type_ODID_OPERATOR_LOCATION_TYPE_TAKEOFF,
    LiveGnss = sys::ODID_operator_location_type_ODID_OPERATOR_LOCATION_TYPE_LIVE_GNSS,
    Fixed = sys::ODID_operator_location_type_ODID_OPERATOR_LOCATION_TYPE_FIXED,
}

/// Type of the [`System::classification`]. Accessible through [`Classification::classification_type`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum ClassificationType {
    #[default]
    Undeclared = sys::ODID_classification_type_ODID_CLASSIFICATION_TYPE_UNDECLARED,
    EuropeanUnion = sys::ODID_classification_type_ODID_CLASSIFICATION_TYPE_EU,
}

/// EU category of the UA in a [`Classification::EuropeanUnion`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum Category {
    #[default]
    Undeclared = sys::ODID_category_EU_ODID_CATEGORY_EU_UNDECLARED,
    Open = sys::ODID_category_EU_ODID_CATEGORY_EU_OPEN,
    Specific = sys::ODID_category_EU_ODID_CATEGORY_EU_SPECIFIC,
    Certified = sys::ODID_category_EU_ODID_CATEGORY_EU_CERTIFIED,
}

/// EU class of the UA in a [`Classification::EuropeanUnion`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum ClassEu {
    #[default]
    Undeclared = sys::ODID_class_EU_ODID_CLASS_EU_UNDECLARED,
    Class0 = sys::ODID_class_EU_ODID_CLASS_EU_CLASS_0,
    Class1 = sys::ODID_class_EU_ODID_CLASS_EU_CLASS_1,
    Class2 = sys::ODID_class_EU_ODID_CLASS_EU_CLASS_2,
    Class3 = sys::ODID_class_EU_ODID_CLASS_EU_CLASS_3,
    Class4 = sys::ODID_class_EU_ODID_CLASS_EU_CLASS_4,
    Class5 = sys::ODID_class_EU_ODID_CLASS_EU_CLASS_5,
    Class6 = sys::ODID_class_EU_ODID_CLASS_EU_CLASS_6,
}

/// Location of the operator in the [`System`] message. The variant is the [`OperatorLocationType`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperatorLocation {
    /// The operator is at the takeoff location.
    Takeoff(OperatorPosition),
    /// Dynamic location of the operator from a live GNSS.
    LiveGnss(OperatorPosition),
    /// Fixed location of the operator.
    Fixed(OperatorPosition),
}

impl OperatorLocation {
    /// Returns the [`OperatorLocationType`] of the location.
    pub fn location_type(&self) -> OperatorLocationType {
        match self {
            OperatorLocation::Takeoff(_) => OperatorLocationType::Takeoff,
            OperatorLocation::LiveGnss(_) => OperatorLocationType::LiveGnss,
            OperatorLocation::Fixed(_) => OperatorLocationType::Fixed,
        }
    }

    /// Returns the position of the operator.
    pub fn position(&self) -> &OperatorPosition {
        match self {
            OperatorLocation::Takeoff(position)
            | OperatorLocation::LiveGnss(position)
            | OperatorLocation::Fixed(position) => position,
        }
    }
}

/// Position of the operator in an [`OperatorLocation`].
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct OperatorPosition {
    /// Coordinates of the operator, `None` if they are unknown.
    pub coordinates: Option<Coordinates>,
    /// Geodetic altitude above the WGS-84 ellipsoid in meters, `None` if it is unknown.
    pub altitude_geodetic: Option<f32>,
}

/// Classification of the UA in the [`System`] message. The variant is the [`ClassificationType`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Classification {
    /// The classification is not declared.
    #[default]
    Undeclared,
    /// Classification of the European Union.
    EuropeanUnion {
        /// EU category of the UA.
        category: Category,
        /// EU class of the UA.
        class: ClassEu,
    },
}

impl Classification {
    /// Returns the [`ClassificationType`] of the classification.
    pub fn classification_type(&self) -> ClassificationType {
        match self {
            Classification::Undeclared => ClassificationType::Undeclared,
            Classification::EuropeanUnion { .. } => ClassificationType::EuropeanUnion,
        }
    }
}

macros::impl_message!(
    /// The System Message contains general system information including information about the
    /// Remote Pilot location and flight area. It can also include information about UAs flying
    /// in formation.
    System,
    sys::ODID_System_data,
    sys::ODID_System_encoded,
    sys::odid_initSystemData,
    sys::encodeSystemMessage,
    sys::decodeSystemMessage
);

impl System {
    /// Returns the location of the operator, or an [`Error`] if the location type is invalid.
    ///
    /// The variant of the [`OperatorLocation`] is the operator location type of the message.
    pub fn operator_location(&self) -> Result<OperatorLocation, Error> {
        let location_type = OperatorLocationType::from_u32(self.data.OperatorLocationType).ok_or(
            Error::EnumMappingError {
                field: "OperatorLocationType",
                value: self.data.OperatorLocationType,
            },
        )?;
        let position = OperatorPosition {
            coordinates: Coordinates::known(
                self.data.OperatorLatitude,
                self.data.OperatorLongitude,
            ),
            altitude_geodetic: none_if(self.data.OperatorAltitudeGeo, sys::INV_ALT as f32),
        };
        Ok(match location_type {
            OperatorLocationType::Takeoff => OperatorLocation::Takeoff(position),
            OperatorLocationType::LiveGnss => OperatorLocation::LiveGnss(position),
            OperatorLocationType::Fixed => OperatorLocation::Fixed(position),
        })
    }

    /// Sets the location of the operator and its type.
    /// Returns an error if a coordinate or the altitude is out of the valid range defined by the
    /// protocol, or for 0°/0° or [`MIN_ALTITUDE`](crate::MIN_ALTITUDE), which mark unknown values.
    pub fn with_operator_location(mut self, location: OperatorLocation) -> Result<Self, Error> {
        let position = location.position();
        (self.data.OperatorLatitude, self.data.OperatorLongitude) =
            Coordinates::checked(position.coordinates)?;
        self.data.OperatorAltitudeGeo =
            checked_altitude("operator_altitude_geodetic", position.altitude_geodetic)?;
        self.data.OperatorLocationType = location.location_type() as u32;
        Ok(self)
    }

    /// Returns the classification of the UA, or an [`Error`] if the classification type, the
    /// category or the class is invalid.
    ///
    /// The variant of the [`Classification`] is the classification type of the message.
    pub fn classification(&self) -> Result<Classification, Error> {
        let classification_type = ClassificationType::from_u32(self.data.ClassificationType)
            .ok_or(Error::EnumMappingError {
                field: "ClassificationType",
                value: self.data.ClassificationType,
            })?;
        Ok(match classification_type {
            ClassificationType::Undeclared => Classification::Undeclared,
            ClassificationType::EuropeanUnion => Classification::EuropeanUnion {
                category: Category::from_u32(self.data.CategoryEU).ok_or(
                    Error::EnumMappingError {
                        field: "CategoryEU",
                        value: self.data.CategoryEU,
                    },
                )?,
                class: ClassEu::from_u32(self.data.ClassEU).ok_or(Error::EnumMappingError {
                    field: "ClassEU",
                    value: self.data.ClassEU,
                })?,
            },
        })
    }

    /// Sets the classification of the UA and its type.
    pub fn with_classification(mut self, classification: Classification) -> Self {
        self.data.ClassificationType = classification.classification_type() as u32;
        let (category, class) = match classification {
            Classification::Undeclared => (Category::Undeclared, ClassEu::Undeclared),
            Classification::EuropeanUnion { category, class } => (category, class),
        };
        self.data.CategoryEU = category as u32;
        self.data.ClassEU = class as u32;
        self
    }

    /// Returns the area count value.
    pub fn area_count(&self) -> u16 {
        self.data.AreaCount
    }

    /// Sets the area count value.
    pub fn with_area_count(mut self, area_count: u16) -> Self {
        self.data.AreaCount = area_count;
        self
    }

    /// Returns the area radius in meters.
    pub fn area_radius(&self) -> u16 {
        self.data.AreaRadius
    }

    /// Sets the area radius in meters.
    pub fn with_area_radius(mut self, area_radius: u16) -> Result<Self, Error> {
        if area_radius > sys::MAX_AREA_RADIUS as u16 {
            return Err(Error::InvalidValue {
                field: "area_radius",
                value: area_radius.to_string(),
            });
        }
        self.data.AreaRadius = area_radius;
        Ok(self)
    }

    /// Returns the geodetic altitude of the area ceiling in meters, or `None` if it is unknown.
    pub fn area_ceiling(&self) -> Option<f32> {
        none_if(self.data.AreaCeiling, sys::INV_ALT as f32)
    }

    /// Sets the geodetic altitude of the area ceiling in meters, `None` if it is unknown.
    /// Returns an error if the altitude is out of the valid range defined by the protocol, or for
    /// [`MIN_ALTITUDE`](crate::MIN_ALTITUDE), which marks an unknown altitude.
    pub fn with_area_ceiling(mut self, altitude: impl Into<Option<f32>>) -> Result<Self, Error> {
        self.data.AreaCeiling = checked_altitude("area_ceiling", altitude.into())?;
        Ok(self)
    }

    /// Returns the geodetic altitude of the area floor in meters, or `None` if it is unknown.
    pub fn area_floor(&self) -> Option<f32> {
        none_if(self.data.AreaFloor, sys::INV_ALT as f32)
    }

    /// Sets the geodetic altitude of the area floor in meters, `None` if it is unknown.
    /// Returns an error if the altitude is out of the valid range defined by the protocol, or for
    /// [`MIN_ALTITUDE`](crate::MIN_ALTITUDE), which marks an unknown altitude.
    pub fn with_area_floor(mut self, altitude: impl Into<Option<f32>>) -> Result<Self, Error> {
        self.data.AreaFloor = checked_altitude("area_floor", altitude.into())?;
        Ok(self)
    }

    /// Returns the timestamp in seconds since 2019-01-01 00:00:00 UTC.
    ///
    /// Convert it with `from_odid_epoch` (feature `chrono`).
    pub fn timestamp(&self) -> u32 {
        self.data.Timestamp
    }

    /// Sets the timestamp in seconds since 2019-01-01 00:00:00 UTC.
    ///
    /// Convert a time with `to_odid_epoch` (feature `chrono`).
    pub fn with_timestamp(mut self, timestamp: u32) -> Self {
        self.data.Timestamp = timestamp;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::*;

    #[test]
    fn operator_location_round_trips_with_type() {
        let position = OperatorPosition {
            coordinates: Some(MUNICH),
            altitude_geodetic: Some(520.5),
        };
        for location in [
            OperatorLocation::Takeoff(position),
            OperatorLocation::LiveGnss(position),
            OperatorLocation::Fixed(OperatorPosition::default()),
        ] {
            let decoded = round_trip(&System::default().with_operator_location(location).unwrap())
                .operator_location()
                .unwrap();

            assert_eq!(decoded.location_type(), location.location_type());
            assert_coordinates_eq(
                decoded.position().coordinates,
                location.position().coordinates,
            );
            assert_eq!(
                decoded.position().altitude_geodetic,
                location.position().altitude_geodetic
            );
        }
    }

    #[test]
    fn operator_location_out_of_range_is_rejected() {
        let result =
            System::default().with_operator_location(OperatorLocation::Fixed(OperatorPosition {
                coordinates: Some(Coordinates {
                    latitude: 91.0,
                    longitude: 0.0,
                }),
                altitude_geodetic: None,
            }));

        assert!(matches!(
            result,
            Err(Error::InvalidValue {
                field: "latitude",
                ..
            })
        ));
    }

    #[test]
    fn area_ceiling_and_floor_round_trip() {
        let system = System::default()
            .with_area_ceiling(150.0)
            .unwrap()
            .with_area_floor(None)
            .unwrap();

        let decoded = round_trip(&system);

        assert_eq!(decoded.area_ceiling(), Some(150.0));
        assert_eq!(decoded.area_floor(), None);
        assert_eq!(System::default().area_ceiling(), None);
    }

    #[test]
    fn classification_round_trips() {
        for classification in [
            Classification::Undeclared,
            Classification::EuropeanUnion {
                category: Category::Specific,
                class: ClassEu::Class2,
            },
        ] {
            let message = System::default().with_classification(classification);

            assert_eq!(
                round_trip(&message).classification().unwrap(),
                classification
            );
        }
    }

    #[test]
    fn undeclared_classification_clears_category_and_class() {
        let message = System::default()
            .with_classification(Classification::EuropeanUnion {
                category: Category::Open,
                class: ClassEu::Class1,
            })
            .with_classification(Classification::Undeclared);

        assert_eq!(message.data.CategoryEU, Category::Undeclared as u32);
        assert_eq!(message.data.ClassEU, ClassEu::Undeclared as u32);
    }
}
