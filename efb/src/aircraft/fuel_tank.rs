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

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::measurements::{Length, Volume};

/// An aircraft's fuel tank.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FuelTank {
    capacity: Volume,
    arm: Length,
}

impl FuelTank {
    pub fn new(capacity: Volume, arm: Length) -> Self {
        Self { capacity, arm }
    }

    /// The tank's capacity.
    pub fn capacity(&self) -> &Volume {
        &self.capacity
    }

    /// The distance of the tank to the aircraft's reference datum.
    pub fn arm(&self) -> &Length {
        &self.arm
    }
}
