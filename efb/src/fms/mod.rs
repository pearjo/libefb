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
use crate::fp::Route;
use crate::nd::NavigationData;

#[repr(C)]
pub struct FMS {
    nd: NavigationData,
    route: Option<Route>,
}

impl FMS {
    /// Constructs a new, empty `FMS`.
    pub fn new() -> Self {
        Self {
            nd: NavigationData::new(),
            route: None,
        }
    }

    pub fn nd(&mut self) -> &mut NavigationData {
        &mut self.nd
    }

    pub fn route(&self) -> Option<&Route> {
        self.route.as_ref()
    }

    pub fn decode(&mut self, route: &str) -> Result<(), Error> {
        let route = Route::decode(route, &self.nd)?;
        self.route = Some(route);

        Ok(())
    }
}
