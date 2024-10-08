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
use std::ops::{Add, Mul};

use super::{Density, Duration, Mass, Volume};

mod constants {
    use super::Density;

    pub const DIESEL_AT_ISA: Density = Density::KilogramPerLiter(0.838);
    pub const JET_A_AT_ISA: Density = Density::KilogramPerLiter(0.8);
}

pub enum Fuel {
    Diesel(Volume),
    JetA(Volume),
}

impl Fuel {
    pub fn mass(self) -> Mass {
        match self {
            Self::Diesel(v) => v * constants::DIESEL_AT_ISA,
            Self::JetA(v) => v * constants::JET_A_AT_ISA,
        }
    }
}

impl Display for Fuel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let v = match self {
            Self::Diesel(v) => v,
            Self::JetA(v) => v,
        };

        let tmp = if let Some(precision) = f.precision() {
            format!("{v:.precision$}")
        } else {
            format!("{v}")
        };

        f.pad_integral(true, "", &tmp)
    }
}

impl Add for Fuel {
    type Output = Fuel;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Diesel(fuel) => match rhs {
                Self::Diesel(fuel_rhs) => Self::Diesel(fuel + fuel_rhs),
                _ => Self::Diesel(fuel),
            },
            Self::JetA(fuel) => match rhs {
                Self::JetA(fuel_rhs) => Self::JetA(fuel + fuel_rhs),
                _ => Self::JetA(fuel),
            },
        }
    }
}

impl Mul<f32> for Fuel {
    type Output = Fuel;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Self::Diesel(fuel) => Self::Diesel(fuel * rhs),
            Self::JetA(fuel) => Self::JetA(fuel * rhs),
        }
    }
}

pub enum FuelFlow {
    PerHour(Fuel),
}

impl Mul<Duration> for FuelFlow {
    type Output = Fuel;

    fn mul(self, rhs: Duration) -> Self::Output {
        let hours: f32 = u32::from(rhs) as f32 / 3600.0;

        match self {
            Self::PerHour(fuel) => fuel * hours,
        }
    }
}
