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

mod aircraft;
mod fuel_planning;
mod legs;
mod mb;
mod perf;
mod route;

pub use aircraft::*;
pub use fuel_planning::*;
pub use legs::Leg;
pub use mb::MassAndBalance;
pub use perf::Performance;
pub use route::{Route, RouteElement};

use crate::Fuel;

pub struct FlightPlanning {
    aircraft: Option<Aircraft>,
    route: Rc<RefCell<Route>>,
    fuel_planning: Option<FuelPlanning>,
}

impl FlightPlanning {
    pub fn new(route: Rc<RefCell<Route>>) -> Self {
        Self {
            aircraft: None,
            route,
            fuel_planning: None,
        }
    }

    pub fn aircraft(&self) -> Option<&Aircraft> {
        self.aircraft.as_ref()
    }

    pub fn set_aircraft(&mut self, aircraft: Aircraft) {
        self.aircraft = Some(aircraft);
    }

    pub fn create_fuel_planning<P>(
        &mut self,
        policy: FuelPolicy,
        taxi: Fuel,
        reserve: &Reserve,
        perf: &P)
    where
        P: Performance, {
        self.fuel_planning = FuelPlanning::new(
            policy,
            taxi,
            self.route.borrow(),
            reserve,
            perf
        );
    }

    pub fn fuel_planning(&self) -> Option<&FuelPlanning> {
        self.fuel_planning.as_ref()
    }
}
