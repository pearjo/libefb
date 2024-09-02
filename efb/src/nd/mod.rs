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

mod airac_cycle;
mod airspace;
mod mag_var;
mod waypoint;

pub use airac_cycle::AiracCycle;
pub use airspace::{Airspace, AirspaceClass, Airspaces};
pub use mag_var::MagneticVariation;
pub use waypoint::{Region, Waypoint, WaypointUsage, Waypoints};

#[derive(Default)]
pub struct NavigationData {
    pub airspaces: Airspaces,
    pub waypoints: Waypoints,
}
