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

use super::{Density, Unit, Volume};

use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Mass {
    Kilogram(f32),
}

impl Unit for Mass {
    fn si(&self) -> f32 {
        match self {
            Self::Kilogram(value) => *value,
        }
    }

    fn from_si(value: f32) -> Self {
        Self::Kilogram(value)
    }
}

impl fmt::Display for Mass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (value, unit) = match self {
            Self::Kilogram(value) => (value, "kg"),
        };

        let tmp = if let Some(precision) = f.precision() {
            format!("{:.precision$} {}", value, unit)
        } else {
            format!("{} {}", value, unit)
        };

        f.pad_integral(true, "", &tmp)
    }
}

impl From<f32> for Mass {
    fn from(value: f32) -> Self {
        Self::from_si(value)
    }
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

impl Div<f32> for Mass {
    type Output = Mass;

    fn div(self, rhs: f32) -> Self::Output {
        match self {
            Self::Kilogram(m) => Self::Kilogram(m / rhs),
        }
    }
}
