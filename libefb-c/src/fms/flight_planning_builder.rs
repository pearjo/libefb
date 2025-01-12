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

use efb::fms::FlightPlanningBuilder;
use efb::fp::{FuelPolicy, Performance, Reserve};
use efb::{Fuel, FuelFlow, Mass, Speed, VerticalDistance};

use super::AircraftBuilder;

/// The aircraft performance at a specific level and configuration.
#[repr(C)]
pub struct PerformanceAtLevel {
    /// The true airspeed.
    pub tas: Speed,

    /// The fuel flow at the level.
    pub ff: FuelFlow,
}

/// Returns a new flight planning builder.
///
/// # Safety
///
/// The memory allocated for the builder needs to be freed by calling
/// [`efb_flight_planning_builder_free`].
#[no_mangle]
pub unsafe extern "C" fn efb_flight_planning_builder_new() -> Box<FlightPlanningBuilder> {
    Box::new(FlightPlanningBuilder::default())
}

/// Frees the flight planning builder.
#[no_mangle]
pub extern "C" fn efb_flight_planning_builder_free(builder: Box<FlightPlanningBuilder>) {
    drop(builder);
}

#[no_mangle]
pub extern "C" fn efb_flight_planning_builder_set_aircraft(
    builder: &mut FlightPlanningBuilder,
    aircraft_builder: &AircraftBuilder,
) {
    if let Some(ac) = aircraft_builder.build() {
        builder.set_aircraft(ac);
    }
}

#[no_mangle]
pub extern "C" fn efb_flight_planning_builder_mass_push(
    builder: &mut FlightPlanningBuilder,
    mass: Mass,
) {
    if let Some(v) = builder.mass() {
        v.push(mass);
    }
}

#[no_mangle]
pub extern "C" fn efb_flight_planning_builder_mass_remove(
    builder: &mut FlightPlanningBuilder,
    i: usize,
) {
    if let Some(v) = builder.mass() {
        v.remove(i);
    }
}

#[no_mangle]
pub extern "C" fn efb_flight_planning_builder_mass_edit(
    builder: &mut FlightPlanningBuilder,
    mass: Mass,
    i: usize,
) {
    if let Some(v) = builder.mass() {
        v.remove(i);
        v.insert(i, mass);
    };
}

#[no_mangle]
pub extern "C" fn efb_flight_planning_builder_set_policy(
    builder: &mut FlightPlanningBuilder,
    policy: FuelPolicy,
) {
    builder.set_policy(policy);
}

#[no_mangle]
pub extern "C" fn efb_flight_planning_builder_set_taxi(
    builder: &mut FlightPlanningBuilder,
    taxi: Fuel,
) {
    builder.set_taxi(taxi);
}

#[no_mangle]
pub extern "C" fn efb_flight_planning_builder_set_reserve(
    builder: &mut FlightPlanningBuilder,
    reserve: Reserve,
) {
    builder.set_reserve(reserve);
}

#[no_mangle]
pub extern "C" fn efb_flight_planning_builder_set_perf(
    builder: &mut FlightPlanningBuilder,
    perf: extern "C" fn(&VerticalDistance) -> PerformanceAtLevel,
    ceiling: VerticalDistance,
) {
    builder.set_perf(Performance::from(
        |vd| {
            let at_level = perf(vd);
            (at_level.tas, at_level.ff)
        },
        ceiling,
    ));
}
