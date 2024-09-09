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

use crate::geom::{point_in_polygon, Coordinate};

mod airac_cycle;
mod airspace;
mod mag_var;
mod waypoint;

pub use airac_cycle::AiracCycle;
pub use airspace::{Airspace, AirspaceClass, Airspaces};
pub use mag_var::MagneticVariation;
pub use waypoint::{Region, Waypoint, WaypointUsage, Waypoints};

#[repr(C)]
pub enum Fix<'a> {
    Waypoint(&'a Waypoint),
}

impl Fix<'_> {
    fn coordinate(&self) -> Coordinate {
        match self {
            Self::Waypoint(wp) => wp.coordinate,
        }
    }
}

#[derive(Default)]
pub struct NavigationData {
    pub airspaces: Airspaces,
    pub waypoints: Waypoints,
}

impl NavigationData {
    pub fn at(&self, point: &Coordinate) -> Vec<&Airspace> {
        self.airspaces
            .iter()
            .filter(|airspace| point_in_polygon(point, &airspace.polygon))
            .collect()
    }

    pub fn find<'a>(&self, ident: &str) -> Option<Fix> {
        self.waypoints
            .iter()
            .find(|&wp| wp.route_ident() == ident)
            .map(|wp| Fix::Waypoint(wp))
    }
}

#[cfg(test)]
mod tests {
    use crate::coord;
    use crate::geom::VerticalDistance;
    use crate::polygon;

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
            waypoints: Waypoints::new(),
        };

        assert_eq!(nd.at(&inside), vec![&nd.airspaces[0]]);
        assert!(nd.at(&outside).is_empty());
    }
}
