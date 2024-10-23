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

use super::{Density, Volume};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Mass {
    Kilogram(f32),
}

impl Add for Mass {
    type Output = Mass;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Kilogram(m) => match rhs {
                Self::Kilogram(rhs) => Self::Kilogram(m + rhs),
            },
        }
    }
}

impl Sub for Mass {
    type Output = Mass;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Kilogram(m) => match rhs {
                Self::Kilogram(rhs) => Self::Kilogram(m - rhs),
            },
        }
    }
}

impl Mul<f32> for Mass {
    type Output = Mass;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Self::Kilogram(m) => Self::Kilogram(m * rhs),
        }
    }
}

impl Div<Density> for Mass {
    type Output = Volume;

    fn div(self, rhs: Density) -> Self::Output {
        match self {
            Self::Kilogram(m) => match rhs {
                Density::KilogramPerLiter(density) => Volume::Liter(m / density),
            },
        }
    }
}
