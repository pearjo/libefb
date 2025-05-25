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

use std::str::FromStr;

use super::{Field, FieldError};

#[derive(Debug, PartialEq)]
pub enum RwyBrg<const I: usize> {
    MagneticNorth(f32),
    TrueNorth(u32),
}

impl<const I: usize> Field for RwyBrg<I> {}

impl<const I: usize> FromStr for RwyBrg<I> {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[I + 3..I + 4] {
            "T" => {
                let degree = s[I..I + 3]
                    .parse::<u32>()
                    .map_err(|_| FieldError::NotANumber)?;

                Ok(Self::TrueNorth(degree))
            }
            _ => {
                let degree = s[I..I + 3]
                    .parse::<u32>()
                    .map_err(|_| FieldError::NotANumber)?;

                let decimal = s[I + 3..I + 4]
                    .parse::<u32>()
                    .map_err(|_| FieldError::NotANumber)?;

                Ok(Self::MagneticNorth(degree as f32 + decimal as f32 / 10.0))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_true_north() {
        assert_eq!("347T".parse::<RwyBrg<0>>(), Ok(RwyBrg::TrueNorth(347)));
    }

    #[test]
    fn parse_magnetic_north() {
        assert_eq!(
            "2302".parse::<RwyBrg<0>>(),
            Ok(RwyBrg::MagneticNorth(230.2))
        );
    }
}
