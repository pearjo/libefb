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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(C)]
pub enum DensityUnit {
    KilogramPerLiter,
}

impl UnitOfMeasure<f32> for DensityUnit {
    fn symbol(&self) -> &'static str {
        match self {
            Self::KilogramPerLiter => "kg/l",
        }
    }

    fn from_si(value: f32, to: &Self) -> f32 {
        match to {
            Self::KilogramPerLiter => value,
        }
    }

    fn to_si(&self, value: &f32) -> f32 {
        match self {
            Self::KilogramPerLiter => *value,
        }
    }
}

pub type Density = Measurement<f32, DensityUnit>;

impl Density {
    pub const fn kg_per_l(value: f32) -> Density {
        Measurement {
            value,
            unit: DensityUnit::KilogramPerLiter,
        }
    }
}
