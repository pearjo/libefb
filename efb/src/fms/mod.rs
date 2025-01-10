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

//! Flight Management System.
use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::nd::NavigationData;
use crate::route::Route;

mod subs;

pub use subs::*;

#[repr(C)]
pub struct FMS {
    nd: NavigationData,
    route: Rc<RefCell<Route>>,
    flight_planner: FlightPlanner,
}

impl FMS {
    /// Constructs a new, empty `FMS`.
    pub fn new() -> Self {
        let route = Rc::new(RefCell::new(Route::new()));
        let flight_planner = FlightPlanner::new(route.clone());

        Self {
            nd: NavigationData::new(),
            route,
            flight_planner,
        }
    }

    pub fn nd(&mut self) -> &mut NavigationData {
        &mut self.nd
    }

    pub fn route(&self) -> Rc<RefCell<Route>> {
        Rc::clone(&self.route)
    }

    pub fn decode(&mut self, route: &str) -> Result<(), Error> {
        Rc::clone(&self.route).borrow_mut().decode(route, &self.nd)?;
        self.flight_planner.notify()?;
        Ok(())
    }

    /// Sets an alternate on the route.
    ///
    /// Returns an [UnknownIdent] error if no [NavAid] is found for the ident
    /// within the navigation data.
    ///
    /// [UnknownIdent]: Error::UnknownIdent
    /// [NavAid]: crate::nd::NavAid
    pub fn set_alternate(&mut self, ident: &str) -> Result<(), Error> {
        match self.nd.find(ident) {
            Some(alternate) => {
                self.route.borrow_mut().set_alternate(Some(alternate));
                Ok(())
            }
            None => Err(Error::UnknownIdent),
        }
    }

    pub fn flight_planner(&mut self) -> &mut FlightPlanner {
        &mut self.flight_planner
    }
}
