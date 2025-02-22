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

pub mod cg_envelope;
pub mod station;

use super::MassAndBalance;
use crate::error::Error;
use crate::{Distance, Fuel, FuelType, Mass, Volume};

pub use cg_envelope::{CGEnvelope, CGLimit};
pub use station::{LoadedStation, Station};

/// An aircraft's fuel tank.
#[derive(Copy, Clone, Debug)]
pub struct FuelTank {
    /// The tank's capacity.
    pub capacity: Volume,

    /// The distance of the tank to the aircraft's reference datum.
    pub arm: Distance,
}

/// The aircraft we're planning to fly with.
///
/// This aircraft provides all information necessary to execute the fuel and
/// mass & balance planning. The aircraft is created in it's empty
/// configuration. All additional mass are loaded at a [`Station`] that places
/// the mass at a fixed distance from a reference datum (the _arm_). The
/// aircraft is created only with the station arms and mass is mapped to the
/// stations when doing the mass & balance calculation. Stations that account
/// for the mass of fuel are derived from the tanks and fuel consumption.
///
/// The aircraft's mass & balance is calculated by [`mb`] for mass and fuel at
/// ramp and after landing. There are further methods to calculate the mass &
/// balance based on simplifications like constant mass during flight or equal
/// fuel distribution across all tanks.
///
/// [`mb`]: Aircraft::mb
///
/// # Examples
///
/// ```
/// use efb::{diesel, Distance, Fuel, FuelType, Mass, Volume};
/// use efb::fp::{Aircraft, CGEnvelope, CGLimit, FuelTank, MassAndBalance, Station};
///
/// // this is how a C172 of our flying club with a Diesel engine would look like:
/// let ac = Aircraft {
///     stations: vec![
///         Station {
///             arm: Distance::Meter(0.94),
///             description: Some(String::from("front seats")),
///         },
///         Station {
///             arm: Distance::Meter(1.85),
///             description: Some(String::from("back seats")),
///         },
///         Station {
///             arm: Distance::Meter(2.41),
///             description: Some(String::from("first cargo compartment")),
///         },
///         Station {
///             arm: Distance::Meter(3.12),
///             description: Some(String::from("second cargo compartment")),
///         },
///     ],
///     empty_mass: Mass::Kilogram(807.0),
///     empty_balance: Distance::Meter(1.0),
///     fuel_type: FuelType::Diesel,
///     tanks: vec![
///         FuelTank {
///             capacity: Volume::Liter(168.8),
///             arm: Distance::Meter(1.22),
///         },
///     ],
///     cg_envelope: CGEnvelope::new(vec![
///         CGLimit { mass: Mass::Kilogram(0.0), distance: Distance::Meter(0.89) },
///         CGLimit { mass: Mass::Kilogram(885.0), distance: Distance::Meter(0.89) },
///         CGLimit { mass: Mass::Kilogram(1111.0), distance: Distance::Meter(1.02) },
///         CGLimit { mass: Mass::Kilogram(1111.0), distance: Distance::Meter(1.20) },
///         CGLimit { mass: Mass::Kilogram(0.0), distance: Distance::Meter(1.20) },
///     ]),
/// };
///
/// // now we can calculate the mass & balance for a flight with one pilot on
/// // board and 20 Liter fuel consumption that is distributed equally across
/// // all tanks
/// let mb = ac.mb_from_const_mass_and_equally_distributed_fuel(
///     &vec![
///         // we're in the front
///         Mass::Kilogram(80.0),
///         // and no mass on the other stations
///         Mass::Kilogram(0.0),
///         Mass::Kilogram(0.0),
///         Mass::Kilogram(0.0)
///     ],
///     // we start our flight with 80 Liter of Diesel
///     &diesel!(Volume::Liter(80.0)),
///     // and land with 60 Liter remaining in our tank
///     &diesel!(Volume::Liter(60.0)),
/// );
///
/// // finally we can check if the aircraft is balanced throughout the flight
/// assert!(ac.is_balanced(&mb.unwrap()));
/// ```
#[derive(Clone, Debug)]
pub struct Aircraft {
    /// The distances from a reference datum at which mass can be loaded
    /// e.g. the position of a seat.
    pub stations: Vec<Station>,

