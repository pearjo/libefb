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
use std::ops::{Add, Div, Mul, Sub};

use super::{Density, Mass};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Volume {
    Liter(f32),
}

impl Display for Volume {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (v, symbol) = match self {
            Self::Liter(v) => (v, "L"),
        };

        if let Some(precision) = f.precision() {
            write!(f, "{v:.precision$} {symbol}")
        } else {
            write!(f, "{v} {symbol}")
        }
    }
}

impl Add for Volume {
    type Output = Volume;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Liter(v) => match rhs {
                Self::Liter(v_rhs) => Self::Liter(v + v_rhs),
            },
        }
    }
}

impl Sub for Volume {
    type Output = Volume;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Liter(v) => match rhs {
                Self::Liter(v_rhs) => Self::Liter(v - v_rhs),
            },
        }
    }
}

impl Mul<f32> for Volume {
    type Output = Volume;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Self::Liter(v) => Self::Liter(v * rhs),
        }
    }
}

impl Mul<Density> for Volume {
    type Output = Mass;

    fn mul(self, rhs: Density) -> Self::Output {
        match self {
            Self::Liter(v) => match rhs {
                Density::KilogramPerLiter(rho) => Mass::Kilogram(v * rho),
            },
        }
    }
}

impl Div<usize> for Volume {
    type Output = Volume;

    fn div(self, rhs: usize) -> Self::Output {
        match self {
            Self::Liter(v) => Self::Liter(v / rhs as f32),
        }
    }
}
