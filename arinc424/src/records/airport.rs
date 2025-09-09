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

// TODO add missing fields
pub struct Airport {
    pub record_type: RecordType,
    pub cust_area: CustArea,
    pub sec_code: SecCode,
    pub arpt_ident: ArptHeliIdent<6>,
    pub icao_code: IcaoCode<10>,
    pub sub_code: SubCode<12>,
    pub iata: Iata<13>,
    pub cont_nr: ContNr<21>,
    pub latitude: Latitude<32>,
    pub longitude: Longitude<41>,
    pub mag_var: MagVar<51, 32, 41>,
    pub mag_true_ind: MagTrueInd<85>,
    pub datum: Datum<86>,
    pub frn: FileRecordNumber,
    pub cycle: Cycle,
}

impl FromStr for Airport {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            record_type: s.parse()?,
            cust_area: s.parse()?,
            sec_code: s.parse()?,
            arpt_ident: s.parse()?,
            icao_code: s.parse()?,
            sub_code: s.parse()?,
            iata: s.parse()?,
            cont_nr: s.parse()?,
            latitude: s.parse()?,
            longitude: s.parse()?,
            mag_var: s.parse()?,
            mag_true_ind: s.parse()?,
            datum: s.parse()?,
            frn: s.parse()?,
            cycle: s.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PA_AIRPORT: &'static str = "SEURP EDDHEDA        0        N N53374900E009591762E002000053                   P    MWGE    HAMBURG                       356512407";

    #[test]
    fn airport_record() {
        match Airport::from_str(PA_AIRPORT) {
            Ok(wp) => {
                assert_eq!(wp.record_type, RecordType::Standard);
                assert_eq!(wp.cust_area, CustArea::EUR);
                assert_eq!(wp.sec_code, SecCode::Airport);
                assert_eq!(wp.arpt_ident, "EDDH");
                assert_eq!(wp.icao_code, "ED");
                assert_eq!(wp.sub_code, SubCode::ReferencePoint);
                assert_eq!(wp.iata, "   ");
                assert_eq!(wp.cont_nr, "0");
                assert_eq!(wp.latitude.cardinal, CardinalDirection::North);
                assert_eq!(wp.latitude.degree, 53);
                assert_eq!(wp.latitude.minutes, 37);
                assert_eq!(wp.latitude.seconds, 49);
                assert_eq!(wp.latitude.centiseconds, 0);
                assert_eq!(wp.longitude.cardinal, CardinalDirection::East);
                assert_eq!(wp.longitude.degree, 9);
                assert_eq!(wp.longitude.minutes, 59);
                assert_eq!(wp.longitude.seconds, 17);
                assert_eq!(wp.longitude.centiseconds, 62);
                assert_eq!(wp.mag_var, MagVar::East(2, 0));
                assert_eq!(wp.mag_true_ind, MagTrueInd::Magnetic);
                assert_eq!(wp.datum, Datum::WGE);
                assert_eq!(wp.frn, 35651);
                assert_eq!(wp.cycle, Cycle { year: 24, cycle: 7 });
            }
            _ => panic!("Waypoint should be parsed."),
        }
    }
}
