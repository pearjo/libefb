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

//! Parser for the OpenAir airspace and terrain description language.
//!
//! The current implementation parses only the airspace descriptions as defined
//! by the [user guide]. For each command, an [OpenAirElement] is created which
//! is than converted to an [Airspace].
//!
//! [user guide]: http://www.winpilot.com/UsersGuide/UserAirspace.asp

use std::str::FromStr;

use super::{Parser, ParserError};
use crate::fc::dms_to_decimal;
use crate::geometry::{Coordinate, VerticalDistance};
use crate::nd::{Airspace, AirspaceClass, NavigationData};

/// An element representing an airspace.
struct OpenAirElement {
    /// Airspace class.
    ac: Option<String>,

    /// Airspace name.
    an: Option<String>,

    /// Airspace ceiling.
    ah: Option<VerticalDistance>,

    /// Airspace floor.
    al: Option<VerticalDistance>,

    /// Polygon point.
    dp: Vec<Coordinate>,
}

// TODO: Change to FromStr!
impl From<String> for AirspaceClass {
    fn from(ac: String) -> Self {
        match ac.as_str() {
            "A" => AirspaceClass::A,
            "B" => AirspaceClass::B,
            "C" => AirspaceClass::C,
            "D" => AirspaceClass::D,
            "E" => AirspaceClass::E,
            "F" => AirspaceClass::F,
            "G" => AirspaceClass::G,
            "CTR" => AirspaceClass::CTR,
            "R" => AirspaceClass::EDR,
            "Q" => AirspaceClass::EDD,
            "P" => AirspaceClass::EDP,
            &_ => todo!("Unknown airspace class: {ac}"),
        }
    }
}

impl OpenAirElement {
    fn new() -> Self {
        Self {
            ac: None,
            an: None,
            ah: None,
            al: None,
            dp: Vec::new(),
        }
    }
}

