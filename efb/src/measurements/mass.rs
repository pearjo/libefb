// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Joe Pearson
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
pub enum MassUnit {
    Kilograms,
    Pounds,
}

impl UnitOfMeasure for MassUnit {
    fn symbol(&self) -> &str {
        match self {
            Self::Kilograms => "kg",
            Self::Pounds => "lb",
        }
    }

    fn from_si(value: f64, to: &Self) -> f64 {
        match to {
            Self::Kilograms => value,
            Self::Pounds => unimplemented!(),
        }
    }

    fn to_si(&self, value: f64) -> f64 {
        match self {
            Self::Kilograms => value,
            Self::Pounds => unimplemented!(),
        }
    }
}

pub type Mass = Measurement<MassUnit>;

pub fn kg(value: f64) -> Mass {
    Mass {
        value,
        unit: MassUnit::Kilograms,
    }
}

pub fn lb(value: f64) -> Mass {
    Mass {
        value,
        unit: MassUnit::Pounds,
    }
}