    /// The mass of the empty aircraft taken from the last mass and balance
    /// report.
    pub empty_mass: Mass,

    /// The center of gravity of the empty aircraft taken from the last mass and
    /// balance report.
    pub empty_balance: Distance,

    /// The aircraft's fuel type.
    pub fuel_type: FuelType,

    /// The fuel tanks with their usable capacity.
    pub tanks: Vec<FuelTank>,

    /// The center of gravity envelope which must contains the CG at a mass for
    /// the aircraft to be balanced.
    pub cg_envelope: CGEnvelope,
}

impl Aircraft {
    /// Returns the usable fuel.
    ///
    /// The usable fuel is the sum of all tank capacities with the aircraft's
    /// fuel type or [`None`] if no tank is defined.
    pub fn usable_fuel(&self) -> Option<Fuel> {
        self.tanks
            .iter()
            .map(|tank| Fuel::from_volume(tank.capacity, &self.fuel_type))
            .reduce(|acc, fuel| acc + fuel)
    }

    /// Tests if the mass & balance is within the aircraft's [`cg_envelope`].
    pub fn is_balanced(&self, mb: &MassAndBalance) -> bool {
        self.cg_envelope.contains(mb)
    }

    /// Returns the mass & balance on ramp and after landing.
    ///
    /// The mass vectors are mapped to the station arms by position e.g. the
    /// mass at index 0 is placed at the station arm at index 0. Thus, the
    /// length of the mass vectors must match the length of the
    /// station arms. The same goes for the fuel, however the fuel vectors
    /// are mapped to the tanks.
    ///
    /// # Errors
    ///
    /// If the length of any mass vector doesn't match the length of the
    /// station arms or the length of any fuel vector doesn't match the
    /// tanks length, an error is returned.
    pub fn mb(
        &self,
        mass_on_ramp: &[Mass],
        mass_after_landing: &[Mass],
        fuel_on_ramp: &[Fuel],
        fuel_after_landing: &[Fuel],
    ) -> Result<MassAndBalance, Error> {
        let mut loaded_stations: Vec<LoadedStation> = Vec::new();
        loaded_stations.append(&mut self.stations_from_mass(mass_on_ramp, mass_after_landing)?);
        loaded_stations.append(&mut self.stations_from_fuel(fuel_on_ramp, fuel_after_landing)?);
        Ok(MassAndBalance::new(&loaded_stations))
    }

    pub fn mb_from_equally_distributed_fuel(
        &self,
        mass_on_ramp: &[Mass],
        mass_after_landing: &[Mass],
        on_ramp: &Fuel,
        after_landing: &Fuel,
    ) -> Result<MassAndBalance, Error> {
        let n = self.tanks.len();

        self.mb(
            mass_on_ramp,
            mass_after_landing,
            &vec![*on_ramp / n; n],
            &vec![*after_landing / n; n],
        )
    }

    pub fn mb_from_const_mass_and_equally_distributed_fuel(
        &self,
        mass: &[Mass],
        on_ramp: &Fuel,
        after_landing: &Fuel,
    ) -> Result<MassAndBalance, Error> {
        self.mb_from_equally_distributed_fuel(mass, mass, on_ramp, after_landing)
    }

    /// Returns a station representing the empty aircraft.
    fn empty(&self) -> LoadedStation {
        LoadedStation {
            station: Station {
                arm: self.empty_balance,
                description: Some(String::from("Empty Aircraft")),
            },
            on_ramp: self.empty_mass,
            after_landing: self.empty_mass,
        }
    }

