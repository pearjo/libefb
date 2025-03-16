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

use efb::aircraft::Aircraft;

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
/// :param CGEnvelope cg_envelope:
/// :param Distance gnd_roll_takeoff:
/// :param Distance gnd_roll_takeoff_50ft_obstacle:
/// :param Distance gnd_roll_landing:
/// :param Distance gnd_roll_landing_50ft_obstacle:
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
    #[pyo3(signature = (
        registration,
        stations,
        empty_mass,
        empty_balance,
        fuel_type,
        tanks,
        cg_envelope,
        gnd_roll_takeoff,
        gnd_roll_takeoff_50ft_obstacle,
        gnd_roll_landing,
        gnd_roll_landing_50ft_obstacle,
        notes=None
    ))]
    pub fn new(
        registration: String,
        stations: Vec<PyStation>,
        empty_mass: PyMass,
        empty_balance: PyLength,
        fuel_type: PyFuelType,
        tanks: Vec<PyFuelTank>,
        cg_envelope: PyCGEnvelope,
        gnd_roll_takeoff: PyDistance,
        gnd_roll_takeoff_50ft_obstacle: PyDistance,
        gnd_roll_landing: PyDistance,
        gnd_roll_landing_50ft_obstacle: PyDistance,
        notes: Option<String>,
    ) -> Self {
        Self {
            aircraft: Aircraft {
                registration,
                stations: stations.into_iter().map(|station| station.into()).collect(),
                empty_mass: empty_mass.into(),
                empty_balance: empty_balance.into(),
                fuel_type: fuel_type.into(),
                tanks: tanks.into_iter().map(|tank| tank.into()).collect(),
                cg_envelope: cg_envelope.into(),
                gnd_roll_takeoff: gnd_roll_takeoff.into(),
                gnd_roll_takeoff_50ft_obstacle: gnd_roll_takeoff_50ft_obstacle.into(),
                gnd_roll_landing: gnd_roll_landing.into(),
                gnd_roll_landing_50ft_obstacle: gnd_roll_landing_50ft_obstacle.into(),
                notes,
            },
        }
    }
}

pub fn register_aircraft_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCGEnvelope>()?;
    m.add_class::<PyFuelTank>()?;
    m.add_class::<PyStation>()?;
    m.add_class::<PyAircraft>()?;
    Ok(())
}
