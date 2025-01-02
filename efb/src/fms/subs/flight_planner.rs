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
use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::fp::*;
use crate::{Fuel, Mass};

use super::SubSystem;

#[derive(Default)]
pub struct FlightPlannerInput {
    pub aircraft: Option<Aircraft>,
    pub mass: Option<Vec<Mass>>,
    pub policy: Option<FuelPolicy>,
    pub taxi: Option<Fuel>,
    pub reserve: Option<Reserve>,
    pub perf: Option<Performance>,
}

pub struct FlightPlanner {
    route: Rc<RefCell<Route>>,
    input: FlightPlannerInput,
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
        match (self.input.aircraft.as_ref(), self.mb.as_ref()) {
            (Some(ac), Some(mb)) => Some(ac.is_balanced(mb)),
            _ => None,
        }
    }

    fn update(&mut self) -> Result<(), Error> {
        self.update_fuel_planning()?;
        self.update_mb()?;
        Ok(())
    }

    fn update_fuel_planning(&mut self) -> Result<(), Error> {
        match (
            &self.input.policy,
            self.input.taxi,
            self.input.reserve,
            &self.input.perf,
        ) {
            (Some(policy), Some(taxi), Some(reserve), Some(perf)) => {
                self.fuel_planning =
                    FuelPlanning::new(policy.clone(), taxi, self.route.borrow(), &reserve, perf);
                Ok(())
            }
            _ => Err(Error::UnexpectedInput),
        }
    }

    fn update_mb(&mut self) -> Result<(), Error> {
        match (&self.input.aircraft, &self.input.mass, &self.fuel_planning) {
            (Some(aircraft), Some(mass), Some(fuel_planning)) => {
                self.mb = Some(aircraft.mb_from_const_mass_and_equally_distributed_fuel(
                    mass,
                    &fuel_planning.on_ramp(),
                    &fuel_planning.after_landing(),
                )?);
                Ok(())
            }
            (None, _, _) => Err(Error::ExpectedAircraft),
            (_, _, None) => Err(Error::ExpectedFuelPlanning),
            _ => Err(Error::UnexpectedInput),
        }
    }
}

impl SubSystem for FlightPlanner {
    type Input = FlightPlannerInput;

    fn new(route: Rc<RefCell<Route>>) -> Self {
        Self {
            route,
            input: Self::Input::default(),
            fuel_planning: None,
            mb: None,
        }
    }

    fn notify(&mut self) -> Result<(), Error> {
        self.update()
    }

    fn enter(&mut self, input: Self::Input) -> Result<(), Error> {
        self.input = input;
        self.update()
    }
}
