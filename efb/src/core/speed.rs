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

use crate::error::Error;

use std::fmt;
use std::str::FromStr;

/// The speed in either nautical or metrical units.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Speed {
    Knots(f32),
    MeterPerSecond(f32),
    Mach(f32),
}

impl Speed {
    /// Consumes `self`, returning its inner value.
    pub fn into_inner(self) -> f32 {
        match self {
            Self::Knots(value) => value,
            Self::MeterPerSecond(value) => value,
            Self::Mach(value) => value,
        }
    }

    /// Converts `self` into knots.
    pub fn to_kt(self) -> Self {
        match self {
            Self::Knots(_) => self,
            Self::MeterPerSecond(value) => Self::Knots(value * 1.943844),
            _ => unimplemented!(),
        }
    }

    /// Converts `self` into m/s.
    pub fn to_ms(self) -> Self {
        match self {
            Self::Knots(value) => Self::MeterPerSecond(value * 0.514444),
            Self::MeterPerSecond(_) => self,
            _ => unimplemented!(),
        }
    }
}

impl FromStr for Speed {
    type Err = Error;

    /// Parses a string `s` to return Speed.
    ///
    /// The string should be according to ICAO Doc. 4444 Annex 2:
    /// - Kilometers per hour, expressed as K followed by 4 figures e.g. `K0830`
    /// - Knots, expressed as N followed by 4 figures e.g. `N0485`
    /// - Mach, expressed as M followed by 3 figures to the nearest hundredth of
    ///   unit Mach e.g. `M082`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        macro_rules! value {
            ($s:expr, $index:expr) => {
                $s.get($index)
                    .and_then(|s| s.parse::<u16>().ok()) // ensure that no dot is within the digit
                    .map(|value| value as f32)
                    .ok_or(Error::UnexpectedString)
            };
        }

        match s.get(0..1).unwrap_or_default() { // first character is the unit
            "K" => Ok(Speed::MeterPerSecond(value!(s, 1..5)? / 3.6)), // value in km/h
            "N" => Ok(Speed::Knots(value!(s, 1..5)?)),
            "M" => Ok(Speed::Mach(value!(s, 1..4)? / 100.0)), // value in hundredth of unit Mach
            _ => Err(Error::UnexpectedString),
        }
    }
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Knots(value) => write!(f, "{value:.0} kt"),
            Self::MeterPerSecond(value) => write!(f, "{value:.0} m/s"),
            Self::Mach(value) => write!(f, "M{value:.3}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!("K0360".parse::<Speed>(), Ok(Speed::MeterPerSecond(100.0)));
        assert_eq!("N0485".parse::<Speed>(), Ok(Speed::Knots(485.0)));
        assert_eq!("M082".parse::<Speed>(), Ok(Speed::Mach(0.82)));
        assert_eq!("M08".parse::<Speed>(), Err(Error::UnexpectedString));
    }
}
