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

use std::str::FromStr;

use crate::error::Error;
use crate::nd::*;

mod from;

pub struct Arinc424Record {
    pub airports: Vec<Airport>,
    pub waypoints: Vec<Waypoint>,
}

impl FromStr for Arinc424Record {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut airports: Vec<Airport> = Vec::new();
        let mut waypoints = Waypoints::new();

        // TODO add some nice error handling
        s.lines().for_each(|line| match &line[4..6] {
            "EA" | "PC" => {
                if let Ok(waypoint_record) = arinc424::Waypoint::from_str(line) {
                    waypoints.push(waypoint_record.into());
                }
            }
            "P " => match &line[12..13] {
                "A" => {
                    if let Ok(airport_record) = arinc424::Airport::from_str(line) {
                        airports.push(airport_record.into());
                    }
                }
                _ => {}
            },
            _ => {}
        });

        Ok(Self {
            airports,
            waypoints,
        })
    }
}
