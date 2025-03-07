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

use super::{Field, FieldError, Latitude, Longitude};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum MagVar<const I: usize, const J: usize, const K: usize> {
    /// The variation is east of true north.
    East(u8, u8),
    /// The variation is west of true north.
    West(u8, u8),
    /// The point is oriented to true north.
    OrientedToTrueNorth,
    /// Use the world magnetic model (WMM) if no variation is provided.
    WMM(Latitude<J>, Longitude<K>),
}

impl<const I: usize, const J: usize, const K: usize> Field for MagVar<I, J, K> {}

impl<const I: usize, const J: usize, const K: usize> FromStr for MagVar<I, J, K> {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_column = &s[I..I + 1];

        match first_column {
            "E" | "W" => {
                let degree: u8 = s[I + 1..I + 4]
                    .parse()
                    .map_err(|_| FieldError::NotANumber)?;
                let centidegree: u8 = s[I + 4..I + 5]
                    .parse()
                    .map_err(|_| FieldError::NotANumber)?;

                if first_column == "E" {
                    Ok(Self::East(degree, centidegree))
                } else {
                    Ok(Self::West(degree, centidegree))
                }
            }
            "T" => Ok(Self::OrientedToTrueNorth),
            " " => Ok(Self::WMM(s.parse()?, s.parse()?)), // TODO this is not valid ARINC 424
            _ => Err(FieldError::UnexpectedChar(
                "expected E, W or T as variation direction",
            )),
        }
    }
}
