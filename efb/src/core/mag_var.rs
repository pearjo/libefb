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
use time::OffsetDateTime;
use wmm::declination;
use crate::geom::Coordinate;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MagneticVariation {
    East(f32),
    West(f32),
    OrientedToTrueNorth,
}

impl From<Coordinate> for MagneticVariation {
    fn from(value: Coordinate) -> Self {
        let date = OffsetDateTime::now_utc().date();
        // TODO this can be improved to not only unwrap
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
