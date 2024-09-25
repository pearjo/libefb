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
use std::ops::{Add, Sub};

use super::MagneticVariation;

mod constants {
    pub const PI2: f32 = std::f32::consts::PI * 2.0;
}

/// An angle in the range from 0° to 360°.
///
/// An angle in degree as [`i16`] or in radians as [`f32`] can be converted into
/// an Angle and it's value will be wrapped into the range from 0° to 360°.
///
/// ```
/// use efb::Angle;
/// let west: Angle = (-90).into();
/// assert_eq!(west.as_degrees(), 270);
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Angle {
    rad: f32,
}

impl From<i16> for Angle {
    /// Converts the angle in degree into an Angle.
    fn from(value: i16) -> Self {
        Self {
            rad: {
                if value.is_negative() {
                    ((360 + (value % 360)) as f32).to_radians()
                } else {
                    ((value % 360) as f32).to_radians()
                }
            },
        }
    }
}

impl From<f32> for Angle {
    /// Converts the angle in radians into an Angle.
    fn from(value: f32) -> Self {
        Self {
            rad: if value.is_sign_negative() {
                constants::PI2 + (value % (constants::PI2))
            } else {
                value % constants::PI2
            },
        }
    }
}

impl Angle {
    /// Returns the inner value in degree.
    pub fn as_degrees(&self) -> u16 {
        self.rad.to_degrees().round() as u16
    }

    /// Returns the inner value in radians.
    pub fn as_radians(&self) -> f32 {
        self.rad
    }
}

impl Add for Angle {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        (self.rad + other.rad).into()
    }
}

impl Add<i16> for Angle {
    type Output = Self;

    fn add(self, other: i16) -> Self {
        (self.as_degrees() as i16 + other).into()
    }
}

impl Sub for Angle {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            rad: self.rad - other.rad,
        }
    }
}

impl Sub<i16> for Angle {
    type Output = Self;

    fn sub(self, other: i16) -> Self {
        (self.as_degrees() as i16 - other).into()
    }
}

impl Add<MagneticVariation> for Angle {
    type Output = Self;

    /// Magnetic variations to the east are subtracted while variations to the
    /// west are add to the angle.
    fn add(self, other: MagneticVariation) -> Self {
        let other_deg: f32 = match other {
            MagneticVariation::East(v) => -v,
            MagneticVariation::West(v) => v,
            MagneticVariation::OrientedToTrueNorth => 0.0,
        };

        (self.rad + other_deg.to_radians()).into()
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{0:03}°", self.as_degrees())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        let north: Angle = 0.into();
        assert_eq!(north, 360.into());

        let west: Angle = (-90).into();
        assert_eq!(west, 270.into());

        let north: Angle = 0.0.into();
        assert_eq!(north, 360.into());

        let south: Angle = std::f32::consts::PI.into();
        assert_eq!(south, 180.into());
    }

    #[test]
    fn add_sub() {
        let north: Angle = 0.into();
        let east: Angle = 90.into();
        let west: Angle = 270.into();

        assert_eq!(east - 90, north);
        assert_eq!(west + 180, east);
        assert_eq!(north - 90, west);
        assert_eq!(west + 180, east);
    }
}
