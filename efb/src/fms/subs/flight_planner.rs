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
pub struct FlightPlannerBuilder {
    aircraft: Option<Aircraft>,
    mass: Option<Vec<Mass>>,
    policy: Option<FuelPolicy>,
    taxi: Option<Fuel>,
    reserve: Option<Reserve>,
    perf: Option<Performance>,
}

impl FlightPlannerBuilder {
    pub fn new() -> FlightPlannerBuilder {
        Self {
            aircraft: None,
            mass: None,
            policy: None,
            taxi: None,
            reserve: None,
            perf: None,
        }
    }

    pub fn aircraft(&mut self, aircraft: Aircraft) -> &mut FlightPlannerBuilder {
        self.aircraft = Some(aircraft);
        self
    }

    pub fn mass(&mut self, mass: Vec<Mass>) -> &mut FlightPlannerBuilder {
        self.mass = Some(mass);
        self
    }

    pub fn policy(&mut self, policy: FuelPolicy) -> &mut FlightPlannerBuilder {
        self.policy = Some(policy);
        self
    }

    pub fn taxi(&mut self, taxi: Fuel) -> &mut FlightPlannerBuilder {
        self.taxi = Some(taxi);
        self
    }

    pub fn reserve(&mut self, reserve: Reserve) -> &mut FlightPlannerBuilder {
        self.reserve = Some(reserve);
        self
    }

    pub fn perf(&mut self, perf: Performance) -> &mut FlightPlannerBuilder {
        self.perf = Some(perf);
        self
    }
}

impl SubSystemBuilder for FlightPlannerBuilder {
    type SubSystem = FlightPlanner;

    fn build(&self, route: &Route) -> Result<FlightPlanner, Error> {
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

        Ok(FlightPlanner {
            aircraft: self.aircraft.clone(),
            fuel_planning,
            mb,
        })
    }
}

pub struct FlightPlanner {
    aircraft: Option<Aircraft>,
    fuel_planning: Option<FuelPlanning>,
    mb: Option<MassAndBalance>,
}

impl FlightPlanner {
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

impl SubSystem for FlightPlanner {
    type Builder = FlightPlannerBuilder;
}
