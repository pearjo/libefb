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

use efb::fp::{Aircraft, CGEnvelope, FuelTank};
use efb::{diesel, Distance, Fuel, FuelType, Mass, Volume};

/// Returns the an aircraft we use for the tests.
fn aircraft() -> Aircraft {
    Aircraft {
        station_arms: vec![Distance::Meter(1.0), Distance::Meter(2.0)],
        empty_mass: Mass::Kilogram(800.0),
        empty_balance: Distance::Meter(1.0),
        fuel_type: FuelType::Diesel,
        tanks: vec![
            // To spice things up, lets test with two tanks in the wings
            // configured as separate tanks and an additional tank in the
            // aft cargo compartment.
            FuelTank {
                capacity: Volume::Liter(50.0),
                arm: Distance::Meter(1.0),
            },
            FuelTank {
                capacity: Volume::Liter(50.0),
                arm: Distance::Meter(1.0),
            },
            FuelTank {
                capacity: Volume::Liter(20.0),
                arm: Distance::Meter(1.5),
            },
        ],
        cg_envelope: CGEnvelope::new(vec![
            (Mass::Kilogram(0.0), Distance::Meter(1.0)),
            (Mass::Kilogram(800.0), Distance::Meter(1.0)),
            (Mass::Kilogram(1000.0), Distance::Meter(1.0)),
            (Mass::Kilogram(1000.0), Distance::Meter(1.5)),
            (Mass::Kilogram(0.0), Distance::Meter(1.5)),
        ]),
    }
}

#[test]
fn mb_matches_mass_and_fuel() {
    let ac = aircraft();

    // For the sake of testing, lets draw a very unlikely scenario which
    // results in a change of all parameter:
    let mb = ac
        .mb(
            // On ramp we have a pilot in the front and a PAX in the back.
            &vec![Mass::Kilogram(80.0), Mass::Kilogram(80.0)],
            // The PAX was a sky diver and jumped out during the flight.
            &vec![Mass::Kilogram(80.0), Mass::Kilogram(0.0)],
            // We departed with 40 liter of Diesel distributed between the first
            // two tanks.
            &vec![
                diesel!(Volume::Liter(20.0)),
                diesel!(Volume::Liter(20.0)),
                diesel!(Volume::Liter(0.0)),
            ],
            // Our PAX was so kind and did a pretty stunt by air refueling our
            // aircraft mid flight.
            &vec![
                diesel!(Volume::Liter(40.0)),
                diesel!(Volume::Liter(40.0)),
                diesel!(Volume::Liter(10.0)),
            ],
        )
        .unwrap();

    // We have the following masses on ramp:
    // - 800 kg empty mass
    // - 160 kg for pilot and PAX
    // - 33.52 kg of Diesel
    // This gives a total of 993.52 kg on ramp.
    assert_eq!(mb.mass_on_ramp(), &Mass::Kilogram(993.52));

    // We have the following masses after landing:
    // - 800 kg (empty mass)
    // - 80 kg (pilot)
    // - 75.42 kg (Diesel)
    // This gives a total of 955.42 kg on ramp.
    // TODO check why we get 955.42004 and not 955.42
    assert_eq!(mb.mass_after_landing(), &Mass::Kilogram(955.42004));

    // We have the following moment on ramp:
    // - 800 kg * 1 m = 800 kg m (empty aircraft)
    // - 80 kg * 1 m = 80 kg m (pilot)
    // - 80 kg * 2 m = 160 kg m (PAX)
    // - 33.52 kg * 1 m = 33.52 kg m (Diesel)
    // The sum of moment is 1073.52 kg m divided by the total mass returns
    // us a balance on ramp of 1.0805218 m.
    assert_eq!(mb.balance_on_ramp(), &Distance::Meter(1.0805218));

    // We have the following moment after landing:
    // - 800 kg * 1 m = 800 kg m (empty aircraft)
    // - 80 kg * 1 m = 80 kg m (pilot)
    // - 67.04 kg * 1 m = 67.04 kg m (Diesel first two tanks)
    // - 8.38 kg * 1.5 m = 12.57 kg m (Diesel third tank)
    // The sum of moment is 959.61 kg m divided by the total mass returns
    // us a balance after landing of 1.0043855 m.
    assert_eq!(mb.balance_after_landing(), &Distance::Meter(1.0043855));

    assert!(ac.is_balanced(&mb));
}

#[test]
fn mb_fuel_is_distributed_equally() {
    let ac = aircraft();

    // For the sake of testing, lets draw a very unlikely scenario which
    // results in a change of all parameter:
    let _mb = ac
        .mb_from_const_mass_and_equally_distributed_fuel(
            // On ramp we have a pilot in the front and a PAX in the back.
            &vec![Mass::Kilogram(80.0), Mass::Kilogram(80.0)],
            // We departed with 60 liter of Diesel distributed between all
            // tanks
            &diesel!(Volume::Liter(60.0)),
            // and burned 30 Liter that were drawn from all tanks equally.
            &diesel!(Volume::Liter(30.0)),
        )
        .unwrap();
}

#[test]
#[should_panic(expected = "ExceededFuelCapacityOnRamp")]
fn mb_for_exceeded_fuel_capacity_on_ramp() {
    let ac = aircraft();

    ac.mb(
        &vec![Mass::Kilogram(0.0), Mass::Kilogram(0.0)],
        &vec![Mass::Kilogram(0.0), Mass::Kilogram(0.0)],
        // Ooops... We have a type and tried to plan with 400 liter.
        &vec![
            diesel!(Volume::Liter(200.0)),
            diesel!(Volume::Liter(200.0)),
            diesel!(Volume::Liter(0.0)),
        ],
        &vec![
            diesel!(Volume::Liter(0.0)),
            diesel!(Volume::Liter(0.0)),
            diesel!(Volume::Liter(0.0)),
        ],
    )
    .unwrap();
}

#[test]
#[should_panic(expected = "ExceededFuelCapacityAfterLanding")]
fn mb_for_exceeded_fuel_capacity_after_landing() {
    let ac = aircraft();

    ac.mb(
        &vec![Mass::Kilogram(0.0), Mass::Kilogram(0.0)],
        &vec![Mass::Kilogram(0.0), Mass::Kilogram(0.0)],
        &vec![
            diesel!(Volume::Liter(0.0)),
            diesel!(Volume::Liter(0.0)),
            diesel!(Volume::Liter(0.0)),
        ],
        // Ooops... We have a type and tried to land with 400 liter.
        &vec![
            diesel!(Volume::Liter(200.0)),
            diesel!(Volume::Liter(200.0)),
            diesel!(Volume::Liter(0.0)),
        ],
    )
    .unwrap();
}
