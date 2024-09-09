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

use crate::fc;
use crate::geom::Coordinate;
use crate::nd::*;

use arinc424;

impl From<arinc424::Cycle> for AiracCycle {
    fn from(value: arinc424::Cycle) -> Self {
        AiracCycle {
            year: value.year,
            month: value.month,
        }
    }
}

impl<const I: usize, const J: usize> From<(arinc424::Latitude<I>, arinc424::Longitude<J>)>
    for Coordinate
{
    fn from(value: (arinc424::Latitude<I>, arinc424::Longitude<J>)) -> Self {
        let lat = fc::dms_to_decimal(value.0.degree, value.0.minutes, value.0.seconds);
        let long = fc::dms_to_decimal(value.1.degree, value.1.minutes, value.1.seconds);

        Coordinate {
            latitude: if value.0.cardinal == arinc424::CardinalDirection::North {
                lat
            } else {
                -lat
            },
            longitude: if value.1.cardinal == arinc424::CardinalDirection::East {
                long
            } else {
                -long
            },
        }
    }
}

impl<const I: usize> From<arinc424::MagVar<I>> for MagneticVariation {
    fn from(value: arinc424::MagVar<I>) -> Self {
        match value {
            arinc424::MagVar::East(d, cd) => Self::East(d as f32 + 100.0 / cd as f32),
            arinc424::MagVar::West(d, cd) => Self::West(d as f32 + 100.0 / cd as f32),
            arinc424::MagVar::OrientedToTrueNorth => Self::OrientedToTrueNorth,
            _ => Self::Unknown,
        }
    }
}

impl<const I: usize> From<arinc424::RegnCode<I>> for Region {
    fn from(value: arinc424::RegnCode<I>) -> Self {
        if value == "ENRT" {
            Self::Enroute
        } else {
            Self::TerminalArea(value.into_inner())
        }
    }
}

impl From<arinc424::Waypoint> for Waypoint {
    fn from(wp: arinc424::Waypoint) -> Waypoint {
        Waypoint {
            fix_ident: wp.fix_ident.to_string(),
            desc: wp.name_desc.to_string(),
            // TODO change type to enum and add matching
            usage: if wp.waypoint_type == "V  " {
                WaypointUsage::VFROnly
            } else {
                WaypointUsage::Unknown
            },
            coordinate: (wp.latitude, wp.longitude).into(),
            region: wp.regn_code.into(),
            mag_var: wp.mag_var.into(),
            cycle: wp.cycle.into(),
        }
    }
}
