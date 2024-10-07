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
use efb::nd::{Fix, InputFormat};

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
    let mut fms = FMS::new();

    if let Err(e) = fms.nd().read_file(&args.path, InputFormat::Arinc424) {
        eprintln!("Error reading ARINC 424: {e:?}");
    }

    if let Err(e) = fms.decode(args.route.as_str()) {
        eprintln!("Error decoding route: {e:?}");
    }

    if let Some(route) = fms.route() {
        println!("╭──────┬──────────┬──────┬──────┬───────┬───────╮");
        println!("│ TC   │  DIST    │ MC   │ MH   │ ETE   │ TO    │");
        println!("├──────┼──────────┼──────┼──────┼───────┼───────┤");

        for leg in route.legs() {
            println!(
                "│ {} │ {} │ {} │ {} │ {} │ {:5} │",
                leg.bearing(),
                leg.dist().to_nm(),
                leg.mc(),
                leg.mh(),
                leg.ete().round(),
                leg.to.ident(),
            );
        }

        println!("╰──────┴──────────┴──────┴──────┴───────┴───────╯");
    }
}
