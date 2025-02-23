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

use efb::aircraft::FuelTank;
use efb::{Distance, Volume};

/// Returns the tanks arm in reference to the aircraft's datum.
#[no_mangle]
pub extern "C" fn efb_fuel_tank_arm(tank: &FuelTank) -> &Distance {
    &tank.arm
}

#[no_mangle]
pub extern "C" fn efb_fuel_tank_set_arm(tank: &mut FuelTank, arm: Distance) {
    tank.arm = arm
}

/// Returns the tanks capacity.
#[no_mangle]
pub extern "C" fn efb_fuel_tank_capacity(tank: &FuelTank) -> &Volume {
    &tank.capacity
}

#[no_mangle]
pub extern "C" fn efb_fuel_tank_set_capacity(tank: &mut FuelTank, capacity: Volume) {
    tank.capacity = capacity
}
