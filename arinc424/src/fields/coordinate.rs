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

use super::{Field, FieldError};
use std::ops::Range;
use std::str::FromStr;

fn parse_numeric_field(
    s: &str,
    idx: usize,
    len: usize,
    range: Range<u8>,
) -> Result<u8, FieldError> {
    s[idx..idx + len]
        .parse()
        .map_err(|_| FieldError::NotANumber)
        .and_then(|v| {
            range
                .contains(&v)
                .then_some(v)
                .ok_or(FieldError::NumberOutOfRange)
        })
}

#[derive(Debug, PartialEq)]
pub enum CardinalDirection {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq)]
pub struct Latitude<const I: usize> {
    pub cardinal: CardinalDirection,
    pub degree: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub centiseconds: u8,
}

impl<const I: usize> Field for Latitude<I> {}

impl<const I: usize> FromStr for Latitude<I> {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cardinal = match &s[I..I + 1] {
            "N" => Ok(CardinalDirection::North),
            "S" => Ok(CardinalDirection::South),
            _ => Err(FieldError::UnexpectedChar(
                "expected N or S cardinal direction",
            )),
        }?;

        let degree = parse_numeric_field(s, I + 1, 2, 0..90)?;
        let minutes = parse_numeric_field(s, I + 3, 2, 0..60)?;
        let seconds = parse_numeric_field(s, I + 5, 2, 0..60)?;
        let centiseconds = parse_numeric_field(s, I + 7, 2, 0..99)?;

        if degree == 90 && (minutes > 0 || seconds > 0 || centiseconds > 0) {
            Err(FieldError::NumberOutOfRange)
        } else {
            Ok(Self {
                cardinal,
                degree,
                minutes,
                seconds,
                centiseconds,
            })
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Longitude<const I: usize> {
    pub cardinal: CardinalDirection,
    pub degree: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub centiseconds: u8,
}

impl<const I: usize> Field for Longitude<I> {}

impl<const I: usize> FromStr for Longitude<I> {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cardinal = match &s[I..I + 1] {
            "W" => Ok(CardinalDirection::West),
            "E" => Ok(CardinalDirection::East),
            _ => Err(FieldError::UnexpectedChar(
                "expected E or W cardinal direction",
            )),
        }?;

        let degree = parse_numeric_field(s, I + 1, 3, 0..181)?;
        let minutes = parse_numeric_field(s, I + 4, 2, 0..61)?;
        let seconds = parse_numeric_field(s, I + 6, 2, 0..61)?;
        let centiseconds = parse_numeric_field(s, I + 8, 2, 0..100)?;

        if degree == 180 && (minutes > 0 || seconds > 0 || centiseconds > 0) {
            Err(FieldError::NumberOutOfRange)
        } else {
            Ok(Self {
                cardinal,
                degree,
                minutes,
                seconds,
                centiseconds,
            })
        }
    }
}
