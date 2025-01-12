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

use efb::fms::*;
use efb::fp::*;
use efb::nd::{Fix, InputFormat};
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
                Speed::Knots(114.0)
            } else if *vd >= VerticalDistance::Altitude(8000) {
                Speed::Knots(112.0)
            } else if *vd >= VerticalDistance::Altitude(6000) {
                Speed::Knots(110.0)
            } else if *vd >= VerticalDistance::Altitude(4000) {
                Speed::Knots(109.0)
            } else {
                Speed::Knots(107.0)
            };

            let ff = FuelFlow::PerHour(diesel!(Volume::Liter(21.0)));

            (tas, ff)
        },
        // The data end at 10000 ft so we don't need to create the Performance
        // with more values.
        VerticalDistance::Altitude(10000),
    );

    let aircraft = Aircraft {
        station_arms: vec![
            // the front seats
            Distance::Meter(0.94),
            // the back seats
            Distance::Meter(1.85),
            // the first cargo compartment
            Distance::Meter(2.41),
            // the second cargo compartment
            Distance::Meter(3.12),
        ],
        empty_mass: Mass::Kilogram(807.0),
        empty_balance: Distance::Meter(1.0),
        fuel_type: FuelType::Diesel,
        tanks: vec![FuelTank {
            capacity: Volume::Liter(168.8),
            arm: Distance::Meter(1.22),
        }],
        cg_envelope: CGEnvelope::new(vec![
            (Mass::Kilogram(0.0), Distance::Meter(0.89)),
            (Mass::Kilogram(885.0), Distance::Meter(0.89)),
            (Mass::Kilogram(1111.0), Distance::Meter(1.02)),
            (Mass::Kilogram(1111.0), Distance::Meter(1.20)),
            (Mass::Kilogram(0.0), Distance::Meter(1.20)),
        ]),
    };

    let mut fms = FMS::new();

    // read the ARINC database
    let _ = fms.nd().read(ARINC_424_RECORDS, InputFormat::Arinc424);

    // decode a route from EDDH to EDHF with winds at 20 kt from 290° and
    // cruising speed of 107 kt and an altitude of 2500 ft.
    let _ = fms.decode("29020KT N0107 A0250 EDDH DHN2 DHN1 EDHF");

    println!("\n   Route\n");

    let route = fms.route();

    for leg in route.legs() {
        println!(
            "{} - {}: TC: {}, dist: {}, MC: {}, MH: {}, ETE: {}",
            leg.from().ident(),
            leg.to().ident(),
            leg.bearing(),
            leg.dist().to_nm(),
            leg.mc(),
            leg.mh().unwrap(),
            leg.ete().unwrap(),
        );
    }

    if let Some(ete) = route.ete() {
        println!("\nETE: {}", ete);
    }

    // Now we can enter some data into the flight planning to get a fuel planning
    // and mass & balance calculation.
    let mut builder = FlightPlanning::builder();

    builder
        .set_aircraft(aircraft)
        .set_mass(vec![
            // we're in the front
            Mass::Kilogram(80.0),
            // and no mass on the other stations
            Mass::Kilogram(0.0),
            Mass::Kilogram(0.0),
            Mass::Kilogram(0.0),
        ])
        .set_policy(FuelPolicy::ManualFuel(diesel!(Volume::Liter(80.0))))
        .set_taxi(diesel!(Volume::Liter(10.0)))
        .set_reserve(Reserve::Manual(Duration::from(1800))) // 30 min
        .set_perf(perf);

    let _ = fms.build_flight_planning(&builder);

    if let Some(flight_planning) = fms.flight_planning() {
        if let Some(fuel_planning) = flight_planning.fuel_planning() {
            println!("\n   Fuel\n");

            println!(
                "trip:    {:>4.0}, taxi:  {:>4.0}, reserve: {:>4.0}",
                fuel_planning.trip, fuel_planning.taxi, fuel_planning.reserve
            );

            println!(
                "minimum: {:>4.0}, extra: {:>4.0}, total:   {:>4.0}",
                fuel_planning.min(),
                fuel_planning.extra().unwrap(),
                fuel_planning.total()
            );
        }

        if let Some(is_balanced) = flight_planning.is_balanced() {
            println!("\n   Mass & Balance\n");

            // With a proper configured aircraft and fuel planning, we get our mass &
            // balance and can check whether the aircraft is balanced.
            println!("balanced: {}", is_balanced);
        }

        if let Some(mb) = flight_planning.mb() {
            println!(
                "on ramp: {} - after landing: {}",
                mb.mass_on_ramp(),
                mb.mass_after_landing()
            );
        }

        println!("");
    }
}
