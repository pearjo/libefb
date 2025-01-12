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

use super::Station;
use crate::{Distance, Mass, Unit};

/// The mass & balance on ramp and after landing.
///
/// The mass and balance of the [`Aircraft`] is computed from [`Station`]s
/// loaded on the aircraft. The mass is computed as sum of all station's mass
/// and the balance is the sum of all moment divided by the total mass.
///
/// [`Aircraft`]: super::Aircraft
/// [`Station`]: super::Station
#[derive(Debug)]
pub struct MassAndBalance {
    on_ramp: Mass,
    after_landing: Mass,
    balance_on_ramp: Distance,
    balance_after_landing: Distance,
}

impl MassAndBalance {
    /// Computes the mass & balance from stations.
    ///
    /// **Note: The stations must define all mass of the aircraft.** This
    /// includes the empty mass, fuel tanks and removable mass.
    pub fn new(stations: &Vec<Station>) -> Self {
        let mut on_ramp = Mass::Kilogram(0.0);
        let mut after_landing = Mass::Kilogram(0.0);
        let mut moment_on_ramp: f32 = 0.0;
        let mut moment_after_landing: f32 = 0.0;

        for station in stations {
            on_ramp = on_ramp + station.on_ramp;
            after_landing = after_landing + station.after_landing;
            moment_on_ramp += station.on_ramp.si() * station.arm.si();
            moment_after_landing += station.after_landing.si() * station.arm.si();
        }

        Self {
            on_ramp,
            after_landing,
            balance_on_ramp: Distance::from_si(moment_on_ramp / on_ramp.si()),
            balance_after_landing: Distance::from_si(moment_after_landing / after_landing.si()),
        }
    }

    pub fn mass_on_ramp(&self) -> &Mass {
        &self.on_ramp
    }

    pub fn mass_after_landing(&self) -> &Mass {
        &self.after_landing
    }

    pub fn balance_on_ramp(&self) -> &Distance {
        &self.balance_on_ramp
    }

    pub fn balance_after_landing(&self) -> &Distance {
        &self.balance_after_landing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_stations() -> Vec<Station> {
        vec![
            Station {
                on_ramp: Mass::Kilogram(80.0),
                after_landing: Mass::Kilogram(80.0),
                arm: Distance::Meter(1.0),
            },
            // we have a skydiver in the back that jumps out during the flight
            Station {
                on_ramp: Mass::Kilogram(80.0),
                after_landing: Mass::Kilogram(0.0),
                arm: Distance::Meter(2.0),
            },
        ]
    }

    #[test]
    fn mass_changes_during_flight() {
        let mb = MassAndBalance::new(&test_stations());
        assert_eq!(mb.mass_on_ramp(), &Mass::Kilogram(160.0));
        assert_eq!(mb.mass_after_landing(), &Mass::Kilogram(80.0));
    }

    #[test]
    fn balance_changes_during_flight() {
        let mb = MassAndBalance::new(&test_stations());
        assert_eq!(mb.balance_on_ramp(), &Distance::Meter(1.50));
        assert_eq!(mb.balance_after_landing(), &Distance::Meter(1.0));
    }
}
