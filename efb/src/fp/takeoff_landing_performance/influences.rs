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

use crate::measurements::{Mass, Speed, Temperature};
use crate::nd::{Runway, RunwayConditionCode, RunwaySurface};
use crate::{VerticalDistance, Wind};

/// Influences affecting the takeoff or landing performance.
pub struct Influences {
    mass: Mass,
    headwind: Speed,
    temperature: Temperature,
    slope: f32,
    level: VerticalDistance,
    surface: RunwaySurface,
    rwycc: RunwayConditionCode,
}

impl Influences {
    /// Creates new performance influences.
    pub fn new(
        mass: Mass,
        rwy: &Runway,
        wind: &Wind,
        temperature: Temperature,
        rwycc: RunwayConditionCode,
    ) -> Self {
        Self {
            mass,
            headwind: wind.headwind(&rwy.bearing),
            temperature,
            slope: rwy.slope,
            level: rwy.elev,
            surface: rwy.surface,
            rwycc,
        }
    }

    pub fn mass(&self) -> &Mass {
        &self.mass
    }

    pub fn headwind(&self) -> &Speed {
        &self.headwind
    }

    pub fn temperature(&self) -> &Temperature {
        &self.temperature
    }

    pub fn level(&self) -> &VerticalDistance {
        &self.level
    }

    pub fn slope(&self) -> &f32 {
        &self.slope
    }

    pub fn surface(&self) -> &RunwaySurface {
        &self.surface
    }

    pub fn rwycc(&self) -> &RunwayConditionCode {
        &self.rwycc
    }
}
