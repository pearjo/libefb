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

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::VerticalDistance;
use crate::geom::Coordinate;

use super::*;

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Airport {
    pub icao_ident: String,
    pub iata_designator: String,
    pub name: String,
    pub coordinate: Coordinate,
    pub mag_var: MagneticVariation,
    pub elevation: VerticalDistance,
    pub runways: Vec<Runway>,
    pub cycle: AiracCycle,
}

impl Fix for Airport {
    fn ident(&self) -> String {
        self.icao_ident.clone()
    }

    fn coordinate(&self) -> Coordinate {
        self.coordinate
    }
}
