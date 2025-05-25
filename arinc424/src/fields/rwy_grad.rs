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

#[derive(Debug, Default, PartialEq)]
pub struct RwyGrad<const I: usize> {
    pub degree: f32,
}

impl<const I: usize> Field for RwyGrad<I> {}

impl<const I: usize> FromStr for RwyGrad<I> {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sign = match &s[I..I + 1] {
            "+" => Ok(1.0),
            "-" => Ok(-1.0),
            _ => Err(FieldError::UnexpectedChar("expected + or -")),
        }?;

        let degree = {
            let degree = s[I + 1..I + 3]
                .parse::<f32>()
                .map_err(|_| FieldError::NotANumber)?;
            let decimal = s[I + 3..I + 6]
                .parse::<f32>()
                .map_err(|_| FieldError::NotANumber)?;
            Ok(degree + decimal / 1000.0)
        }?;

        Ok(Self {
            degree: degree * sign,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_upwward_gradient() {
        assert_eq!("+10000".parse::<RwyGrad<0>>(), Ok(RwyGrad { degree: 10.0 }));
    }

    #[test]
    fn parse_downwward_gradient() {
        assert_eq!(
            "-00450".parse::<RwyGrad<0>>(),
            Ok(RwyGrad { degree: -0.45 })
        );
    }
}