impl From<&mut OpenAirElement> for Airspace {
    fn from(element: &mut OpenAirElement) -> Self {
        let mut polygon = element.dp.clone();
        match polygon.first() {
            Some(first) => {
                if first != polygon.last().unwrap() {
                    polygon.push(first.clone());
                }
            }
            None => (),
        }

        Self {
            name: element.an.take().unwrap_or_default(),
            class: element.ac.take().unwrap_or_default().into(),
            ceiling: element.ah.unwrap_or_default(),
            floor: element.al.unwrap_or_default(),
            polygon: polygon,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseOpenAirCoordinateError;

impl FromStr for Coordinate {
    type Err = ParseOpenAirCoordinateError;

    // 37:53:00 N 116:55:30 W
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(&[' ', ':'][..]);

        // parse latitude
        let d = iter.next().and_then(|s| s.parse::<i32>().ok());
        let m = iter.next().and_then(|s| s.parse::<i32>().ok());
        let s = iter.next().and_then(|s| s.parse::<i32>().ok());
        let ns = iter.next();

        let latitude = match (d, m, s, ns) {
            (Some(d), Some(m), Some(s), Some(ns)) => match ns {
                "N" => Some(dms_to_decimal(d, m, s)),
                "S" => Some(-1.0 * dms_to_decimal(d, m, s)),
                _ => None,
            },
            _ => None,
        };

        // parse longitude
        let d = iter.next().and_then(|s| s.parse::<i32>().ok());
        let m = iter.next().and_then(|s| s.parse::<i32>().ok());
        let s = iter.next().and_then(|s| s.parse::<i32>().ok());
        let ew = iter.next();

        let longitude = match (d, m, s, ew) {
            (Some(d), Some(m), Some(s), Some(ew)) => match ew {
                "E" => Some(dms_to_decimal(d, m, s)),
                "W" => Some(-1.0 * dms_to_decimal(d, m, s)),
                _ => None,
            },
            _ => None,
        };

        match (latitude, longitude) {
            (Some(latitude), Some(longitude)) => Ok(Coordinate {
                latitude,
                longitude,
            }),
            _ => Err(ParseOpenAirCoordinateError),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseOpenAirVerticalDistanceError;

impl FromStr for VerticalDistance {
    type Err = ParseOpenAirVerticalDistanceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value_fromstr = s
            .trim()
            .replace(' ', "")
            .trim_matches(char::is_alphabetic)
            .parse::<u16>()
            .map_err(|_| ParseOpenAirVerticalDistanceError);

        let suffix_fromstr = s.trim_matches(char::is_numeric).trim().to_uppercase();

        value_fromstr.map_or(
            // for value less distances we get an Err as value
            match suffix_fromstr.as_str() {
                "UNLIM" | "UNLIMITED" => Ok(VerticalDistance::Unlimited),
                "GND" | "SFC" => Ok(VerticalDistance::Gnd),
                _ => Err(ParseOpenAirVerticalDistanceError),
            },
            |value| match suffix_fromstr.as_str() {
                "FL" => Ok(VerticalDistance::Fl(value)),
                "FT AGL" | "AGL" => Ok(VerticalDistance::Agl(value)),
                "FT MSL" | "MSL" => Ok(VerticalDistance::Msl(value)),
                "FT" => Ok(VerticalDistance::Altitude(value)),
                _ => Err(ParseOpenAirVerticalDistanceError),
            },
        )
    }
}

pub struct OpenAirParser;

impl OpenAirParser {
    fn parse_command(command: &str, element: &mut OpenAirElement) -> Option<Airspace> {
        let record_type = command.get(0..2);
        let record = command.get(3..);
        let mut airspace = None;

        // TODO: Flag invalid airspaces!
        match record_type {
            Some("AC") => {
                if element.ac.is_some() {
                    airspace = Some(element.into());
                    *element = OpenAirElement::new();
                }

                element.ac = record?.parse::<String>().ok();
            }
            Some("AN") => element.an = record?.parse::<String>().ok(),
            Some("AH") => element.ah = record?.parse::<VerticalDistance>().ok(),
            Some("AL") => element.al = record?.parse::<VerticalDistance>().ok(),
            Some("DP") => {
                if let Ok(coordinate) = record?.parse::<Coordinate>() {
                    element.dp.push(coordinate);
                }
            }
            _ => {}
        }

        airspace
    }
}

impl Parser for OpenAirParser {
    fn parse(s: &str) -> Result<Airspaces, ParserError> {
        let mut element = OpenAirElement::new();
        let mut airspaces: Airspaces = Airspaces::new();

        s.lines().for_each(|command| {
            if let Some(airspace) = Self::parse_command(command, &mut element) {
                airspaces.push(airspace);
            }
        });

        airspaces.push((&mut element).into());

        Ok(airspaces)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::fc::dms_to_decimal;
    use crate::polygon;

    #[test]
    fn parses_command() {
        let airspaces = OpenAirParser::parse(
            r#"AC D
AN TMA BREMEN A
AH FL 65
AL 1500msl
DP 53:06:04 N 8:58:30 E
DP 53:06:10 N 9:04:45 E
DP 52:58:13 N 9:05:04 E
DP 52:58:08 N 8:58:56 E
DP 53:06:04 N 8:58:30 E
"#,
        );

        let tma_bremen_a = Airspace {
            name: String::from("TMA BREMEN A"),
            class: AirspaceClass::D,
            ceiling: VerticalDistance::Fl(65),
            floor: VerticalDistance::Msl(1500),
            polygon: polygon![
                (dms_to_decimal(53, 6, 4), dms_to_decimal(8, 58, 30)),
                (dms_to_decimal(53, 6, 10), dms_to_decimal(9, 4, 45)),
                (dms_to_decimal(52, 58, 13), dms_to_decimal(9, 5, 4)),
                (dms_to_decimal(52, 58, 8), dms_to_decimal(8, 58, 56)),
                (dms_to_decimal(53, 6, 4), dms_to_decimal(8, 58, 30))
            ],
        };

        assert_eq!(airspaces, Ok(vec!(tma_bremen_a)));
    }

    #[test]
    fn parses_coordinate() {
        let north_west = "37:53:00 N 116:55:30 W".parse::<Coordinate>();
        assert_eq!(
            north_west,
            Ok(Coordinate {
                latitude: dms_to_decimal(37, 53, 0),
                longitude: -dms_to_decimal(116, 55, 30),
            })
        );

        let south_east = "50:34:00 S 16:55:30 E".parse::<Coordinate>();
        assert_eq!(
            south_east,
            Ok(Coordinate {
                latitude: -dms_to_decimal(50, 34, 0),
                longitude: dms_to_decimal(16, 55, 30),
            })
        );

        let invalid = "50.1202 X 16.214 Y".parse::<Coordinate>();
        assert_eq!(invalid, Err(ParseOpenAirCoordinateError),);
    }

    #[test]
    fn parses_vertical_distance() {
        let agl = "1500 ft agl".parse::<VerticalDistance>();
        assert_eq!(agl, Ok(VerticalDistance::Agl(1500)));

        let altitude = "6400ft".parse::<VerticalDistance>();
        assert_eq!(altitude, Ok(VerticalDistance::Altitude(6400)));

        let fl = "FL95".parse::<VerticalDistance>();
        assert_eq!(fl, Ok(VerticalDistance::Fl(95)));

        let gnd = "GND".parse::<VerticalDistance>();
        assert_eq!(gnd, Ok(VerticalDistance::Gnd));

        let msl = "2500msl".parse::<VerticalDistance>();
        assert_eq!(msl, Ok(VerticalDistance::Msl(2500)));

        let unlimited = "UNLIM".parse::<VerticalDistance>(); // UNLIM (Mon-Fri)
        assert_eq!(unlimited, Ok(VerticalDistance::Unlimited));

        let err = "1500 foo".parse::<VerticalDistance>();
        assert_eq!(err, Err(ParseOpenAirVerticalDistanceError));
    }
}
