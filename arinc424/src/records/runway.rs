// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Joe Pearson
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

// TODO add missing fields and handle different versions
pub struct Runway {
    pub record_type: RecordType,
    pub cust_area: CustArea,
    pub sec_code: SecCode,
    pub arpt_ident: ArptHeliIdent<6>,
    pub icao_code: IcaoCode<10>,
    pub sub_code: SubCode<12>,
    pub runway_id: RunwayId<13>,
    pub cont_nr: ContNr<21>,
    /// Runway length in feet.
    pub runway_length: NumericField<22, 5>,
    pub rwy_brg: RwyBrg<27>,
    pub threshould_source: Source<31>,
    pub threshould_latitude: Latitude<32>,
    pub threshould_longitude: Longitude<41>,
    pub rwy_grad: RwyGrad<51>,
    pub frn: FileRecordNumber,
    pub cycle: Cycle,
}

impl FromStr for Runway {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            record_type: s.parse()?,
            cust_area: s.parse()?,
            sec_code: s.parse()?,
            arpt_ident: s.parse()?,
            icao_code: s.parse()?,
            sub_code: s.parse()?,
            runway_id: s.parse()?,
            cont_nr: s.parse()?,
            runway_length: s.parse()?,
            rwy_brg: s.parse()?,
            threshould_source: s.parse()?,
            threshould_latitude: s.parse()?,
            threshould_longitude: s.parse()?,
            // TODO: How to properly handle fields that are not provided?
            rwy_grad: s.parse().unwrap_or_default(),
            frn: s.parse()?,
            cycle: s.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PG_RUNWAY: &'static str = "SEURP EDDHEDGRW33    0120273330 N53374300E009595081                          151                                           124362502";

    #[test]
    fn runway_record() {
        match Runway::from_str(PG_RUNWAY) {
            Ok(rwy) => {
                assert_eq!(rwy.record_type, RecordType::Standard);
                assert_eq!(rwy.cust_area, CustArea::EUR);
                assert_eq!(rwy.sec_code, SecCode::Airport);
                assert_eq!(rwy.arpt_ident, "EDDH");
                assert_eq!(rwy.icao_code, "ED");
                assert_eq!(rwy.sub_code, SubCode::Runway);
                assert_eq!(rwy.runway_id.designator, "33".to_string());
                assert_eq!(rwy.cont_nr, "0");
                assert_eq!(rwy.runway_length, 12027u32);
                assert_eq!(rwy.rwy_brg, RwyBrg::MagneticNorth(333.0));
                assert_eq!(rwy.threshould_source, Source::OtherSources);
                assert_eq!(rwy.frn, 12436);
                assert_eq!(rwy.cycle, Cycle { year: 25, cycle: 2 });
            }
            Err(e) => panic!("Runway should be parsed. {:#?}", e),
        }
    }
}
