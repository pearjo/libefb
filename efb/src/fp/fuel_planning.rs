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

use super::{Aircraft, Performance};
use crate::route::Route;
use crate::{Duration, Fuel, VerticalDistance};

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Reserve {
    Manual(Duration),
}

impl Reserve {
    pub fn fuel(self, perf: &Performance, cruise: &VerticalDistance) -> Fuel {
        match self {
            Self::Manual(duration) => perf.ff(cruise) * duration,
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub enum FuelPolicy {
    MinimumFuel,
    MaximumFuel,
    Manual(Fuel),
    Landing(Fuel),
    Extra(Fuel),
}

#[derive(Clone)]
pub struct FuelPlanning {
    pub taxi: Fuel,
    pub climb: Option<Fuel>,
    pub trip: Fuel,
    pub alternate: Option<Fuel>,
    pub reserve: Fuel,

    policy: FuelPolicy,
    usable_fuel: Option<Fuel>,
}

impl FuelPlanning {
    pub fn new(
        aircraft: &Aircraft,
        policy: FuelPolicy,
        taxi: Fuel,
        route: &Route,
        reserve: &Reserve,
        perf: &Performance,
    ) -> Option<Self> {
        Some(Self {
            policy,
            usable_fuel: aircraft.usable_fuel(),
            taxi,
            climb: None, // TODO add climb fuel
            trip: route.fuel(perf)?,
            alternate: {
                match route.alternate() {
                    Some(alternate) => alternate.fuel(perf),
                    _ => None,
                }
            },
            reserve: reserve.fuel(perf, &route.level()?),
        })
    }

    pub fn total(&self) -> Fuel {
        match self.extra() {
            Some(extra) => self.min() + extra,
            None => self.min(),
        }
    }

    pub fn min(&self) -> Fuel {
        let mut min = self.taxi + self.trip + self.reserve;

        if let Some(climb) = self.climb {
            min = min + climb;
        }

        if let Some(alternate) = self.alternate {
            min = min + alternate;
        }

        min
    }

    pub fn extra(&self) -> Option<Fuel> {
        match &self.policy {
            FuelPolicy::MinimumFuel => None,
            FuelPolicy::MaximumFuel => self.usable_fuel.map(|usable_fuel| usable_fuel - self.min()),
            FuelPolicy::Manual(fuel) => Some(*fuel - self.min()),
            FuelPolicy::Landing(fuel) => Some(*fuel), // TODO is this correct?
            FuelPolicy::Extra(fuel) => Some(*fuel),
        }
    }

    pub fn on_ramp(&self) -> Fuel {
        self.total()
    }

    pub fn after_landing(&self) -> Fuel {
        self.total() - self.taxi - self.trip
    }
}
