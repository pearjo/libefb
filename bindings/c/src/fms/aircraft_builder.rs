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

use std::ffi::{c_char, CStr};
use std::slice::Iter;

use efb::fp::{Aircraft, CGEnvelope, CGLimit, FuelTank, Station};
use efb::{Distance, FuelType, Mass, Volume};

#[derive(Default)]
pub struct AircraftBuilder<'a> {
    stations: Vec<Station>,
    stations_iter: Option<Iter<'a, Station>>,
    empty_mass: Option<Mass>,
    empty_balance: Option<Distance>,
    fuel_type: Option<FuelType>,
    tanks: Vec<FuelTank>,
    cg_envelope: Vec<CGLimit>,
    cg_envelope_iter: Option<Iter<'a, CGLimit>>,
}

impl<'a> AircraftBuilder<'a> {
    pub(super) fn build(&self) -> Option<Aircraft> {
        Some(Aircraft {
            stations: self.stations.clone(),
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
/// Use the builder to gradually provide all the different inputs required to
/// define an aircraft.
///
/// # Safety
///
/// The memory allocated for the builder needs to be freed by calling
/// [`efb_aircraft_builder_free`].
#[no_mangle]
pub unsafe extern "C" fn efb_aircraft_builder_new<'a>() -> Box<AircraftBuilder<'a>> {
    Box::new(AircraftBuilder::default())
}

/// Frees the aircraft builder.
#[no_mangle]
pub extern "C" fn efb_aircraft_builder_free(builder: Box<AircraftBuilder>) {
    drop(builder);
}

#[no_mangle]
pub extern "C" fn efb_aircraft_builder_stations_push(
    builder: &mut AircraftBuilder,
    arm: Distance,
    description: *const c_char,
) {
    let description = unsafe { CStr::from_ptr(description).to_str() };

    builder.stations.push(Station {
        arm,
        description: description.ok().map(String::from),
    });
}

#[no_mangle]
pub extern "C" fn efb_aircraft_builder_stations_remove(builder: &mut AircraftBuilder, at: usize) {
    builder.stations.remove(at);
}

/// Returns the first station.
///
/// To iterate over all stations, call [`efb_aircraft_builder_stations_next`]
/// until `NULL` is returned:
///
/// ```c
/// for (const EfbStation *station = efb_aircraft_builder_stations_first(builder);
///      station != NULL;
///      station = efb_aircraft_builder_stations_next(builder))
/// ```
#[no_mangle]
pub extern "C" fn efb_aircraft_builder_stations_first<'a>(
    builder: &'a mut AircraftBuilder<'a>,
) -> Option<&'a Station> {
    builder.stations_iter.insert(builder.stations.iter()).next()
}

/// Returns the next station.
///
/// When the end of the stations is reached, this function returns a null pointer.
#[no_mangle]
pub extern "C" fn efb_aircraft_builder_stations_next<'a>(
    builder: &'a mut AircraftBuilder<'a>,
) -> Option<&'a Station> {
    builder.stations_iter.as_mut().and_then(|iter| iter.next())
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
/// Pushes a new CG limit into the envelope and returns a pointer to the new
/// limit.
#[no_mangle]
pub extern "C" fn efb_aircraft_builder_cg_envelope_push<'a>(
    builder: &'a mut AircraftBuilder,
    mass: Mass,
    distance: Distance,
) -> Option<&'a CGLimit> {
    builder.cg_envelope.push(CGLimit { mass, distance });
    builder.cg_envelope.last()
}

#[no_mangle]
pub extern "C" fn efb_aircraft_builder_cg_envelope_remove(
    builder: &mut AircraftBuilder,
    at: usize,
) {
    builder.cg_envelope.remove(at);
}

/// Returns the first CG limit.
///
/// To iterate over all CG limits, call [`efb_aircraft_builder_cg_envelope_next`]
/// until `NULL` is returned:
///
/// ```c
/// for (const EfbCGLimit *limit = efb_aircraft_builder_cg_envelope_first(builder);
///      limit != NULL;
///      limit = efb_aircraft_builder_cg_envelope_next(builder))
/// ```
#[no_mangle]
pub extern "C" fn efb_aircraft_builder_cg_envelope_first<'a>(
    builder: &'a mut AircraftBuilder<'a>,
) -> Option<&'a CGLimit> {
    builder
        .cg_envelope_iter
        .insert(builder.cg_envelope.iter())
        .next()
}

/// Returns the next CG limit.
///
/// When the end of the CG limits is reached, this function returns a null pointer.
#[no_mangle]
pub extern "C" fn efb_aircraft_builder_cg_envelope_next<'a>(
    builder: &'a mut AircraftBuilder<'a>,
) -> Option<&'a CGLimit> {
    builder
        .cg_envelope_iter
        .as_mut()
        .and_then(|iter| iter.next())
}
