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

/// Performance setting with 65% load in cruise.
///
/// This is the performance profile of a Cessna C172 with an TAE125-02-114
/// Diesel engine.
struct Performance65PercentLoad {}

impl Performance for Performance65PercentLoad {
    fn tas(&self, vd: &VerticalDistance) -> Speed {
        if *vd >= VerticalDistance::Altitude(10000) {
            Speed::Knots(114.0)
        } else if *vd >= VerticalDistance::Altitude(8000) {
            Speed::Knots(112.0)
        } else if *vd >= VerticalDistance::Altitude(6000) {
            Speed::Knots(110.0)
        } else if *vd >= VerticalDistance::Altitude(4000) {
            Speed::Knots(109.0)
        } else {
            Speed::Knots(107.0)
        }
    }

    fn ff(&self, _: &VerticalDistance) -> FuelFlow {
        FuelFlow::PerHour(diesel!(Volume::Liter(21.0)))
    }
}

fn main() {
    let args = Cli::parse();
    let perf = Performance65PercentLoad {};
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

    fms.fp().create_fuel_planning(
        FuelPolicy::Manual(diesel!(Volume::Liter(80.0))),
        diesel!(Volume::Liter(10.0)),
        &Reserve::Manual(Duration::from(1800)), // 30 min
        &perf,
    );

    if let Some(fuel) = fms.fp().fuel_planning() {
        println!("│ FUEL                                          │");
        println!("├───────────┬───────────────────────────────────┤");
        println!("│ TRIP      │ {:<8.0}                          │", fuel.trip);

        if let Some(climb) = fuel.climb {
            println!("│ CLIMB     │ {:<8.0}                          │", climb);
        }

        println!("│ TAXI      │ {:<8.0}                          │", fuel.taxi);

        if let Some(alternate) = fuel.alternate {
            println!("│ ALTERNATE │ {:<8.0}                          │", alternate);
        }

        println!("│ RESERVE   │ {:<8.0}                          │", fuel.reserve);
        println!("├───────────┼───────────────────────────────────┤");
        println!("│ MINIMUM   │ {:<8.0}                          │", fuel.min());

        if let Some(extra) = fuel.extra() {
            println!("│ EXTRA     │ {:<8.0}                          │", extra);
        }

        println!("├───────────┼───────────────────────────────────┤");
        println!("│ TOTAL     │ {:<8.0}                          │", fuel.total());
        println!("├───────────┴───────────────────────────────────┤");
    }

    println!("│                                               │");
    println!("╰───────────────────────────────────────────────╯");
}
