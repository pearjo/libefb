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
use std::ops::Add;

/// An angle as value between 0° and 360°.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Angle {
    pub deg: i16,
    pub rad: f32,
}

impl Angle {
    pub fn from_deg(deg: i16) -> Self {
        Self {
            deg: deg,
            rad: (deg as f32).to_radians(),
        }
    }

    pub fn from_rad(rad: f32) -> Self {
        Self {
            deg: rad.to_degrees().round() as i16,
            rad: rad,
        }
    }
}

impl Add for Angle {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from_deg((self.deg + other.deg) % 360)
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{0:03}", self.deg)
    }
}
