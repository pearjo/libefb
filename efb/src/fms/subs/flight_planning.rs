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

//! Flight Planning.
use crate::error::Error;
use crate::fp::*;
use crate::route::Route;
use crate::{Fuel, Mass};

use super::{SubSystem, SubSystemBuilder};

#[derive(Default)]
pub struct FlightPlanningBuilder {
    aircraft: Option<Aircraft>,
    mass: Option<Vec<Mass>>,
    policy: Option<FuelPolicy>,
    taxi: Option<Fuel>,
    reserve: Option<Reserve>,
    perf: Option<Performance>,
}

impl FlightPlanningBuilder {
    pub fn new() -> FlightPlanningBuilder {
        Self {
            aircraft: None,
            mass: None,
            policy: None,
            taxi: None,
            reserve: None,
            perf: None,
        }
    }

    pub fn aircraft(&self) -> Option<&Aircraft> {
        self.aircraft.as_ref()
    }

    pub fn set_aircraft(&mut self, aircraft: Aircraft) -> &mut FlightPlanningBuilder {
        self.aircraft = Some(aircraft);
        self
    }

    pub fn mass(&self) -> Option<&Vec<Mass>> {
        self.mass.as_ref()
    }

    pub fn set_mass(&mut self, mass: Vec<Mass>) -> &mut FlightPlanningBuilder {
        self.mass = Some(mass);
        self
    }

    pub fn policy(&self) -> Option<&FuelPolicy> {
        self.policy.as_ref()
    }

    pub fn set_policy(&mut self, policy: FuelPolicy) -> &mut FlightPlanningBuilder {
        self.policy = Some(policy);
        self
    }

    pub fn taxi(&self) -> Option<&Fuel> {
        self.taxi.as_ref()
    }

    pub fn set_taxi(&mut self, taxi: Fuel) -> &mut FlightPlanningBuilder {
        self.taxi = Some(taxi);
        self
    }

    pub fn reserve(&self) -> Option<&Reserve> {
        self.reserve.as_ref()
    }

    pub fn set_reserve(&mut self, reserve: Reserve) -> &mut FlightPlanningBuilder {
        self.reserve = Some(reserve);
        self
    }

    pub fn perf(&self) -> Option<&Performance> {
        self.perf.as_ref()
    }

    pub fn set_perf(&mut self, perf: Performance) -> &mut FlightPlanningBuilder {
        self.perf = Some(perf);
        self
    }
}

impl SubSystemBuilder for FlightPlanningBuilder {
    type SubSystem = FlightPlanning;

    fn build(&self, route: &Route) -> Result<FlightPlanning, Error> {
        let fuel_planning = match (&self.policy, self.taxi, self.reserve, &self.perf) {
            (Some(policy), Some(taxi), Some(reserve), Some(perf)) => {
                FuelPlanning::new(policy.clone(), taxi, route, &reserve, perf)
            }
            _ => None,
        };

        let mb = match (&self.aircraft, &self.mass, &fuel_planning) {
            (Some(aircraft), Some(mass), Some(fuel_planning)) => {
                Some(aircraft.mb_from_const_mass_and_equally_distributed_fuel(
                    mass,
                    &fuel_planning.on_ramp(),
                    &fuel_planning.after_landing(),
                )?)
            }
            _ => None,
        };

        Ok(FlightPlanning {
            aircraft: self.aircraft.clone(),
            fuel_planning,
            mb,
        })
    }
}

pub struct FlightPlanning {
    aircraft: Option<Aircraft>,
    fuel_planning: Option<FuelPlanning>,
    mb: Option<MassAndBalance>,
}

impl FlightPlanning {
    pub fn fuel_planning(&self) -> Option<&FuelPlanning> {
        self.fuel_planning.as_ref()
    }

    pub fn mb(&self) -> Option<&MassAndBalance> {
        self.mb.as_ref()
    }

    pub fn is_balanced(&self) -> Option<bool> {
        match (self.aircraft.as_ref(), self.mb.as_ref()) {
            (Some(ac), Some(mb)) => Some(ac.is_balanced(mb)),
            _ => None,
        }
    }
}

impl SubSystem for FlightPlanning {
    type Builder = FlightPlanningBuilder;
}
