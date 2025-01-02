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

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::fp::Route;

mod flight_planner;

pub use flight_planner::*;

/// A trait to implement a type as FMS sub-system.
///
/// A sub-system implements an encapsulated part of the FMS e.g. a flight
/// planner. It borrows the FMS's route and inputs are entered into it.
pub trait SubSystem {
    /// The associated input that is entered into this type.
    type Input;

    /// Creates a new sub-system.
    fn new(route: Rc<RefCell<Route>>) -> Self;

    /// Notifies this type that the route changed.
    ///
    /// Returns an [`Err`] if something goes wrong while updating the sub-system.
    fn notify(&mut self) -> Result<(), Error>;

    /// Enters a new input for this type.
    ///
    /// Returns an [`Err`] if something goes wrong while updating the sub-system.
    fn enter(&mut self, input: Self::Input) -> Result<(), Error>;
}
