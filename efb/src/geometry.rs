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
use std::ops::Add;

use crate::algorithm;

/// An angle as value between 0° and 360°.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Angle {
    pub deg: i16,
    pub rad: f32,
}

impl Angle {
    pub fn from_deg(deg: i16) -> Self {
        Self {
            deg: deg,
            rad: (deg as f32).to_radians(),
        }
    }

    pub fn from_rad(rad: f32) -> Self {
        Self {
            deg: rad.to_degrees().round() as i16,
            rad: rad,
        }
    }
}

impl Add for Angle {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from_deg((self.deg + other.deg) % 360)
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{0:03}", self.deg)
    }
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

#[macro_export]
macro_rules! coord {
    ($latitude:expr, $longitude:expr) => {
        Coordinate {
            latitude: $latitude,
            longitude: $longitude,
        }
    };
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({0}, {1})", self.latitude, self.longitude)
    }
}

pub type Line = (Coordinate, Coordinate);

pub type Polygon = Vec<Coordinate>;

/// Creates a [`Polygon`] containing the coordinates.
///
/// ```
/// use efb::polygon;
/// use efb::geometry::Coordinate;
///
/// let p = polygon![(0.0, 0.0), (10.0, 10.0)];
/// assert_eq!(p[0], Coordinate { latitude: 0.0, longitude: 0.0 });
/// assert_eq!(p[1], Coordinate { latitude: 10.0, longitude: 10.0 });
/// ```
#[macro_export]
macro_rules! polygon {
    ( $( $p:expr ),* ) => {
        {
            let mut polygon = Vec::new();
            $(
                polygon.push(
                    Coordinate {
                        latitude: $p.0,
                        longitude: $p.1,
                    }
                );
            )*
            polygon
        }
    };
}

/// Returns `true` if the `point` is inside the `polygon`.
pub fn point_in_polygon(point: &Coordinate, polygon: &Polygon) -> bool {
    algorithm::winding_number(point, polygon) != 0
}

/// A vertical distance value.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VerticalDistance {
    /// Absolute distance above ground level in feet.
    Agl(u16),

    /// Altitude in feet with reference to a local air pressure.
    Altitude(u16),

    /// Flight level in hundreds of feet as altitude at standard air pressure.
    Fl(u16),

    /// Ground level.
    Gnd,

    /// Distance above mean sea level at standard air pressure.
    Msl(u16),

    /// An unlimited vertical distance.
    Unlimited,
}

// TODO: Do we need the default?
impl Default for VerticalDistance {
    fn default() -> Self {
        Self::Gnd
    }
}

impl Display for VerticalDistance {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            VerticalDistance::Gnd => write!(f, "GND"),
            VerticalDistance::Fl(value) => write!(f, "FL{value}"),
            VerticalDistance::Agl(value) => write!(f, "{value} AGL"),
            VerticalDistance::Msl(value) => write!(f, "{value} MSL"),
            VerticalDistance::Altitude(value) => write!(f, "{value} ALT"),
            VerticalDistance::Unlimited => write!(f, "unlimited"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_is_in_polygon() {
        let point = coord!(15.0, 15.0);
        let polygon = polygon![
            (10.0, 10.0),
            (20.0, 10.0),
            (20.0, 20.0),
            (10.0, 20.0),
            (10.0, 10.0)
        ];
        assert!(point_in_polygon(&point, &polygon));
    }

    #[test]
    fn point_is_not_in_polygon() {
        let point = coord!(20.0, 0.0);
        let polygon = polygon![
            (-10.0, 10.0),
            (10.0, 10.0),
            (10.0, -10.0),
            (-10.0, -10.0),
            (-10.0, 10.0)
        ];
        assert!(!point_in_polygon(&point, &polygon));
    }
}
