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

use std::fmt;
use std::str::FromStr;

use crate::error::Error;
use crate::measurements::{Angle, Speed, SpeedUnit};

/// The wind with a speed and direction.
///
/// The wind can be split into headwind (or tailwind) and crosswind components
/// for a direction. This provides e.g. information of the crosswind component
/// on landing.
///
/// # Examples
///
/// ```
/// # use std::str::FromStr;
/// # use efb::error::Error;
/// # use efb::measurements::{Angle, Speed};
/// # use efb::Wind;
/// #
/// # fn main() -> Result<(), Error> {
/// // the wind as reported from our destinations METAR
/// // blowing from the south
/// let wind = Wind::from_str("00010KT")?;
///
/// // we land on runway 09 pointing to the east so we have full 10 knots
/// // crosswind from the right
/// assert_eq!(wind.crosswind(&Angle::t(90.0)), Speed::kt(-10.0));
/// #     Ok(())
/// # }
/// ```
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Wind {
    /// The direction from which the wind comes.
    pub direction: Angle,
    /// The wind speed.
    pub speed: Speed,
}

impl Wind {
    /// Returns the headwind relative to the direction.
    ///
    /// A negative value indicates a tailwind.
    pub fn headwind(&self, direction: &Angle) -> Speed {
        let relative_wind = self.direction - *direction;
        // TODO: Deal better with headwind or tailwind.
        self.speed * -relative_wind.to_si().cos()
    }

    /// Returns the cross wind relative to the direction.
    ///
    /// A negative value indicates that the wind comes from the left side
    /// relative to the direction.
    pub fn crosswind(&self, direction: &Angle) -> Speed {
        let relative_wind = self.direction - *direction;
        self.speed * relative_wind.to_si().sin()
    }
}

impl FromStr for Wind {
    type Err = Error;

    /// Parses a string `s` to return Wind.
    ///
    /// The string is formatted according to the wind usage of a METAR
    /// e.g. `23008KT` for wind from 230° with a speed of 8 Knots.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction: Option<f32> = s.get(0..3).and_then(|s| s.parse().ok());
        let speed: Option<f32> = s.get(3..5).and_then(|s| s.parse().ok());
        let unit: &str = s.get(5..s.len()).unwrap_or_default();

        match (direction, speed, unit) {
            (Some(direction), Some(speed), "KT") => Ok(Wind {
                direction: Angle::t(direction),
                speed: Speed::kt(speed),
            }),
            (Some(direction), Some(speed), "MPS") => Ok(Wind {
                direction: Angle::t(direction),
                speed: Speed::mps(speed),
            }),
            _ => Err(Error::UnexpectedString),
        }
    }
}

impl fmt::Display for Wind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{0}/{1}",
            self.direction,
            self.speed.convert_to(SpeedUnit::Knots),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(
            "33008KT".parse::<Wind>(),
            Ok(Wind {
                direction: Angle::t(330.0),
                speed: Speed::kt(8.0),
            }),
        );
        assert_eq!(
            "33004MPS".parse::<Wind>(),
            Ok(Wind {
                direction: Angle::t(330.0),
                speed: Speed::mps(4.0),
            }),
        );
        assert_eq!("330".parse::<Wind>(), Err(Error::UnexpectedString));
    }

    #[test]
    fn full_headwind() {
        let wind = Wind {
            direction: Angle::t(0.0),
            speed: Speed::kt(10.0),
        };

        assert_eq!(wind.headwind(&Angle::t(180.0)), Speed::kt(10.0));
    }

    #[test]
    fn full_tailwind() {
        let wind = Wind {
            direction: Angle::t(0.0),
            speed: Speed::kt(10.0),
        };

        assert_eq!(wind.headwind(&Angle::t(0.0)), Speed::kt(-10.0));
    }

    #[test]
    fn full_crosswind_left() {
        let wind = Wind {
            direction: Angle::t(0.0),
            speed: Speed::kt(10.0),
        };

        assert_eq!(wind.crosswind(&Angle::t(90.0)), Speed::kt(-10.0));
    }

    #[test]
    fn full_crosswind_right() {
        let wind = Wind {
            direction: Angle::t(0.0),
            speed: Speed::kt(10.0),
        };

        assert_eq!(wind.crosswind(&Angle::t(270.0)), Speed::kt(10.0));
    }
}
