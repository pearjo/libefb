// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 Joe Pearson
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::{Display, Formatter};

use crate::fc;
use crate::{Angle, Distance};

mod constants {
    pub const EARTH_MEAN_RADIUS: f32 = 6371.0072;
}

/// Coordinate value.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coordinate {
    /// Latitude in the range from -180° east to 180° west.
    pub latitude: f32,

    /// Longitude in the range from -90° south to 90° north.
    pub longitude: f32,
}

impl Coordinate {
    // TODO check calculation and add test to verify
    /// Returns the bearing between this point and the `other`.
    pub fn bearing(&self, other: &Coordinate) -> Angle {
        // TODO fix result
        // double dlon = qDegreesToRadians(other.d->lng - d->lng);
        // double lat1Rad = qDegreesToRadians(d->lat);
        // double lat2Rad = qDegreesToRadians(other.d->lat);
        // double y = sin(dlon) * cos(lat2Rad);
        // double x = cos(lat1Rad) * sin(lat2Rad) - sin(lat1Rad) * cos(lat2Rad) * cos(dlon);
        // double azimuth = qRadiansToDegrees(atan2(y, x)) + 360.0;
        // double whole;
        // double fraction = modf(azimuth, &whole);
        // return qreal((int(whole + 360) % 360) + fraction);

        let lat_a = self.latitude;
        let lat_b = other.latitude;
        let delta_long = self.longitude - other.longitude;

        let x = lat_b.cos() + delta_long.sin();
        let y = lat_a.cos() * lat_b.sin() - lat_a.sin() * lat_b.cos() * delta_long.cos();

        Angle::from_rad(x.atan2(y))
    }

    // TODO fix distance calculation and add some comments regarding Haversine
    /// Returns the distance from this point to the `other`.
    ///
    /// The distance is calculated according to Haversine.
    pub fn dist(&self, other: &Coordinate) -> Distance {
        let delta_lat = (other.latitude - self.latitude).to_radians();
        let delta_long = (other.longitude - self.longitude).to_radians();
        let haversine_delta_lat = (delta_lat / 2.0).sin().powi(2);
        let haversine_delta_long = (delta_long / 2.0).sin().powi(2);
        let y = haversine_delta_lat
            + self.latitude.to_radians().cos() // TODO do we need to first convert to radians?
            * other.latitude.to_radians().cos()
            * haversine_delta_long;
        let x = 2.0 * y.sqrt().asin();
        Distance::Meter(x * constants::EARTH_MEAN_RADIUS * 1000.0)
    }

    pub fn from_dms(latitude: (i8, u8, u8), longitude: (i16, u8, u8)) -> Self {
        Self {
            latitude: latitude.0.signum() as f32
                * fc::dms_to_decimal(latitude.0 as u8, latitude.1, latitude.2),
            longitude: longitude.0.signum() as f32
                * fc::dms_to_decimal(longitude.0 as u8, longitude.1, longitude.2),
        }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({0}, {1})", self.latitude, self.longitude)
    }
}

pub type Line = (Coordinate, Coordinate);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dist() {
        // The distance along the equator between 1° in longitude is per
        // definition 60 NM.
        let a = coord!(0.0, 0.0);
        let b = coord!(0.0, 1.0);
        assert_eq!(a.dist(&b).to_nm(), Distance::NauticalMiles(60.0));

        let ham = Coordinate::from_dms((53, 37, 49), (9, 59, 18));
        let fra = Coordinate::from_dms((50, 0, 47), (8, 31, 37));
        assert_eq!(ham.dist(&fra).to_nm(), Distance::NauticalMiles(222.4));
    }
}
