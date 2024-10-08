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

use crate::{FuelFlow, Speed, VerticalDistance};

/// Used to provide [Speed] or [FuelFlow] for a defined performance setting at
/// different vertical distances.
pub trait Performance {
    /// Returns the true airspeed at a vertical distance.
    fn tas(&self, vd: VerticalDistance) -> Speed;

    /// Returns the fuel flow at a vertical distance.
    fn ff(&self, vd: VerticalDistance) -> FuelFlow;
}
