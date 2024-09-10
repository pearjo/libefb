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

mod constants {
    pub const NAUTICAL_MILE_IN_METER: f32 = 1852.0;
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

#[derive(Debug, PartialEq)]
pub enum Distance {
    Meter(f32),
    NauticalMiles(f32),
}

impl Distance {
    /// Converts to meters.
    pub fn to_m(self) -> Self {
        match self {
            Self::Meter(_) => self,
            Self::NauticalMiles(nm) => Self::Meter(nm * constants::NAUTICAL_MILE_IN_METER),
        }
    }

    /// Converts to nautical miles.
    pub fn to_nm(self) -> Self {
        match self {
            Self::Meter(m) => Self::NauticalMiles(m / constants::NAUTICAL_MILE_IN_METER),
            Self::NauticalMiles(_) => self,
        }
    }
}

impl Display for Distance {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Distance::Meter(value) => write!(f, "{value} m"),
            Distance::NauticalMiles(value) => write!(f, "{value} NM"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance() {
        let nm = Distance::NauticalMiles(1.0);
        assert_eq!(nm.to_m(), Distance::Meter(1852.0));
    }
}
