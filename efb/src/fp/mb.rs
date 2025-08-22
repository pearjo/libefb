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

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::aircraft::LoadedStation;
use crate::measurements::{Length, LengthUnit, Mass};

/// The mass & balance on ramp and after landing.
///
/// The mass and balance of the [`Aircraft`] is computed from [`Station`]s
/// loaded on the aircraft. The mass is computed as sum of all station's mass
/// and the balance is the sum of all moment divided by the total mass.
///
/// [`Aircraft`]: crate::aircraft::Aircraft
/// [`Station`]: crate::aircraft::Station
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct MassAndBalance {
    on_ramp: Mass,
    after_landing: Mass,
    balance_on_ramp: Length,
    balance_after_landing: Length,
}

impl MassAndBalance {
    /// Computes the mass & balance from loaded stations.
    ///
    /// **Note: The stations must define all mass of the aircraft.** This
    /// includes the empty mass, fuel tanks and removable mass.
    pub fn new(loaded_stations: &Vec<LoadedStation>) -> Self {
        let mut on_ramp = Mass::kg(0.0);
        let mut after_landing = Mass::kg(0.0);
        let mut moment_on_ramp: f32 = 0.0;
        let mut moment_after_landing: f32 = 0.0;

        for loaded_station in loaded_stations {
            on_ramp = on_ramp + loaded_station.on_ramp;
            after_landing = after_landing + loaded_station.after_landing;
            moment_on_ramp += loaded_station.on_ramp.to_si() * loaded_station.station.arm().to_si();
            moment_after_landing +=
                loaded_station.after_landing.to_si() * loaded_station.station.arm().to_si();
        }

        Self {
            on_ramp,
            after_landing,
            balance_on_ramp: Length::from_si(moment_on_ramp / on_ramp.to_si(), LengthUnit::Meters),
            balance_after_landing: Length::from_si(
                moment_after_landing / after_landing.to_si(),
                LengthUnit::Meters,
            ),
        }
    }

    pub fn mass_on_ramp(&self) -> &Mass {
        &self.on_ramp
    }

    pub fn mass_after_landing(&self) -> &Mass {
        &self.after_landing
    }

    pub fn balance_on_ramp(&self) -> &Length {
        &self.balance_on_ramp
    }

    pub fn balance_after_landing(&self) -> &Length {
        &self.balance_after_landing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aircraft::Station;

    fn test_stations() -> Vec<LoadedStation> {
        vec![
            LoadedStation {
                station: Station::new(Length::m(1.0), None),
                on_ramp: Mass::kg(80.0),
                after_landing: Mass::kg(80.0),
            },
            // we have a skydiver in the back that jumps out during the flight
            LoadedStation {
                station: Station::new(Length::m(2.0), None),
                on_ramp: Mass::kg(80.0),
                after_landing: Mass::kg(0.0),
            },
        ]
    }

    #[test]
    fn mass_changes_during_flight() {
        let mb = MassAndBalance::new(&test_stations());
        assert_eq!(mb.mass_on_ramp(), &Mass::kg(160.0));
        assert_eq!(mb.mass_after_landing(), &Mass::kg(80.0));
    }

    #[test]
    fn balance_changes_during_flight() {
        let mb = MassAndBalance::new(&test_stations());
        assert_eq!(mb.balance_on_ramp(), &Length::m(1.50));
        assert_eq!(mb.balance_after_landing(), &Length::m(1.0));
    }
}
