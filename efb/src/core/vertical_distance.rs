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

use core::f32;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::fmt;
use std::ops::Div;
use std::str::FromStr;

use crate::error::Error;
use crate::measurements::Pressure;

mod constants {
    use crate::measurements::Pressure;

    pub const METER_IN_FEET: f32 = 3.28084;
    pub const STD_PRESSURE: Pressure = Pressure::h_pa(1013.23); // 29.92 inHg
}

/// A vertical distance.
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum VerticalDistance {
    /// Absolute Altitude as distance above ground level in feet.
    Agl(u16),

    /// Altitude in feet with reference to a local air pressure.
    Altitude(u16), // TODO does it make sense to have ALT?

    /// Pressure altitude in feet.
    PressureAltitude(i16),

    /// Flight level in hundreds of feet as altitude at standard air pressure.
    Fl(u16),

    /// Ground level.
    Gnd,

    /// True Altitude as distance above mean sea level.
    Msl(u16),

    /// An unlimited vertical distance.
    Unlimited,
}

impl VerticalDistance {
    /// Returns the pressure altitude based on the elevation and the QNH.
    pub fn pa(elevation: &i16, qnh: &Pressure) -> Self {
        // https://www.weather.gov/media/epz/wxcalc/pressureAltitude.pdf
        Self::PressureAltitude(
            elevation
                + (145366.45 * (1.0 - (*qnh / constants::STD_PRESSURE).powf(0.190284)).round())
                    as i16,
        )
    }
}

impl FromStr for VerticalDistance {
    type Err = Error;

    /// Parses a string `s` to return a VerticalDistance.
    ///
    /// The string should be according to ICAO Doc. 4444 Annex 2:
    /// - Flight level, expressed as F followed by 3 figures e.g. `F085`
    /// - Standard metric level in tens of metres, expressed by S followed by 4
    ///   figures e.g. `S1130`
    /// - Altitude in hundreds of feet, expressed as A followed by 3 figures
    ///   e.g. `A045`
    /// - Altitude in tens of metres, expressed as M followed by 4 figures e.g.
    ///   `M0840`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        macro_rules! value {
            ($s:expr, $index:expr) => {
                $s.get($index)
                    .and_then(|s| s.parse::<u16>().ok())
                    .ok_or(Error::UnexpectedString)
            };
        }

        match s.get(0..1).unwrap_or_default() {
            // first character is the unit
            "F" => Ok(Self::Fl(value!(s, 1..4)?)),
            "S" => Ok(Self::Fl(
                // value in tens of meter or hundreds of feet
                (value!(s, 1..5)? as f32 * constants::METER_IN_FEET / 10.0).round() as u16,
            )),
            "A" => Ok(Self::Altitude(value!(s, 1..4)? * 100)), // value in hundredth of feet
            "M" => Ok(Self::Altitude(
                // value in tens of meter
                (value!(s, 1..5)? as f32 * constants::METER_IN_FEET).round() as u16,
            )),
            _ => Err(Error::UnexpectedString),
        }
    }
}

impl fmt::Display for VerticalDistance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VerticalDistance::Gnd => write!(f, "GND"),
            VerticalDistance::Fl(value) => write!(f, "FL{value}"),
            VerticalDistance::Agl(value) => write!(f, "{value} AGL"),
            VerticalDistance::Msl(value) => write!(f, "{value} MSL"),
            VerticalDistance::Altitude(value) => write!(f, "{value} ALT"),
            VerticalDistance::PressureAltitude(value) => write!(f, "PA {value}"),
            VerticalDistance::Unlimited => write!(f, "unlimited"),
        }
    }
}

