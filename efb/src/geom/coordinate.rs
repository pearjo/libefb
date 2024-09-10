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

use std::fmt::{Display, Formatter, Result};

use super::Angle;

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

    /// Returns the distance from this point to the `other`.
    pub fn dist(&self, other: &Coordinate) -> f32 {
        // Haversine

        let delta_lat = other.latitude - self.latitude;
        let delta_long = other.longitude - self.longitude;

        // double dlat = qDegreesToRadians(other.d->lat - d->lat);
        // double dlon = qDegreesToRadians(other.d->lng - d->lng);
        // double haversine_dlat = sin(dlat / 2.0);
        // haversine_dlat *= haversine_dlat;
        // double haversine_dlon = sin(dlon / 2.0);
        // haversine_dlon *= haversine_dlon;
        // double y = haversine_dlat
        //     + cos(qDegreesToRadians(d->lat))
        //     * cos(qDegreesToRadians(other.d->lat))
        //     * haversine_dlon;
        // double x = 2 * asin(sqrt(y));
        // return qreal(x * qgeocoordinate_EARTH_MEAN_RADIUS * 1000);
        0.0
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({0}, {1})", self.latitude, self.longitude)
    }
}

pub type Line = (Coordinate, Coordinate);

#[cfg(test)]
mod tests {}
