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

use std::cell::Ref;

use super::{Aircraft, Performance, Route};
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
pub enum FuelPolicy<'a> {
    MinimumFuel,
    MaximumFuel(&'a Aircraft),
    Manual(Fuel),
    Landing(Fuel),
    Extra(Fuel),
}

#[derive(Copy, Clone)]
pub struct FuelPlanning<'a> {
    policy: FuelPolicy<'a>,
    pub taxi: Fuel,
    pub climb: Option<Fuel>,
    pub trip: Fuel,
    pub alternate: Option<Fuel>,
    pub reserve: Fuel,
}

impl<'a> FuelPlanning<'a> {
    pub fn new<P>(
        policy: FuelPolicy<'a>,
        taxi: Fuel,
        route: Ref<'a, Route>,
        reserve: &Reserve,
        perf: &P,
    ) -> Option<Self>
    where
        P: Performance,
    {
        Some(Self {
            policy,
            taxi,
            climb: None,
            trip: route.fuel(perf)?,
            alternate: route.alternate().map(|alternate| alternate.fuel(perf))?,
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
        match self.policy {
            FuelPolicy::MinimumFuel => None,
            FuelPolicy::MaximumFuel(ac) => match ac.usable_fuel() {
                Some(usable_fuel) => Some(usable_fuel - self.min()),
                None => None,
            },
            FuelPolicy::Manual(fuel) => Some(fuel - self.min()),
            FuelPolicy::Landing(fuel) => Some(fuel), // TODO is this correct?
            FuelPolicy::Extra(fuel) => Some(fuel),
        }
    }

    pub fn on_ramp(&self) -> Fuel {
        self.total()
    }

    pub fn after_landing(&self) -> Fuel {
        self.total() - self.taxi - self.trip
    }
}
