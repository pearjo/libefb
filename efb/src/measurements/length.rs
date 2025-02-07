// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Joe Pearson
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

use super::{Measurement, UnitOfMeasure};

mod constants {
    pub const NAUTICAL_MILE_IN_METER: f64 = 1852.0;
    pub const INCH_IN_METER: f64 = 0.0254;
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum LengthUnit {
    Meters,
    NauticalMiles,
    Inches,
}

impl UnitOfMeasure for LengthUnit {
    fn symbol(&self) -> &str {
        match self {
            Self::Meters => "m",
            Self::NauticalMiles => "NM",
            Self::Inches => "in",
        }
    }

    fn from_si(value: f64, to: &Self) -> f64 {
        match to {
            Self::Meters => value,
            Self::NauticalMiles => value / constants::NAUTICAL_MILE_IN_METER,
            Self::Inches => value / constants::INCH_IN_METER,
        }
    }

    fn to_si(&self, value: f64) -> f64 {
        match self {
            Self::Meters => value,
            Self::NauticalMiles => value * constants::NAUTICAL_MILE_IN_METER,
            Self::Inches => value * constants::INCH_IN_METER,
        }
    }
}

pub type Length = Measurement<LengthUnit>;

impl Length {
    pub fn m(value: f64) -> Self {
        Self {
            value,
            unit: LengthUnit::Meters,
        }
    }

    pub fn nm(value: f64) -> Self {
        Self {
            value,
            unit: LengthUnit::NauticalMiles,
        }
    }

    pub fn inch(value: f64) -> Self {
        Self {
            value,
            unit: LengthUnit::Inches,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_shows_symbol() {
        let m = Length::m(0.0);
        let nm = Length::nm(0.0);

        assert_eq!(m.symbol(), "m");
        assert_eq!(nm.symbol(), "NM");
    }

    #[test]
    fn length_to_si() {
        let nm = Length::nm(1.0);
        let inch = Length::inch(1.0);
        assert_eq!(nm.to_si(), constants::NAUTICAL_MILE_IN_METER);
        assert_eq!(inch.to_si(), constants::INCH_IN_METER);
    }

    #[test]
    fn convert_length() {
        assert_eq!(
            Length::m(0.0254).convert_to(LengthUnit::Inches),
            Length::inch(1.0)
        );
    }
}
