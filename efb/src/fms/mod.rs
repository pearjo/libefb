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
use crate::error::Error;
use crate::nd::NavigationData;
use crate::route::Route;

mod subs;

pub use subs::*;

pub struct FMS {
    nd: NavigationData,
    route: Route,
    flight_planning_builder: FlightPlanningBuilder,
}

impl FMS {
    /// Constructs a new, empty `FMS`.
    pub fn new() -> Self {
        let route = Route::default();
        let flight_planning_builder = FlightPlanningBuilder::default();

        Self {
            nd: NavigationData::new(),
            route,
            flight_planning_builder,
        }
    }

    pub fn nd(&mut self) -> &mut NavigationData {
        &mut self.nd
    }

    pub fn route(&mut self) -> &mut Route {
        &mut self.route
    }

    pub fn decode(&mut self, route: &str) -> Result<(), Error> {
        self.route.decode(route, &self.nd)?;
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
                self.route.set_alternate(Some(alternate));
                Ok(())
            }
            None => Err(Error::UnknownIdent),
        }
    }

    pub fn flight_planning_builder(&mut self) -> &mut FlightPlanningBuilder {
        &mut self.flight_planning_builder
    }

    pub fn flight_planning(&self) -> Result<FlightPlanning, Error> {
        self.flight_planning_builder.build(&self.route)
    }
}

impl Default for FMS {
    fn default() -> Self {
        Self::new()
    }
}
