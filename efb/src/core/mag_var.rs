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

use crate::geom::Coordinate;
use std::fmt::{Display, Formatter, Result};
use time::{Date, Month, OffsetDateTime, Time};
use wmm::declination;

/// The magnetic variation (declination) of a point.
///
/// Any [Coordinate] can be converted into a declination.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MagneticVariation {
    /// The declination is towards the east.
    East(f32),
    /// The declination is towards the west.
    West(f32),
    /// The point is oriented to true north.
    OrientedToTrueNorth,
}

impl From<Coordinate> for MagneticVariation {
    fn from(value: Coordinate) -> Self {
        // TODO Use a new WMM library which has the 2025 model implemented!
        let date = OffsetDateTime::new_utc(
            Date::from_calendar_date(2024, Month::December, 31).unwrap(),
            Time::from_hms_nano(12, 59, 59, 500_000_000).unwrap(),
        )
        .date();

        // TODO write an own implementation of the WMM that returns the last
        // known declination, even if the coefficients are outdated.
        let mag_var = declination(date, value.latitude, value.longitude).unwrap();

        if mag_var.is_sign_negative() {
            Self::West(mag_var.abs())
        } else {
            Self::East(mag_var)
        }
    }
}

impl Display for MagneticVariation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::East(value) => write!(f, "{:.1}° E", value),
            Self::West(value) => write!(f, "{:.1}° W", value),
            Self::OrientedToTrueNorth => write!(f, "T"),
        }
    }
}
