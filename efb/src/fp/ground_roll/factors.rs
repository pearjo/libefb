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

use std::collections::HashMap;

use crate::measurements::{Length, Mass, Speed};

use super::GroundRollInfluences;

// TODO: Find a better name
pub enum GroundRollPenalty<T> {
    Range(HashMap<T, f32>),

    /// Percent per value.
    Rate(f32, T),
}

/// A factor to in- or decrease the ground roll on takeoff or landing.
///
/// The factor is applied to the ground roll depending on the ground roll
/// influences.
pub enum GroundRollFactor {
    DecreaseHeadwind(GroundRollPenalty<Speed>),
    IncreaseTailwind(GroundRollPenalty<Speed>),
    IncreaseDryGrass,
    IncreaseWetGrass,
    IncreaseRunwayConditionCode,
    RunwaySlope(GroundRollPenalty<f32>),
    Mass(GroundRollPenalty<Mass>),
}

impl GroundRollFactor {
    /// Returns the penalty due to the ground roll factors and influences.
    pub fn apply(&self, ground_roll: &Length, influences: &GroundRollInfluences) -> Length {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::VerticalDistance;

    use super::*;

    #[test]
    fn penalty() {
        let range = GroundRollPenalty::<VerticalDistance>::Range(HashMap::from([
            (VerticalDistance::PressureAltitude(1000), 0.1),
            (VerticalDistance::PressureAltitude(3000), 0.13),
            (VerticalDistance::Unlimited, 0.18),
        ]));
    }
}
