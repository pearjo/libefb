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
use efb::{Distance, FuelType, Mass, Volume};

#[derive(Default)]
pub struct AircraftBuilder {
    station_arms: Vec<Distance>,
    empty_mass: Option<Mass>,
    empty_balance: Option<Distance>,
    fuel_type: Option<FuelType>,
    tanks: Vec<FuelTank>,
    cg_envelope: Vec<(Mass, Distance)>,
}

impl AircraftBuilder {
    pub(super) fn build(&self) -> Option<Aircraft> {
        Some(Aircraft {
            station_arms: self.station_arms.clone(),
            empty_mass: self.empty_mass?,
            empty_balance: self.empty_balance?,
            fuel_type: self.fuel_type?,
            tanks: self.tanks.clone(),
            cg_envelope: CGEnvelope::new(self.cg_envelope.clone()),
        })
    }
}

/// Returns a new aircraft builder.
///
/// # Safety
///
/// The memory allocated for the builder needs to be freed by calling
/// [`efb_aircraft_builder_free`].
#[no_mangle]
pub unsafe extern "C" fn efb_aircraft_builder_new() -> Box<AircraftBuilder> {
    Box::new(AircraftBuilder::default())
}

/// Frees the aircraft builder.
#[no_mangle]
pub extern "C" fn efb_aircraft_builder_free(builder: Box<AircraftBuilder>) {
    drop(builder);
}

#[no_mangle]
pub extern "C" fn efb_aircraft_builder_station_arms_push(
    builder: &mut AircraftBuilder,
    distance: Distance,
) {
    builder.station_arms.push(distance);
}

#[no_mangle]
pub extern "C" fn efb_aircraft_builder_station_arms_remove(
    builder: &mut AircraftBuilder,
    i: usize,
) {
    builder.station_arms.remove(i);
}

#[no_mangle]
pub extern "C" fn efb_aircraft_builder_station_arms_edit(
    builder: &mut AircraftBuilder,
    distance: Distance,
    i: usize,
) {
    builder.station_arms.remove(i);
    builder.station_arms.insert(i, distance);
}

#[no_mangle]
pub extern "C" fn efb_aircraft_builder_empty_mass(builder: &mut AircraftBuilder, mass: Mass) {
    let _ = builder.empty_mass.insert(mass);
}

#[no_mangle]
pub extern "C" fn efb_aircraft_builder_empty_balance(
    builder: &mut AircraftBuilder,
    distance: Distance,
) {
    let _ = builder.empty_balance.insert(distance);
}

#[no_mangle]
pub extern "C" fn efb_aircraft_builder_fuel_type(
    builder: &mut AircraftBuilder,
    fuel_type: FuelType,
) {
    let _ = builder.fuel_type.insert(fuel_type);
}

#[no_mangle]
pub extern "C" fn efb_aircraft_builder_tanks_push(
    builder: &mut AircraftBuilder,
    capacity: Volume,
    arm: Distance,
) {
    builder.tanks.push(FuelTank { capacity, arm });
}

#[no_mangle]
pub extern "C" fn efb_aircraft_builder_tanks_remove(builder: &mut AircraftBuilder, i: usize) {
    builder.tanks.remove(i);
}

#[no_mangle]
pub extern "C" fn efb_aircraft_builder_tanks_edit(
    builder: &mut AircraftBuilder,
    capacity: Volume,
    arm: Distance,
    i: usize,
) {
    builder.tanks.remove(i);
    builder.tanks.insert(i, FuelTank { capacity, arm });
}

#[no_mangle]
pub extern "C" fn efb_aircraft_builder_cg_envelope_push(
    builder: &mut AircraftBuilder,
    mass: Mass,
    distance: Distance,
) {
    builder.cg_envelope.push((mass, distance));
}

#[no_mangle]
pub extern "C" fn efb_aircraft_builder_cg_envelope_remove(builder: &mut AircraftBuilder, i: usize) {
    builder.cg_envelope.remove(i);
}

#[no_mangle]
pub extern "C" fn efb_aircraft_builder_cg_envelope_edit(
    builder: &mut AircraftBuilder,
    mass: Mass,
    distance: Distance,
    i: usize,
) {
    builder.cg_envelope.remove(i);
    builder.cg_envelope.insert(i, (mass, distance));
}
