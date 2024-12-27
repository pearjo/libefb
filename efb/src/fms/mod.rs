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
use std::rc::Rc;
use std::cell::{Ref, RefCell};

use crate::error::Error;
use crate::fp::Route;
use crate::nd::NavigationData;

#[repr(C)]
pub struct FMS {
    nd: NavigationData,
    route: Rc<RefCell<Route>>,
}

impl FMS {
    /// Constructs a new, empty `FMS`.
    pub fn new() -> Self {
        let route = Rc::new(RefCell::new(Route::new()));
        Self {
            nd: NavigationData::new(),
            route,
        }
    }

    pub fn nd(&mut self) -> &mut NavigationData {
        &mut self.nd
    }

    pub fn route(&self) -> Ref<'_, Route> {
        self.route.borrow()
    }

    pub fn decode(&mut self, route: &str) -> Result<(), Error> {
        self.route.borrow_mut().decode(route, &self.nd)?;
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
            },
            None => Err(Error::UnknownIdent)
        }
    }
}
