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

use efb::aircraft::CGLimit;
use efb::{Distance, Mass};

/// Returns the limit's mass.
#[no_mangle]
pub extern "C" fn efb_cg_limit_mass(limit: &CGLimit) -> &Mass {
    &limit.mass
}

#[no_mangle]
pub extern "C" fn efb_cg_limit_set_mass(limit: &mut CGLimit, mass: Mass) {
    limit.mass = mass
}

/// Returns the limit's distance in reference to the aircraft's datum.
#[no_mangle]
pub extern "C" fn efb_cg_limit_distance(limit: &CGLimit) -> &Distance {
    &limit.distance
}

#[no_mangle]
pub extern "C" fn efb_cg_limit_set_distance(limit: &mut CGLimit, distance: Distance) {
    limit.distance = distance
}
