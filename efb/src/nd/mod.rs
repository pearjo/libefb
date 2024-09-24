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

//! Navigation Data.

use crate::geom::Coordinate;
use crate::MagneticVariation;

mod airac_cycle;
mod airport;
mod airspace;
mod waypoint;

pub use airac_cycle::AiracCycle;
pub use airport::Airport;
pub use airspace::{Airspace, AirspaceClass, Airspaces};
pub use waypoint::{Region, Waypoint, WaypointUsage, Waypoints};

/// A fix location with coordinates.
#[repr(C)]
#[derive(Clone, Copy)]
pub enum Fix<'a> {
    Airport(&'a Airport),
    Waypoint(&'a Waypoint),
}

impl Fix<'_> {
    pub fn ident(&self) -> String {
        match self {
            Self::Airport(aprt) => aprt.icao_ident.clone(),
            Self::Waypoint(wp) => wp.fix_ident.clone(),
        }
    }

    pub fn coordinate(&self) -> Coordinate {
        match self {
            Self::Airport(aprt) => aprt.coordinate,
            Self::Waypoint(wp) => wp.coordinate,
        }
    }

    pub fn var(&self) -> MagneticVariation {
        match self {
            Self::Airport(aprt) => aprt.mag_var,
            Self::Waypoint(wp) => wp.mag_var,
        }
    }
}

#[derive(Default)]
pub struct NavigationData {
    pub airports: Vec<Airport>,
    pub airspaces: Airspaces,
    pub waypoints: Waypoints,
}

impl NavigationData {
    pub fn at(&self, point: &Coordinate) -> Vec<&Airspace> {
        self.airspaces
            .iter()
            .filter(|airspace| airspace.polygon.contains(point))
            .collect()
    }

    pub fn find<'a>(&self, ident: &str) -> Option<Fix> {
        self.waypoints
            .iter()
            .find(|&wp| wp.route_ident() == ident)
            .map(|wp| Fix::Waypoint(wp))
            .or(
                self.airports
                    .iter()
                    .find(|&aprt| aprt.route_ident() == ident)
                    .map(|aprt| Fix::Airport(aprt))
            )
    }
}

#[cfg(test)]
mod tests {
    use crate::geom::Polygon;
    use crate::VerticalDistance;

    use super::*;

    #[test]
    fn airspace_at_point() {
        let inside = coord!(53.03759, 9.00533);
        let outside = coord!(53.04892, 8.90907);

        let nd = NavigationData {
            airspaces: vec![Airspace {
                name: String::from("TMA BREMEN A"),
                class: AirspaceClass::D,
                ceiling: VerticalDistance::Fl(65),
                floor: VerticalDistance::Msl(1500),
                polygon: polygon![
                    (53.10111, 8.974999),
                    (53.102776, 9.079166),
                    (52.97028, 9.084444),
                    (52.96889, 8.982222),
                    (53.10111, 8.974999)
                ],
            }],
            airports: Vec::new(),
            waypoints: Waypoints::new(),
        };

        assert_eq!(nd.at(&inside), vec![&nd.airspaces[0]]);
        assert!(nd.at(&outside).is_empty());
    }
}
