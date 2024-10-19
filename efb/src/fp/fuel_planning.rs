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

use super::{Performance, Route};
use crate::nd::NavAid;
use crate::{Duration, Fuel, VerticalDistance};

#[derive(Copy, Clone)]
pub enum Reserve {
    Manual(Duration),
}

impl Reserve {
    pub fn fuel<P>(self, perf: &P, cruise: &VerticalDistance) -> Fuel
    where
        P: Performance,
    {
        match self {
            Self::Manual(duration) => perf.ff(cruise) * duration,
        }
    }
}

#[derive(Copy, Clone)]
pub enum FuelPolicy {
    MinimumFuel,
    MaximumFuel,
    Manual(Fuel),
    Landing(Fuel),
    Extra(Fuel),
}

#[derive(Copy, Clone)]
pub struct FuelPlanning {
    policy: FuelPolicy,
    pub taxi: Fuel,
    pub climb: Option<Fuel>,
    pub trip: Fuel,
    pub alternate: Option<Fuel>,
    pub reserve: Fuel,
}

impl FuelPlanning {
    pub fn new<P>(
        policy: FuelPolicy,
        taxi: Fuel,
        route: &Route,
        reserve: &Reserve,
        alternate: Option<NavAid>,
        perf: &P,
    ) -> Self
    where
        P: Performance,
    {
        Self {
            policy,
            taxi,
            climb: None,
            trip: route.fuel(perf).unwrap(),
            alternate: match alternate {
                Some(alternate) => Some(route.alternate(alternate).fuel(perf)),
                None => None,
            },
            reserve: reserve.fuel(perf, &VerticalDistance::Gnd), // TODO get cruise altitude once add to route or flight plan
        }
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
        match self.policy {
            FuelPolicy::MinimumFuel => None,
            FuelPolicy::Manual(fuel) => Some(fuel - self.min()),
            FuelPolicy::Extra(fuel) => Some(fuel),
            _ => unimplemented!(), // TODO add once we have an aircraft with total usable fuel, MTOW and PAX in flight plan
        }
    }
}
