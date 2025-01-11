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

use efb::route::{Leg, Route};

/// The [Route] to fly.
///
/// This type is a wrapper around the [Route] with an initial cruise speed,
/// level and all legs along the route.
///
/// The [`efb_route_legs_first`] and [`efb_route_legs_next`] functions return a
/// leg of the route and can be used to iterate over the route:
///
/// ```
/// for (const EfbLeg *leg = efb_route_legs_first(route);
///      leg != NULL;
///      leg = efb_route_legs_next(route))
/// ```
pub struct EfbRoute<'a> {
    inner: &'a mut Route,
    legs: Option<Legs<'a>>,
}

impl<'a> From<&'a mut Route> for EfbRoute<'a> {
    fn from(route: &'a mut Route) -> Self {
        Self {
            inner: route,
            legs: None,
        }
    }
}

struct Legs<'a> {
    route: &'a Route,
    count: usize,
}

impl<'a> Legs<'a> {
    fn new(route: &'a Route) -> Legs<'a> {
        Self { route, count: 0 }
    }
}

impl<'a> Iterator for Legs<'a> {
    type Item = &'a Leg;

    fn next(&mut self) -> Option<Self::Item> {
        match self.route.legs().get(self.count) {
            Some(leg) => {
                self.count += 1;
                Some(leg)
            }
            None => None,
        }
    }
}

/// Returns the first leg in the route.
#[no_mangle]
pub extern "C" fn efb_route_legs_first<'a>(route: &'a mut EfbRoute<'a>) -> Option<&'a Leg> {
    route.legs.insert(Legs::new(route.inner)).next()
}

/// Returns the next leg in the route.
///
/// When the end of the legs is reached, this function returns a null pointer.
#[no_mangle]
pub extern "C" fn efb_route_legs_next<'a>(route: &'a mut EfbRoute) -> Option<&'a Leg> {
    route.legs.as_mut().and_then(|legs| legs.next())
}
