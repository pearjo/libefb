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

use pyo3::prelude::*;

use efb::fms::FlightPlanningBuilder;

use crate::aircraft::PyAircraft;
use crate::core::PyFuel;
use crate::fp::{PyFuelPolicy, PyPerformance, PyReserve};
use crate::measurements::PyMass;

/// :param Aircraft aircraft:
/// :param Mass mass:
/// :param FuelPolicy policy:
/// :param Fuel taxi: The fuel that should be planned for taxiing.
/// :param Reserve reserve:
/// :param Performance perf:
#[pyclass(module = "efb", name = "FlightPlanningBuilder")]
#[derive(Clone)]
pub struct PyFlightPlanningBuilder {
    builder: FlightPlanningBuilder,
}

impl From<PyFlightPlanningBuilder> for FlightPlanningBuilder {
    fn from(builder: PyFlightPlanningBuilder) -> Self {
        builder.builder
    }
}

#[pymethods]
impl PyFlightPlanningBuilder {
    #[new]
    pub fn new(
        aircraft: PyAircraft,
        mass: Vec<PyMass>,
        policy: PyFuelPolicy,
        taxi: PyFuel,
        reserve: PyReserve,
        perf: PyPerformance,
    ) -> Self {
        let mut builder = FlightPlanningBuilder::new();

        builder
            .set_aircraft(aircraft.into())
            .set_mass(mass.into_iter().map(|mass| mass.into()).collect())
            .set_policy(policy.into())
            .set_taxi(taxi.into())
            .set_reserve(reserve.into())
            .set_perf(perf.into());

        Self { builder }
    }
}