/// # Panics
///
/// Explain why and when we panic...
impl Ord for VerticalDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // ground is always less
            (Self::Gnd, Self::Gnd) => Ordering::Equal,
            (Self::Gnd, _) => Ordering::Less,
            (_, Self::Gnd) => Ordering::Greater,

            // and unlimited is always greater
            (Self::Unlimited, Self::Unlimited) => Ordering::Equal,
            (Self::Unlimited, _) => Ordering::Greater,
            (_, Self::Unlimited) => Ordering::Less,

            // now compare what can only be compared to the same type
            (Self::Agl(v), Self::Agl(o)) => v.cmp(o),
            (Self::PressureAltitude(v), Self::PressureAltitude(o)) => v.cmp(o),

            _ => {
                fn to_msl(vd: &VerticalDistance) -> u16 {
                    match vd {
                        VerticalDistance::Fl(v) => v * 100,
                        VerticalDistance::Msl(v) => *v,
                        VerticalDistance::Altitude(v) => *v,
                        _ => panic!(
                            "We can't compare {} here, since it doesn't reference to common datum.",
                            vd
                        ),
                    }
                }

                to_msl(self).cmp(&to_msl(other))
            }
        }
    }
}

impl PartialOrd for VerticalDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Div for VerticalDistance {
    type Output = f32;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Gnd, Self::Gnd) => 1.0,
            (Self::Fl(a), Self::Fl(b)) => (a / b).into(),
            (Self::Agl(a), Self::Agl(b)) => (a / b).into(),
            (Self::Msl(a), Self::Msl(b)) => (a / b).into(),
            (Self::Altitude(a), Self::Altitude(b)) => (a / b).into(),
            (Self::PressureAltitude(a), Self::PressureAltitude(b)) => (a / b).into(),
            (Self::Unlimited, Self::Unlimited) => 1.0,
            _ => unimplemented!(
                "Division of vertical distances of different types is not yet supported!"
            ),
        }
    }
}

impl From<VerticalDistance> for f32 {
    fn from(value: VerticalDistance) -> Self {
        match value {
            VerticalDistance::Gnd => 0.0,
            VerticalDistance::Fl(value) => value.into(),
            VerticalDistance::Agl(value) => value.into(),
            VerticalDistance::Msl(value) => value.into(),
            VerticalDistance::Altitude(value) => value.into(),
            VerticalDistance::PressureAltitude(value) => value.into(),
            VerticalDistance::Unlimited => f32::INFINITY,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_distance_from_str() {
        assert_eq!(
            "F085".parse::<VerticalDistance>(),
            Ok(VerticalDistance::Fl(85))
        );
        assert_eq!(
            "S1130".parse::<VerticalDistance>(),
            Ok(VerticalDistance::Fl(371))
        );
        assert_eq!(
            "A025".parse::<VerticalDistance>(),
            Ok(VerticalDistance::Altitude(2500))
        );
        assert_eq!(
            "M0762".parse::<VerticalDistance>(),
            Ok(VerticalDistance::Altitude(2500))
        );
        assert_eq!(
            "F08".parse::<VerticalDistance>(),
            Err(Error::UnexpectedString)
        );
    }

    #[test]
    fn gnd_is_least() {
        assert!(VerticalDistance::Gnd < VerticalDistance::Agl(1000));
        assert!(VerticalDistance::Gnd < VerticalDistance::Altitude(1000));
        assert!(VerticalDistance::Gnd < VerticalDistance::Fl(10));
        assert!(VerticalDistance::Gnd == VerticalDistance::Gnd);
        assert!(VerticalDistance::Gnd < VerticalDistance::Msl(100));
        assert!(VerticalDistance::Gnd < VerticalDistance::Unlimited);
    }

    #[test]
    fn unlimited_is_greatest() {
        assert!(VerticalDistance::Unlimited > VerticalDistance::Agl(1000));
        assert!(VerticalDistance::Unlimited > VerticalDistance::Altitude(1000));
        assert!(VerticalDistance::Unlimited > VerticalDistance::Fl(10));
        assert!(VerticalDistance::Unlimited > VerticalDistance::Gnd);
        assert!(VerticalDistance::Unlimited > VerticalDistance::Msl(100));
        assert!(VerticalDistance::Unlimited == VerticalDistance::Unlimited);
    }

    #[test]
    fn cmp_vertical_distances() {
        assert!(VerticalDistance::Agl(1000) < VerticalDistance::Agl(2000));
        assert!(VerticalDistance::Altitude(1000) < VerticalDistance::Altitude(2000));
        assert!(VerticalDistance::Msl(1000) < VerticalDistance::Fl(100));
    }
}
