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

use crate::airspace::{Airspace, Airspaces};
use crate::geometry::{point_in_polygon, Coordinate};
use crate::nd::{Airspace, Airspaces, Waypoint, Waypoints, NavigationData};

pub struct FMS {
    airspaces: Airspaces,
}

impl FMS {
    /// Returns all [Airspace] at the `point`.
    pub fn at(&self, point: &Coordinate) -> Vec<&Airspace> {
        self.airspaces
            .iter()
            .filter(|airspace| point_in_polygon(point, &airspace.polygon))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::coord;
    use crate::geometry::VerticalDistance;
    use crate::nd::*;
    use crate::polygon;

    use super::*;

    #[test]
    fn airspace_at_point() {
        let inside = coord!(53.03759, 9.00533);
        let outside = coord!(53.04892, 8.90907);

        let fms = FMS {
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
        };

        assert_eq!(fms.at(&inside), vec![&fms.airspaces[0]]);
        assert!(fms.at(&outside).is_empty());
    }
}
