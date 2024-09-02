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

use super::{Parser, ParserError};
use crate::nd::*;
use std::str::FromStr;

mod from;

pub struct Arinc424Parser;

impl Parser for Arinc424Parser {
    fn parse(s: &str) -> Result<NavigationData, ParserError> {
        let airspaces = Airspaces::new();
        let mut waypoints = Waypoints::new();

        // TODO add some nice error handling
        s.lines().for_each(|line| match &line[4..6] {
            "EA" | "PC" => {
                if let Ok(waypoint_record) = arinc424::Waypoint::from_str(line) {
                    waypoints.push(waypoint_record.into());
                }
            }
            _ => {}
        });

        Ok(NavigationData {
            airspaces,
            waypoints,
        })
    }
}
