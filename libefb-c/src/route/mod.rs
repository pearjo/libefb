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

use super::EfbArray;
use efb::route::{Route, Leg};

/// The [Route] to fly.
///
/// This type is a wrapper around the [Route] with an initial cruise speed and
/// level and all legs along the route.
pub struct EfbRoute {
    inner: Box<EfbRouteInner>,
}

#[repr(C)]
struct EfbRouteInner {
    route: Rc<RefCell<Route>>,
}

impl EfbRoute {
    /// Returns an array of pointer to the legs.
    ///
    /// # Safety
    ///
    /// It's up to the caller to free the allocated memory of the array by
    /// calling [efb_route_legs_free].
    ///
    #[no_mangle]
    pub unsafe extern "C" fn efb_route_legs_new(route: &EfbRoute) -> EfbArray<*const Leg> {
        route
            .inner
            .route
            .borrow()
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
}

impl From<Rc<RefCell<Route>>> for EfbRoute {
    fn from(route: Rc<RefCell<Route>>) -> Self {
        Self {
            inner: Box::new(EfbRouteInner {
                route: Rc::clone(&route),
            }),
        }
    }
}
