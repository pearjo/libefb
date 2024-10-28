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

use super::{Angle, Speed};

/// The wind with a speed and direction
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Wind {
    /// The direction from which the wind comes.
    pub direction: Angle,
    /// The wind speed.
    pub speed: Speed,
}

impl FromStr for Wind {
    type Err = Error;

    /// Parses a string `s` to return Wind.
    ///
    /// The string is formatted according to the wind usage of a METAR
    /// e.g. `23008KT` for wind from 230° with a speed of 8 Knots.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction: Option<i16> = s.get(0..3).and_then(|s| s.parse().ok());
        let speed: Option<f32> = s.get(3..5).and_then(|s| s.parse().ok());
        let unit: &str = s.get(5..s.len()).unwrap_or_default();

        match (direction, speed, unit) {
            (Some(direction), Some(speed), "KT") => Ok(Wind {
                direction: Angle::from(direction),
                speed: Speed::Knots(speed),
            }),
            (Some(direction), Some(speed), "MPS") => Ok(Wind {
                direction: Angle::from(direction),
                speed: Speed::MeterPerSecond(speed),
            }),
            _ => Err(Error::UnexpectedString),
        }
    }
}

impl fmt::Display for Wind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0}/{1}", self.direction, self.speed.to_kt())
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
                direction: Angle::from(330),
                speed: Speed::Knots(8.0),
            }),
        );
        assert_eq!(
            "33004MPS".parse::<Wind>(),
            Ok(Wind {
                direction: Angle::from(330),
                speed: Speed::MeterPerSecond(4.0),
            }),
        );
        assert_eq!("330".parse::<Wind>(), Err(Error::UnexpectedString));
    }
}
