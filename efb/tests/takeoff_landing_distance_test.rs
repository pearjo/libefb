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

use std::collections::HashMap;
use std::str::FromStr;

use efb::aircraft::{LoadedStation, Station};
use efb::fp::{
    AlteringFactor, AlteringFactors, FactorOfEffect, MassAndBalance, RunwayAnalysis,
    TakeoffLandingPerformance, TakeoffLandingPerformanceChart,
};
use efb::nd::{Runway, RunwayConditionCode, RunwaySurface};
use efb::{VerticalDistance, Wind};

use efb::measurements::*;

fn c172r_takeoff_performance() -> TakeoffLandingPerformance {
    TakeoffLandingPerformance {
        chart: TakeoffLandingPerformanceChart::Table(vec![
            (
                VerticalDistance::PressureAltitude(0),
                Temperature::celsius(0.0),
                Length::ft(845.0),
                Length::ft(1510.0),
            ),
            (
                VerticalDistance::PressureAltitude(0),
                Temperature::celsius(10.0),
                Length::ft(910.0),
                Length::ft(1625.0),
            ),
            (
                VerticalDistance::PressureAltitude(0),
                Temperature::celsius(20.0),
                Length::ft(980.0),
                Length::ft(1745.0),
            ),
            (
                VerticalDistance::PressureAltitude(0),
                Temperature::celsius(30.0),
                Length::ft(1055.0),
                Length::ft(1875.0),
            ),
            (
                VerticalDistance::PressureAltitude(0),
                Temperature::celsius(40.0),
                Length::ft(1135.0),
                Length::ft(2015.0),
            ),
        ]),
        factors: AlteringFactors::new(&vec![
            // Decrease distances 10% for each 9 knots headwind. For operation
            // with tail winds up to 10 knots, increase distances by 10% for
            // each 2 knots.
            AlteringFactor::DecreaseHeadwind(FactorOfEffect::Rate {
                numerator: 0.1,
                denominator: Speed::kt(9.0),
            }),
            AlteringFactor::IncreaseTailwind(FactorOfEffect::Rate {
                numerator: 0.1,
                denominator: Speed::kt(2.0),
            }),
            // For operation on dry, grass runway, increase distances by 15% of
            // the "ground roll" figure.
            AlteringFactor::IncreaseRWYCC(HashMap::from([
                ((None, Some(RunwaySurface::Grass)), 0.15), // we'll add 15% on any grass
            ])),
        ]),
    }
}

#[test]
fn takeoff_ground_roll() {
    let perf = c172r_takeoff_performance();

    let west_grass_rwy = Runway {
        designator: String::from("27"),
        bearing: Angle::t(270.0),
        length: Length::m(1100.0),
        tora: Length::m(900.0),
        toda: Length::m(900.0),
        lda: Length::m(900.0),
        surface: RunwaySurface::Grass,
        slope: 0.0,
        elev: VerticalDistance::Gnd,
    };

    // the mass and balance is irrelevant for this test since we don't have any
    // mass factors
    let mb = MassAndBalance::new(&vec![LoadedStation {
        station: Station {
            arm: Length::m(1.0),
            description: None,
        },
        on_ramp: Mass::kg(1111.0),
        after_landing: Mass::kg(1111.0),
    }]);

    let analysis = RunwayAnalysis::takeoff(
        &west_grass_rwy,
        RunwayConditionCode::Six,
        &Wind::from_str("27010KT").unwrap(),
        Temperature::celsius(20.0),
        &mb,
        &perf,
    );

    assert_eq!(analysis.headwind(), &Speed::kt(10.0));
    assert_eq!(analysis.crosswind(), &Speed::kt(0.0));

    // We have 10kt headwind (-11.1111%) and dry grass (+15%). At 20°C on ground
    // level, the ground roll from the table is 980ft and distance to clear a
    // 50ft obstacle is 1745ft. The grass has only an effect on the ground roll
    // so we get the following estimated lengths:
    assert!(
        *(*analysis.ground_roll() - Length::ft(1001.7777)).value() <= f32::EPSILON,
        "the ground roll estimated with {} wasn't correct!",
        analysis.ground_roll()
    );
}
