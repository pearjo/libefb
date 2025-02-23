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

use super::Performance;
use crate::aircraft::Aircraft;
use crate::route::Route;
use crate::{Duration, Fuel, VerticalDistance};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
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
#[derive(Clone, Debug)]
pub enum FuelPolicy {
    MinimumFuel,
    MaximumFuel,
    ManualFuel(Fuel),
    FuelAtLanding(Fuel),
    ExtraFuel(Fuel),
}

#[derive(Clone, Debug)]
pub struct FuelPlanning {
    taxi: Fuel,
    climb: Option<Fuel>,
    trip: Fuel,
    alternate: Option<Fuel>,
    reserve: Fuel,
    total: Fuel,
    min: Fuel,
    extra: Option<Fuel>,
    after_landing: Fuel,
}

impl FuelPlanning {
    pub fn new(
        aircraft: &Aircraft,
        policy: &FuelPolicy,
        taxi: Fuel,
        route: &Route,
        reserve: &Reserve,
        perf: &Performance,
    ) -> Option<Self> {
        let climb = None; // TODO add climb fuel
        let trip = route.fuel(perf)?;
        let alternate = route.alternate().and_then(|alternate| alternate.fuel(perf));
        let reserve = reserve.fuel(perf, &route.level()?);

        let min = {
            let mut min = taxi + trip + reserve;

            if let Some(climb) = climb {
                min = min + climb;
            }

            if let Some(alternate) = alternate {
                min = min + alternate;
            }

            min
        };

        let extra = {
            match policy {
                FuelPolicy::MinimumFuel => None,
                FuelPolicy::MaximumFuel => {
                    aircraft.usable_fuel().map(|usable_fuel| usable_fuel - min)
                }
                FuelPolicy::ManualFuel(fuel) => Some(*fuel - min),
                FuelPolicy::FuelAtLanding(fuel) => Some(*fuel), // TODO is this correct?
                FuelPolicy::ExtraFuel(fuel) => Some(*fuel),
            }
        };

        let total = {
            match extra {
                Some(extra) => min + extra,
                None => min,
            }
        };

        let after_landing = total - taxi - trip;

        Some(Self {
            taxi,
            climb,
            trip,
            alternate,
            reserve,
            total,
            min,
            extra,
            after_landing,
        })
    }

    pub fn taxi(&self) -> &Fuel {
        &self.taxi
    }

    pub fn climb(&self) -> Option<&Fuel> {
        self.climb.as_ref()
    }

    pub fn trip(&self) -> &Fuel {
        &self.trip
    }

    pub fn alternate(&self) -> Option<&Fuel> {
        self.alternate.as_ref()
    }

    pub fn reserve(&self) -> &Fuel {
        &self.reserve
    }

    pub fn total(&self) -> &Fuel {
        &self.total
    }

    pub fn min(&self) -> &Fuel {
        &self.min
    }

    pub fn extra(&self) -> Option<&Fuel> {
        self.extra.as_ref()
    }

    pub fn on_ramp(&self) -> &Fuel {
        &self.total
    }

    pub fn after_landing(&self) -> &Fuel {
        &self.after_landing
    }
}
