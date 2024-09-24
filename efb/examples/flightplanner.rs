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
use efb::fp::Route;

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the ARINC 424 navigation data to use
    path: std::path::PathBuf,
    /// The route and performance elements
    route: String,
}

fn main() {
    let args = Cli::parse();

    if let Ok(fms) = FMS::from_arinc424(&args.path) {
        match Route::decode(args.route.as_str(), &fms.navigation_data) {
            Ok(route) => {
                println!("╭──────┬──────────┬──────┬──────┬───────┬───────╮");
                println!("│ TC   │  DIST    │ MC   │ MH   │ TIME  │ TO    │");
                println!("├──────┼──────────┼──────┼──────┼───────┼───────┤");

                for leg in route.legs() {
                    println!(
                        "│ {} │ {} │ {} │ {} │ {} │ {:5} │",
                        leg.bearing(),
                        leg.dist().to_nm(),
                        leg.mc(),
                        leg.mh(),
                        leg.time().round(),
                        leg.to.ident(),
                    );
                }

                println!("╰──────┴──────────┴──────┴──────┴───────┴───────╯");
            }
            Err(e) => eprintln!("Error: {e:?}"),
        }
    }
}
