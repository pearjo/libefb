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

use super::{Measurement, UnitOfMeasure};

#[repr(C)]
pub enum SpeedUnit {
    MetersPerSecond,
    Knots,
}

impl UnitOfMeasure for SpeedUnit {
    fn symbol(&self) -> &str {
        match self {
            Self::MetersPerSecond => "mps",
            Self::Knots => "kt",
        }
    }

    fn from_si(value: f64, to: &Self) -> f64 {
        match to {
            Self::MetersPerSecond => value,
            Self::Knots => unimplemented!(),
        }
    }

    fn to_si(&self, value: f64) -> f64 {
        match self {
            Self::MetersPerSecond => value,
            Self::Knots => unimplemented!(),
        }
    }
}

pub type Speed = Measurement<SpeedUnit>;
