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

use std::fmt;
use std::fs;
use std::path::Path;
use std::rc::Rc;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::geom::Coordinate;
use crate::MagneticVariation;

mod airac_cycle;
mod airport;
mod airspace;
mod fix;
mod parser;
mod runway;
mod waypoint;

pub use airac_cycle::AiracCycle;
pub use airport::Airport;
pub use airspace::{Airspace, AirspaceClass, Airspaces};
pub use fix::Fix;
use parser::*;
pub use runway::*;
pub use waypoint::*;

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub enum NavAid {
    Airport(Rc<Airport>),
    Waypoint(Rc<Waypoint>),
}

impl Fix for NavAid {
    fn ident(&self) -> String {
        match self {
            Self::Airport(aprt) => aprt.ident(),
            Self::Waypoint(wp) => wp.ident(),
        }
    }

    fn coordinate(&self) -> Coordinate {
        match self {
            Self::Airport(aprt) => aprt.coordinate(),
            Self::Waypoint(wp) => wp.coordinate(),
        }
    }

    fn mag_var(&self) -> MagneticVariation {
        match self {
            Self::Airport(aprt) => aprt.mag_var(),
            Self::Waypoint(wp) => wp.mag_var(),
        }
    }
}

impl fmt::Display for NavAid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ident())
    }
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum InputFormat {
    Arinc424,
    OpenAir,
}

#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NavigationData {
    pub airports: Vec<Rc<Airport>>,
    pub airspaces: Airspaces,
    pub waypoints: Vec<Rc<Waypoint>>,
}

impl NavigationData {
    pub fn new() -> Self {
        Self {
            airports: Vec::new(),
            airspaces: Vec::new(),
            waypoints: Vec::new(),
        }
    }

    pub fn at(&self, point: &Coordinate) -> Vec<&Airspace> {
        self.airspaces
            .iter()
            .filter(|airspace| airspace.polygon.contains(point))
            .collect()
    }

    pub fn find(&self, ident: &str) -> Option<NavAid> {
        self.waypoints
            .iter()
            .find(|&wp| wp.ident() == ident)
            .map(|wp| NavAid::Waypoint(Rc::clone(wp)))
            .or(self
                .airports
                .iter()
                .find(|&aprt| aprt.ident() == ident)
                .map(|aprt| NavAid::Airport(Rc::clone(aprt))))
    }

    pub fn read(&mut self, s: &str, fmt: InputFormat) -> Result<(), Error> {
        match fmt {
            InputFormat::Arinc424 => {
                let mut record = s.parse::<Arinc424Record>()?;
                self.airports.append(&mut record.airports);
                self.waypoints.append(&mut record.waypoints);
            }
            InputFormat::OpenAir => {
                let mut record = s.parse::<OpenAirRecord>()?;
                self.airspaces.append(&mut record.airspaces);
            }
        };

        Ok(())
    }

    pub fn read_file(&mut self, path: &Path, fmt: InputFormat) -> Result<(), Error> {
        match fs::read_to_string(path) {
            Ok(string) => self.read(&string, fmt),
            // Err(err) => Err(match err.kind() {
            //     ErrorKind::NotFound => ParserError::NotFound,
            //     ErrorKind::PermissionDenied => ParserError::PermissionDenied,
            //     _ => ParserError::FileNotRead,
            // }),
            Err(_) => Err(Error::UnexpectedString),
        }
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
            waypoints: Vec::new(),
        };

        assert_eq!(nd.at(&inside), vec![&nd.airspaces[0]]);
        assert!(nd.at(&outside).is_empty());
    }
}
