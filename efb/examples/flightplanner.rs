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

use clap::Parser;

/// Computes a flight plan from a route.
///
/// The flight planner uses navaids provided by an ARINC 424 database. The route
/// is composed out of route elements that provide performance data and fix
/// points. For more, please refer to the Route documentation.
#[derive(Parser)]
struct Cli {
    /// The path to the ARINC 424 navigation data to use
    path: std::path::PathBuf,
    /// The route and performance elements
    route: String,
}

fn main() {
    let args = Cli::parse();

    // Performance setting with 65% load in cruise.
    //
    // This is the performance profile of a Cessna C172 with an TAE125-02-114
    // Diesel engine.
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

    if let Err(e) = fms.nd().read_file(&args.path, InputFormat::Arinc424) {
        eprintln!("Error reading ARINC 424: {e:?}");
    }

    if let Err(e) = fms.decode(args.route.as_str()) {
        eprintln!("Error decoding route: {e:?}");
    }

    println!("╭───────────────────────────────────────────────╮");
    println!("│ ROUTE                                         │");
    println!("├──────┬──────────┬──────┬──────┬───────┬───────┤");
    println!("│ TC   │  DIST    │ MC   │ MH   │ ETE   │ TO    │");
    println!("├──────┼──────────┼──────┼──────┼───────┼───────┤");

    for leg in fms.route().legs() {
        match (leg.mh(), leg.ete()) {
            (Some(mh), Some(ete)) => {
                println!(
                    "│ {} │ {} │ {} │ {} │ {} │ {:5} │",
                    leg.bearing(),
                    leg.dist().to_nm(),
                    leg.mc(),
                    mh,
                    ete.round(),
                    leg.to.ident(),
                );
            }
            _ => {
                println!(
                    "│ {} │ {} │ {} │      │       │ {:5} │",
                    leg.bearing(),
                    leg.dist().to_nm(),
                    leg.mc(),
                    leg.to.ident(),
                );
            }
        }
    }

    println!("├──────┴──────────┴──────┴──────┴───────┴───────┤");

    if let Some(ete) = fms.route().ete() {
        println!("│ ETE {:8.0}                                  │", ete);
        println!("├───────────────────────────────────────────────┤");
    }

    if let Err(e) = fms.flight_planner().enter(FlightPlannerInput {
        aircraft: Some(aircraft),
        mass: Some(vec![
            // we're in the front
            Mass::Kilogram(80.0),
            // and no mass on the other stations
            Mass::Kilogram(0.0),
            Mass::Kilogram(0.0),
            Mass::Kilogram(0.0),
        ]),
        policy: Some(FuelPolicy::Manual(diesel!(Volume::Liter(80.0)))),
        taxi: Some(diesel!(Volume::Liter(10.0))),
        reserve: Some(Reserve::Manual(Duration::from(1800))), // 30 min
        perf: Some(perf),
    }) {
        eprintln!("Error when entering data into flight planner: {e:?}");
    }

    if let Some(fuel) = fms.flight_planner().fuel_planning() {
        println!("│ FUEL                                          │");
        println!("├───────────┬───────────────────────────────────┤");
        println!(
            "│ TRIP      │ {:<8.0}                          │",
            fuel.trip
        );

        if let Some(climb) = fuel.climb {
            println!("│ CLIMB     │ {:<8.0}                          │", climb);
        }

        println!(
            "│ TAXI      │ {:<8.0}                          │",
            fuel.taxi
        );

        if let Some(alternate) = fuel.alternate {
            println!(
                "│ ALTERNATE │ {:<8.0}                          │",
                alternate
            );
        }

        println!(
            "│ RESERVE   │ {:<8.0}                          │",
            fuel.reserve
        );
        println!("├───────────┼───────────────────────────────────┤");
        println!(
            "│ MINIMUM   │ {:<8.0}                          │",
            fuel.min()
        );

        if let Some(extra) = fuel.extra() {
            println!("│ EXTRA     │ {:<8.0}                          │", extra);
        }

        println!("├───────────┼───────────────────────────────────┤");
        println!(
            "│ TOTAL     │ {:<8.0}                          │",
            fuel.total()
        );
        println!("├───────────┴───────────────────────────────────┤");
    }

    let flight_planner = fms.flight_planner();

    if let Some(mb) = flight_planner.mb() {
        println!("│ MASS & BALANCE                                │");
        println!("├───────────────┬───────────────────────────────┤");
        println!(
            "│ BALANCED      │ {}                          │",
            flight_planner.is_balanced().unwrap()
        );
        println!(
            "│ ON RAMP       │ {:<8.0}                      │",
            mb.mass_on_ramp()
        );
        println!(
            "│ AFTER LANDING │ {:<8.0}                      │",
            mb.mass_after_landing()
        );
        println!("├───────────────┴───────────────────────────────┤");
    }

    println!("│                                               │");
    println!("╰───────────────────────────────────────────────╯");
}
