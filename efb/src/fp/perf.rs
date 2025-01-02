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

use std::collections::HashMap;

use crate::{FuelFlow, Speed, VerticalDistance};

/// Used to provide [Speed] or [FuelFlow] for a defined performance setting at
/// different level.
pub struct Performance {
    map: HashMap<VerticalDistance, (Speed, FuelFlow)>,
}

impl Performance {
    /// Creates the performance profile from a function.
    ///
    /// The function `f` is called in 1000 ft intervals up to the ceiling.
    pub fn from<F>(f: F, ceiling: VerticalDistance) -> Self
    where
        F: Fn(&VerticalDistance) -> (Speed, FuelFlow),
    {
        let mut map: HashMap<VerticalDistance, (Speed, FuelFlow)> = HashMap::new();
        let mut vd = VerticalDistance::Gnd;
        let mut alt = 0;

        while vd <= ceiling {
            let (tas, ff) = f(&vd);
            map.insert(vd, (tas, ff));

            alt += 1000;
            vd = VerticalDistance::Altitude(alt);
        }

        Self { map }
    }

    /// Returns the true airspeed at a level.
    pub fn tas(&self, level: &VerticalDistance) -> Speed {
        self.at_level(level).0
    }

    /// Returns the fuel flow at a level.
    pub fn ff(&self, level: &VerticalDistance) -> FuelFlow {
        self.at_level(level).1
    }

    /// Returns the speed and fuel flow at a level.
    ///
    /// # Panics
    ///
    /// Panics if the map holds no performance value which should never happen.
    fn at_level(&self, level: &VerticalDistance) -> &(Speed, FuelFlow) {
        self.map
            .keys()
            .reduce(|nearest, key| {
                if level >= key {
                    key.max(nearest)
                } else {
                    key.min(nearest)
                }
            })
            .and_then(|key| self.map.get(key))
            .expect("There should be at least one performance value.")
    }
}
