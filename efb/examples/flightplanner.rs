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

use efb::aircraft::*;
use efb::fms::*;
use efb::fp::*;
use efb::measurements::*;
use efb::nd::InputFormat;
use efb::*;

const ARINC_424_RECORDS: &'static str = r#"SEURP EDDHEDA        0        N N53374900E009591762E002000053                   P    MWGE    HAMBURG                       356462409
SEURPCEDDHED N1    ED0    V     N53482105E010015451                                 WGE           NOVEMBER1                359892409
SEURPCEDDHED N2    ED0    V     N53405701E010000576                                 WGE           NOVEMBER2                359902409
SEURP EDHFEDA        0        N N53593300E009343600E000000082                   P    MWGE    ITZEHOE/HUNGRIGER WOLF        320782409
"#;

fn main() {
    // Performance setting with 65% load in cruise. This is the performance
    // profile of a Cessna C172 with an TAE125-02-114 Diesel engine.
    let perf = Performance::from(
        |vd| {
            let tas = if *vd >= VerticalDistance::Altitude(10000) {
                Speed::kt(114.0)
            } else if *vd >= VerticalDistance::Altitude(8000) {
                Speed::kt(112.0)
            } else if *vd >= VerticalDistance::Altitude(6000) {
                Speed::kt(110.0)
            } else if *vd >= VerticalDistance::Altitude(4000) {
                Speed::kt(109.0)
            } else {
                Speed::kt(107.0)
            };

            let ff = FuelFlow::PerHour(diesel!(Volume::l(21.0)));

            (tas, ff)
        },
        // The data end at 10000 ft so we don't need to create the Performance
        // with more values.
        VerticalDistance::Altitude(10000),
    );

    let aircraft = Aircraft {
        registration: String::from("N12345"),
        stations: vec![
            Station {
                arm: Length::m(0.94),
                description: Some(String::from("front seats")),
            },
            Station {
                arm: Length::m(1.85),
                description: Some(String::from("back seats")),
            },
            Station {
                arm: Length::m(2.41),
                description: Some(String::from("first cargo compartment")),
            },
            Station {
                arm: Length::m(3.12),
                description: Some(String::from("second cargo compartment")),
            },
        ],
        empty_mass: Mass::kg(807.0),
        empty_balance: Length::m(1.0),
        fuel_type: FuelType::Diesel,
        tanks: vec![FuelTank {
            capacity: Volume::l(168.8),
            arm: Length::m(1.22),
        }],
        cg_envelope: CGEnvelope::new(vec![
            CGLimit {
                mass: Mass::kg(0.0),
                distance: Length::m(0.89),
            },
            CGLimit {
                mass: Mass::kg(885.0),
                distance: Length::m(0.89),
            },
            CGLimit {
                mass: Mass::kg(1111.0),
                distance: Length::m(1.02),
            },
            CGLimit {
                mass: Mass::kg(1111.0),
                distance: Length::m(1.20),
            },
            CGLimit {
                mass: Mass::kg(0.0),
                distance: Length::m(1.20),
            },
        ]),
        notes: None,
    };

    let mut fms = FMS::new();

    // read the ARINC database
    let _ = fms.nd().read(ARINC_424_RECORDS, InputFormat::Arinc424);

    // decode a route from EDDH to EDHF with winds at 20 kt from 290° and
    // cruising speed of 107 kt and an altitude of 2500 ft.
    let _ = fms.decode("29020KT N0107 A0250 EDDH DHN2 DHN1 EDHF");

    // Now we can enter some data into the flight planning to get a fuel planning
    // and mass & balance calculation.
    let mut builder = FlightPlanning::builder();

    builder
        .set_aircraft(aircraft)
        .set_mass(vec![
            // we're in the front
            Mass::kg(80.0),
            // and no mass on the other stations
            Mass::kg(0.0),
            Mass::kg(0.0),
            Mass::kg(0.0),
        ])
        .set_policy(FuelPolicy::ManualFuel(diesel!(Volume::l(80.0))))
        .set_taxi(diesel!(Volume::l(10.0)))
        .set_reserve(Reserve::Manual(Duration::s(1800))) // 30 min
        .set_perf(perf);

    let _ = fms.build_flight_planning(&builder);

    println!("{}", fms.print(40))
}
