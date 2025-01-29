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

mod leg;

pub use leg::Leg;

use crate::error::Error;
use crate::fp::Performance;
use crate::nd::*;
use crate::{Distance, Duration, Fuel, Speed, VerticalDistance, Wind};

#[derive(Debug)]
pub enum RouteElement {
    Speed(Speed),
    Level(VerticalDistance),
    Wind(Wind),
    NavAid(NavAid),
}

/// A route that goes from an origin to a destination.
///
/// The route is composed of legs where each [`leg`] describes path between two
/// [`fixes`].
///
/// # Decoding
///
/// The route can be decoded from a space separated list of fixes, wind values
/// and performance elements. The route elements
///
/// ```text
/// 135@09 107KT EDDH DHD HLW EDHL
/// ```
///
/// would create a route from Hamburg to Luebeck via outbound delta routing and
/// inbound whisky routing with a desired TAS of 107kt and a wind of 9kt from
/// south-east. Performance elements can be add at any point but latest before
/// the first leg is defined (we have from and to fix).
///
/// Thus, each leg is computed based on the latest performance elements defined
/// on the route. Extending our route to
///
/// ```text
/// 13509KT N0107 EDDH DHD 18009KT HLW EDHL
/// ```
///
/// we would have wind from south-east (135°) on the leg from EDDH to DHD, but
/// the wind would turn to south (180°) for the remaining legs.
///
/// [`leg`]: crate::fp::legs::Leg
/// [`fixes`]: crate::nd::Fix
#[derive(Debug)]
pub struct Route {
    elements: Vec<RouteElement>,
    legs: Vec<Leg>,
    speed: Option<Speed>,
    level: Option<VerticalDistance>,
    alternate: Option<NavAid>,
}

impl Route {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            legs: Vec::new(),
            speed: None,
            level: None,
            alternate: None,
        }
    }

    /// Decodes a `route` that is composed of a space separated list of fix
    /// idents read from the navigation data `nd`.
    pub fn decode(&mut self, route: &str, nd: &NavigationData) -> Result<(), Error> {
        let mut elements: Vec<RouteElement> = Vec::new();

        // TODO level and speed needs to be properly update from decoder
        for element in route.split_whitespace() {
            if let Some(navaid) = nd.find(element) {
                elements.push(RouteElement::NavAid(navaid));
            } else if let Ok(value) = element.parse::<VerticalDistance>() {
                self.level.get_or_insert(value);
                elements.push(RouteElement::Level(value));
            } else if let Ok(value) = element.parse::<Speed>() {
                self.speed.get_or_insert(value);
                elements.push(RouteElement::Speed(value));
            } else if let Ok(value) = element.parse::<Wind>() {
                elements.push(RouteElement::Wind(value));
            } else {
                return Err(Error::UnexpectedRouteElement);
            }
        }

        self.elements = elements;
        self.legs = Self::legs_from_elements(&self.elements);

        Ok(())
    }

    pub fn insert(&mut self, index: usize, element: RouteElement) {
        self.elements.insert(index, element);
        self.legs = Self::legs_from_elements(&self.elements);
    }

    pub fn push(&mut self, element: RouteElement) {
        self.elements.push(element);
        self.legs = Self::legs_from_elements(&self.elements);
    }

    pub fn elements(&self) -> &Vec<RouteElement> {
        &self.elements
    }

    /// Returns the legs of the route.
    pub fn legs(&self) -> &Vec<Leg> {
        &self.legs
    }

    /// Sets the cruise speed and level.
    ///
    /// The cruise speed or level is remove from the route by setting it to
    /// `None`.
    pub fn set_cruise(&mut self, _speed: Option<Speed>, _level: Option<VerticalDistance>) {
        todo!("Add/remove speed and level from the elements")
    }

    pub fn speed(&self) -> Option<Speed> {
        self.speed
    }

    pub fn level(&self) -> Option<VerticalDistance> {
        self.level
    }

    /// Sets an alternate on the route.
    ///
    /// The alternate is remove by setting it to `None`.
    pub fn set_alternate(&mut self, alternate: Option<NavAid>) {
        self.alternate = alternate;
    }

    /// Returns the final leg but going to the alternate.
    pub fn alternate(&self) -> Option<Leg> {
        let final_leg = self.legs[self.legs.len() - 1].clone();
        Some(Leg::new(
            final_leg.from().clone(),
            self.alternate.clone()?,
            final_leg.level().copied(),
            final_leg.tas().copied(),
            final_leg.wind().copied(),
        ))
    }

    /// Returns the fuel consumption en-route for the given performance.
    pub fn fuel(&self, perf: &Performance) -> Option<Fuel> {
        self.legs
            .iter()
            .filter_map(|leg| leg.fuel(perf))
            .reduce(|acc, fuel| acc + fuel)
    }

    /// Returns the total distance of the route.
    pub fn dist(&self) -> Option<Distance> {
        self.legs
            .iter()
            .map(|leg| *leg.dist())
            .reduce(|acc, dist| acc + dist)
            .map(|dist| dist.to_nm())
    }

    /// Returns the estimated time en-route.
    pub fn ete(&self) -> Option<Duration> {
        self.legs
            .iter()
            .filter_map(|leg| leg.ete().cloned())
            .reduce(|acc, ete| acc + ete)
    }

    fn legs_from_elements(elements: &Vec<RouteElement>) -> Vec<Leg> {
        let mut level: Option<VerticalDistance> = None;
        let mut tas: Option<Speed> = None;
        let mut wind: Option<Wind> = None;
        let mut from: Option<NavAid> = None;
        let mut to: Option<NavAid> = None;
        let mut legs: Vec<Leg> = Vec::new();

        for element in elements {
            match element {
                RouteElement::Speed(value) => tas = Some(*value),
                RouteElement::Level(value) => level = Some(*value),
                RouteElement::Wind(value) => wind = Some(*value),
                RouteElement::NavAid(navaid) => {
                    if from.is_none() {
                        from = Some(navaid.clone());
                    } else if to.is_none() {
                        to = Some(navaid.clone());
                    }
                }
            }

            match (from.clone(), to.clone()) {
                (Some(from), Some(to)) => {
                    legs.push(Leg::new(from, to, level, tas, wind));
                }
                _ => continue,
            }

            (from, to) = (to, None);
        }

        legs
    }
}

impl Default for Route {
    fn default() -> Self {
        Self::new()
    }
}
