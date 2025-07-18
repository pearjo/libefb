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

use crate::VerticalDistance;
use crate::geom::Polygon;

pub type Airspaces = Vec<Airspace>;

/// Airspace.
///
/// The airspace is classified by the `class` and enclosed by the `polygon`.
/// It ranges from the `floor` to `ceiling` in vertical direction.
#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct Airspace {
    pub name: String,
    pub class: AirspaceClass,
    pub ceiling: VerticalDistance,
    pub floor: VerticalDistance,
    pub polygon: Polygon,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AirspaceClass {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    CTR,
    EDR,
    EDD,
    EDP,
    // Restricted,
    // Danger,
    // Prohibited,
}

impl Display for AirspaceClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AirspaceClass::A => write!(f, "Class A"),
            AirspaceClass::B => write!(f, "Class B"),
            AirspaceClass::C => write!(f, "Class C"),
            AirspaceClass::D => write!(f, "Class D"),
            AirspaceClass::E => write!(f, "Class E"),
            AirspaceClass::F => write!(f, "Class F"),
            AirspaceClass::G => write!(f, "Class G"),
            AirspaceClass::CTR => write!(f, "CTR"),
            AirspaceClass::EDR => write!(f, "ED-R"),
            AirspaceClass::EDD => write!(f, "ED-D"),
            AirspaceClass::EDP => write!(f, "ED-P"),
        }
    }
}

impl Display for Airspace {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}: {} | {}/{}",
            self.name, self.class, self.ceiling, self.floor
        )
    }
}
