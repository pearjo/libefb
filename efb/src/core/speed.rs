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

#[derive(Copy, Clone)]
pub enum Speed {
    Knots(f32),
    MeterPerSecond(f32),
}

impl Speed {
    pub fn into_inner(self) -> f32 {
        match self {
            Self::Knots(value) => value,
            Self::MeterPerSecond(value) => value,
        }
    }

    pub fn to_kt(self) -> Self {
        match self {
            Self::Knots(_) => self,
            Self::MeterPerSecond(value) => Self::Knots(value * 1.943844),
        }
    }

    pub fn to_ms(self) -> Self {
        match self {
            Self::Knots(value) => Self::MeterPerSecond(value * 0.514444),
            Self::MeterPerSecond(_) => self,
        }
    }
}

impl FromStr for Speed {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 5 && &s[3..5] == "KT" {
            if let Ok(speed) = s[0..3].parse::<f32>() {
                Ok(Speed::Knots(speed))
            } else {
                Err(Error::UnexpectedString)
            }
        } else {
            Err(Error::UnexpectedString)
        }
    }
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Knots(value) => write!(f, "{value:.0} kt"),
            Self::MeterPerSecond(value) => write!(f, "{value:.0} m/s"),
        }
    }
}