    /// Returns stations for the mass on ramp and after landing where each entry
    /// is mapped to the station arms at the same index.
    fn stations_from_mass(
        &self,
        on_ramp: &[Mass],
        after_landing: &[Mass],
    ) -> Result<Vec<LoadedStation>, Error> {
        let n = self.stations.len();

        // The mass must match our station arms!
        if on_ramp.len() == n && after_landing.len() == n {
            let mut loaded_stations = vec![self.empty()];

            for i in 0..n {
                loaded_stations.push(LoadedStation {
                    station: self.stations[i].clone(),
                    on_ramp: on_ramp[i],
                    after_landing: after_landing[i],
                });
            }

            Ok(loaded_stations)
        } else {
            Err(Error::UnexpectedMassesForStations)
        }
    }

    /// Returns stations for the fuel on ramp and after landing where each entry
    /// is mapped to the tanks at the same index.
    fn stations_from_fuel(
        &self,
        on_ramp: &[Fuel],
        after_landing: &[Fuel],
    ) -> Result<Vec<LoadedStation>, Error> {
        let n = self.tanks.len();

        // We can't load more fuel than we have tanks!
        if on_ramp.len() == n && after_landing.len() == n {
            let mut loaded_stations: Vec<LoadedStation> = Vec::new();

            for i in 0..n {
                let fuel_on_ramp = on_ramp[i];
                let fuel_after_landing = after_landing[i];
                let tank = self.tanks[i];

                // The fuel after landing might be more than on ramp (if we do
                // air refueling with our C172), but it can never be more than
                // our tank's capacity!
                if fuel_on_ramp.volume() > tank.capacity {
                    return Err(Error::ExceededFuelCapacityOnRamp);
                }

                if fuel_after_landing.volume() > tank.capacity {
                    return Err(Error::ExceededFuelCapacityAfterLanding);
                }

                loaded_stations.push(LoadedStation {
                    station: Station {
                        arm: tank.arm,
                        description: None,
                    },
                    on_ramp: fuel_on_ramp.mass,
                    after_landing: fuel_after_landing.mass,
                });
            }

            Ok(loaded_stations)
        } else {
            Err(Error::UnexpectedNumberOfFuelStations)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usable_fuel_matches_tank_capacity() {
        // we have two fuel tanks with 40 Liter each
        let ac = Aircraft {
            stations: vec![],
            empty_mass: Mass::Kilogram(0.0),
            empty_balance: Distance::Meter(0.0),
            fuel_type: FuelType::Diesel,
            tanks: vec![
                FuelTank {
                    capacity: Volume::Liter(40.0),
                    arm: Distance::Meter(1.0),
                },
                FuelTank {
                    capacity: Volume::Liter(40.0),
                    arm: Distance::Meter(1.0),
                },
            ],
            cg_envelope: CGEnvelope::new(vec![]),
        };

        // thus our total usable fuel is 80 Liter
        assert_eq!(ac.usable_fuel(), Some(diesel!(Volume::Liter(80.0))));
    }

    #[test]
    #[should_panic(expected = "UnexpectedMassesForStations")]
    fn create_stations_with_missing_mass() {
        let ac = Aircraft {
            stations: vec![
                Station {
                    arm: Distance::Meter(1.0),
                    description: None,
                },
                Station {
                    arm: Distance::Meter(2.0),
                    description: None,
                },
            ],
            empty_mass: Mass::Kilogram(0.0),
            empty_balance: Distance::Meter(0.0),
            fuel_type: FuelType::Diesel,
            tanks: vec![],
            cg_envelope: CGEnvelope::new(vec![]),
        };

        // the aircraft has two stations but we provide no mass for any
        ac.stations_from_mass(&vec![], &vec![]).unwrap();
    }

    #[test]
    fn stations_include_empty_mass() {
        // we configure no station and only an empty mass of 800 kg at 1.0 m
        let ac = Aircraft {
            stations: vec![],
            empty_mass: Mass::Kilogram(800.0),
            empty_balance: Distance::Meter(1.0),
            fuel_type: FuelType::Diesel,
            tanks: vec![],
            cg_envelope: CGEnvelope::new(vec![]),
        };

        let stations = ac.stations_from_mass(&vec![], &vec![]).unwrap();

        // we're expecting only our fuel station
        assert_eq!(stations.len(), 1);
        // which should match our empty station
        assert_eq!(stations[0], ac.empty());
    }
}
