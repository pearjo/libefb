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

use pyo3::prelude::*;

use efb::prelude::*;

use crate::core::PyFuelType;
use crate::measurements::{PyLength, PyMass};

mod cg_envelope;
use cg_envelope::*;

mod fuel_tank;
use fuel_tank::*;

mod station;
use station::*;

/// :param str registration: The registration code.
/// :param list[Station] stations: The loadable stations.
/// :param Mass empty_mass: The aircraft's empty mass.
/// :param Distance empty_balance: The aircraft's empty balance.
/// :param FuelType fuel_type:
/// :param list[FuelTank] tanks:
/// :param list[CGLimit] cg_envelope:
/// :param str | None notes: Some notes to make about the aircraft.
#[pyclass(module = "efb.aircraft", name = "Aircraft", frozen)]
#[derive(Clone)]
pub struct PyAircraft {
    aircraft: Aircraft,
}

impl From<PyAircraft> for Aircraft {
    fn from(aircraft: PyAircraft) -> Self {
        aircraft.aircraft
    }
}

#[pymethods]
impl PyAircraft {
    #[new]
    #[pyo3(signature = (registration, stations, empty_mass, empty_balance, fuel_type, tanks, cg_envelope, notes=None))]
    pub fn new(
        registration: String,
        stations: Vec<PyStation>,
        empty_mass: PyMass,
        empty_balance: PyLength,
        fuel_type: PyFuelType,
        tanks: Vec<PyFuelTank>,
        cg_envelope: Vec<PyCGLimit>,
        notes: Option<String>,
    ) -> Self {
        let mut builder = Aircraft::builder();

        builder
            .registration(registration)
            .stations(stations.into_iter().map(|station| station.into()).collect())
            .empty_mass(empty_mass.into())
            .empty_balance(empty_balance.into())
            .fuel_type(fuel_type.into())
            .tanks(tanks.into_iter().map(|tank| tank.into()).collect())
            .cg_envelope(cg_envelope.into_iter().map(|limit| limit.into()).collect());

        if let Some(notes) = notes {
            builder.notes(notes);
        }

        Self {
            aircraft: builder.build().expect("aircraft should build"),
        }
    }
}

pub fn register_aircraft_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCGLimit>()?;
    m.add_class::<PyFuelTank>()?;
    m.add_class::<PyStation>()?;
    m.add_class::<PyAircraft>()?;
    Ok(())
}
