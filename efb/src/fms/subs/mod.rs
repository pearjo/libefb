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

use crate::error::Error;
use crate::route::Route;

mod flight_planner;

pub use flight_planner::*;

/// A trait to implement a type as FMS sub-system.
///
/// A sub-system implements an encapsulated part of the FMS e.g. a flight
/// planner. It is created by a builder that holds optional values that are
/// needed to create the system and a reference to the Route during creation.
pub trait SubSystem {
    /// The associated input that is entered into this type.
    type Builder: SubSystemBuilder;

    /// Returns the builder to create the sub-system.
    fn builder() -> Self::Builder {
        Self::Builder::default()
    }
}

pub trait SubSystemBuilder: Default {
    type SubSystem: SubSystem;

    fn build(&self, route: &Route) -> Result<Self::SubSystem, Error>;
}
