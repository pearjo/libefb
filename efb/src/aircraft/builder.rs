// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Joe Pearson
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

use super::*;
use crate::error::Error;
use crate::measurements::{Length, Mass};

/// Aircraft factory, which can be used to configure the properties of an Aircraft.
///
/// # Examples
///
/// ```
/// # use efb::FuelType;
/// # use efb::measurements::{Mass, Length};
/// use efb::aircraft::Aircraft;
///
/// let mut builder = Aircraft::builder();
///
/// // this is a very impractical aircraft since it has no stations nor tanks
/// let aircraft = builder
///     .registration("N12345".to_string())
///     .empty_mass(Mass::kg(807.0))
///     .empty_balance(Length::m(1.0))
///     .fuel_type(FuelType::JetA)
///     .build();
///
/// assert!(aircraft.is_ok());
/// ```
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct AircraftBuilder {
    registration: Option<String>,
    icao_type: Option<String>,
    stations: Vec<Station>,
    empty_mass: Option<Mass>,
    empty_balance: Option<Length>,
    fuel_type: Option<FuelType>,
    tanks: Vec<FuelTank>,
    cg_envelope: Vec<CGLimit>,
    notes: Option<String>,
}

impl AircraftBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn registration(&mut self, registration: String) -> &mut Self {
        self.registration = Some(registration);
        self
    }

    pub fn icao_type(&mut self, icao_type: String) -> &mut Self {
        self.icao_type = Some(icao_type);
        self
    }

    pub fn stations(&mut self, stations: Vec<Station>) -> &mut Self {
        self.stations = stations;
        self
    }

    pub fn empty_mass(&mut self, empty_mass: Mass) -> &mut Self {
        self.empty_mass = Some(empty_mass);
        self
    }

    pub fn empty_balance(&mut self, empty_balance: Length) -> &mut Self {
        self.empty_balance = Some(empty_balance);
        self
    }

    pub fn fuel_type(&mut self, fuel_type: FuelType) -> &mut Self {
        self.fuel_type = Some(fuel_type);
        self
    }

    pub fn tanks(&mut self, tanks: Vec<FuelTank>) -> &mut Self {
        self.tanks = tanks;
        self
    }

    pub fn cg_envelope(&mut self, cg_envelope: Vec<CGLimit>) -> &mut Self {
        self.cg_envelope = cg_envelope;
        self
    }

    pub fn notes(&mut self, notes: String) -> &mut Self {
        self.notes = Some(notes);
        self
    }

    /// Builds an aircraft.
    ///
    /// # Errors
    ///
    /// Returns an error if one of the following parameters is not configured:
    ///
    /// - `registration`
    /// - `empty_mass`
    /// - `empty_balance`
    /// - `fuel_type`
    pub fn build(&self) -> Result<Aircraft, Error> {
        Ok(Aircraft {
            registration: self
                .registration
                .clone()
                .ok_or(Error::ExpectedRegistration)?,
            // TODO: Do we wan't the unassigned type as default?
            icao_type: self.icao_type.clone().unwrap_or("ZZZZ".to_string()),
            stations: self.stations.clone(),
            empty_mass: self.empty_mass.ok_or(Error::ExpectedEmptyMass)?,
            empty_balance: self.empty_balance.ok_or(Error::ExpectedEmptyBalance)?,
            fuel_type: self.fuel_type.ok_or(Error::ExpectedFuelType)?,
            tanks: self.tanks.clone(),
            cg_envelope: CGEnvelope::new(self.cg_envelope.clone()),
            notes: self.notes.clone(),
        })
    }
}
