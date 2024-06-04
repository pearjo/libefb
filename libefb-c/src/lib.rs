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

use libc::*;

use efb::fc::{Wind};
use efb::fp::legs::{Leg};
use efb::geometry::{Angle};

#[no_mangle]
pub extern "C" fn efb_fp_leg(
    tc: i16,
    dist: c_float,
    wind_direction: i16,
    wind_speed: i16,
    var: i16,
    tas: i16,
) -> Leg {
    Leg::new(
        Angle::from_deg(tc),
        dist,
        Wind {
            direction: Angle::from_deg(wind_direction),
            speed: wind_speed,
        },
        Angle::from_deg(var),
        tas,
    )
}
