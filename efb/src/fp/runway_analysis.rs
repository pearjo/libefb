// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Joe Pearson
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

use crate::measurements::{Length, Speed};
use crate::nd::Runway;
use crate::Wind;

use super::TakeoffLandingDistance;

/// Analysis the runway length and direction to wind.
///
/// The analysis provides the headwind and crosswind components in runway
/// direction and the ground roll with distance over a 50 feet obstacle for
/// takeoff and landing.
#[derive(Copy, Clone, Debug)]
pub struct RunwayAnalysis {
    headwind: Speed,
    crosswind: Speed,
    takeoff_ground_roll: Length,
    takeoff_ground_roll_50ft_obstacle: Length,
    landing_ground_roll: Length,
    landing_ground_roll_50ft_obstacle: Length,
}

impl RunwayAnalysis {
    /// Creates a new runway analysis from the wind at the aerodrome and a
    /// penalty that reduces the ground roll.
    pub fn new(runway: Runway, wind: Wind) -> Self {
        unimplemented!()
    }

    /// The headwind component relative to the runway direction.
    pub fn headwind(&self) -> &Speed {
        &self.headwind
    }

    /// The crosswind component relative to the runway direction.
    pub fn crosswind(&self) -> &Speed {
        &self.crosswind
    }

    /// The ground roll available for takeoff after all penalties.
    pub fn takeoff_ground_roll(&self) -> &Length {
        &self.takeoff_ground_roll
    }

    /// The total distance available to clear a 50 feet obstacle on takeoff
    /// after all penalties.
    pub fn takeoff_ground_roll_50ft_obstacle(&self) -> &Length {
        &self.takeoff_ground_roll_50ft_obstacle
    }

    /// The ground roll available for landing after all penalties.
    pub fn landing_ground_roll(&self) -> &Length {
        &self.landing_ground_roll
    }

    /// The total distance available over a 50 feet obstacle for landing after
    /// all penalties.
    pub fn landing_ground_roll_50ft_obstacle(&self) -> &Length {
        &self.landing_ground_roll_50ft_obstacle
    }

    pub fn takeoff_margin(&self) -> &Length {
        unimplemented!()
    }

    pub fn landing_margin(&self) -> &Length {
        unimplemented!()
    }
}
