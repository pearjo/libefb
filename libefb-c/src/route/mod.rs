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

use super::EfbArray;
use efb::route::{Route, Leg};

/// The [Route] to fly.
///
/// This type is a wrapper around the [Route] with an initial cruise speed and
/// level and all legs along the route.
pub struct EfbRoute<'a> {
    pub(super) inner: &'a mut Route,
}

/// Returns an array of pointer to the legs.
///
/// # Safety
///
/// It's up to the caller to free the allocated memory of the array by
/// calling [efb_route_legs_free].
#[no_mangle]
pub unsafe extern "C" fn efb_route_legs_new(route: &EfbRoute) -> EfbArray<*const Leg> {
    route
        .inner
        .legs()
        .iter()
        .map(|leg| leg as *const Leg)
        .collect::<Vec<*const Leg>>()
        .into()
}

/// Frees the memory of the legs array.
#[no_mangle]
pub extern "C" fn efb_route_legs_free(legs: &mut EfbArray<*const Leg>) {
    legs.into_vec();
}
