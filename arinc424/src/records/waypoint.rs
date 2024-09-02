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

use crate::fields::*;
use std::str::FromStr;

pub struct Waypoint {
    pub record_type: RecordType,
    pub cust_area: CustArea,
    pub sec_code: SecCode,
    pub sub_code: SubCode<5>,
    pub regn_code: RegnCode<6>,
    pub icao_code: IcaoCode<10>,
    pub fix_ident: FixIdent<13>,
    pub cont_nr: ContNr<21>,
    pub waypoint_type: WaypointType<26>,
    pub waypoint_usage: WaypointUsage,
    pub latitude: Latitude<32>,
    pub longitude: Longitude<41>,
    pub mag_var: MagVar<74>,
    pub datum: Datum<84>,
    pub name_ind: NameInd<95>,
    pub name_desc: NameDesc<98>,
    pub frn: FileRecordNumber,
    pub cycle: Cycle,
}

impl FromStr for Waypoint {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            record_type: s.parse()?,
            cust_area: s.parse()?,
            sec_code: s.parse()?,
            sub_code: s.parse()?,
            regn_code: s.parse()?,
            icao_code: s.parse()?,
            fix_ident: s.parse()?,
            cont_nr: s.parse()?,
            waypoint_type: s.parse()?,
            waypoint_usage: s.parse()?,
            latitude: s.parse()?,
            longitude: s.parse()?,
            mag_var: s.parse()?,
            datum: s.parse()?,
            name_ind: s.parse()?,
            name_desc: s.parse()?,
            frn: s.parse()?,
            cycle: s.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PC_WAYPOINT: &'static str = "SEURPCEDDHED W1    ED0    V     N53341894E009404512                                 WGE           WHISKEY1                 122922407";

    #[test]
    fn waypoint_record() {
        match Waypoint::from_str(PC_WAYPOINT) {
            Ok(wp) => {
                assert_eq!(wp.record_type, RecordType::Standard);
                assert_eq!(wp.cust_area, CustArea::EUR);
                assert_eq!(wp.sec_code, SecCode::Airport);
                assert_eq!(wp.sub_code, SubCode::TerminalWaypoint);
                assert_eq!(wp.regn_code, "EDDH");
                assert_eq!(wp.icao_code, "ED");
                assert_eq!(wp.fix_ident, "W1   ");
                assert_eq!(wp.cont_nr, "0");
                assert_eq!(wp.waypoint_type, "V  ");
                assert_eq!(wp.waypoint_usage, WaypointUsage::TerminalOnly);
                assert_eq!(wp.latitude.cardinal, CardinalDirection::North);
                assert_eq!(wp.latitude.degree, 53);
                assert_eq!(wp.latitude.minutes, 34);
                assert_eq!(wp.latitude.seconds, 18);
                assert_eq!(wp.latitude.centiseconds, 94);
                assert_eq!(wp.longitude.cardinal, CardinalDirection::East);
                assert_eq!(wp.longitude.degree, 9);
                assert_eq!(wp.longitude.minutes, 40);
                assert_eq!(wp.longitude.seconds, 45);
                assert_eq!(wp.longitude.centiseconds, 12);
                assert_eq!(wp.mag_var, MagVar::Unknown);
                assert_eq!(wp.datum, Datum::WGE);
                assert_eq!(wp.name_ind, NameInd::Unspecified);
                assert_eq!(wp.name_desc, "WHISKEY1                 ");
                assert_eq!(wp.frn, 12292);
                assert_eq!(wp.cycle, Cycle { year: 24, month: 7 });
            }
            _ => panic!("Waypoint should be parsed."),
        }
    }
}
