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

use efb::{Fuel, FuelFlow, FuelType};

use super::{PyMass, PyVolume};

/// The type of fuel.
#[pyclass(module = "efb", name = "FuelType", eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum PyFuelType {
    #[pyo3(name = "DIESEL")]
    Diesel,

    #[pyo3(name = "JET_A")]
    JetA,
}

impl From<PyFuelType> for FuelType {
    fn from(fuel_type: PyFuelType) -> Self {
        match fuel_type {
            PyFuelType::Diesel => FuelType::Diesel,
            PyFuelType::JetA => FuelType::JetA,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[pyclass(module = "efb", name = "Fuel", subclass, frozen)]
#[derive(Clone)]
pub struct PyFuel {
    fuel: Fuel,
}

impl From<PyFuel> for Fuel {
    fn from(fuel: PyFuel) -> Self {
        fuel.fuel
    }
}

#[pymethods]
impl PyFuel {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.fuel))
    }
}

/// Diesel fuel.
///
/// :param quantity: The fuel quantity as volume or mass.
/// :type quantity: Volume | Mass
#[pyclass(module = "efb", name = "Diesel", extends = PyFuel)]
pub struct PyDiesel;

#[pymethods]
impl PyDiesel {
    #[new]
    pub fn new(quantity: FuelQuantity) -> (Self, PyFuel) {
        (
            Self {},
            PyFuel {
                fuel: match quantity {
                    FuelQuantity::Volume(v) => Fuel::from_volume(v.into(), &FuelType::Diesel),
                    FuelQuantity::Mass(m) => Fuel {
                        fuel_type: FuelType::Diesel,
                        mass: m.into(),
                    },
                },
            },
        )
    }
}

/// Jet-A fuel.
///
/// :param quantity: The fuel quantity as volume or mass.
/// :type quantity: Volume | Mass
#[pyclass(module = "efb", name = "JetA", extends = PyFuel)]
pub struct PyJetA;

#[pymethods]
impl PyJetA {
    #[new]
    pub fn new(quantity: FuelQuantity) -> (Self, PyFuel) {
        (
            Self {},
            PyFuel {
                fuel: match quantity {
                    FuelQuantity::Volume(v) => Fuel::from_volume(v.into(), &FuelType::JetA),
                    FuelQuantity::Mass(m) => Fuel {
                        fuel_type: FuelType::JetA,
                        mass: m.into(),
                    },
                },
            },
        )
    }
}

#[derive(FromPyObject)]
pub enum FuelQuantity {
    Volume(PyVolume),
    Mass(PyMass),
}

////////////////////////////////////////////////////////////////////////////////

#[pyclass(module = "efb", name = "FuelFlow", subclass, frozen)]
#[derive(Clone)]
pub struct PyFuelFlow {
    ff: FuelFlow,
}

impl From<PyFuelFlow> for FuelFlow {
    fn from(ff: PyFuelFlow) -> Self {
        ff.ff
    }
}

/// Fuel flow in fuel per hour.
///
/// :param Fuel fuel: The fuel that flows per hour.
#[pyclass(module = "efb", name = "PerHour", extends = PyFuelFlow)]
pub struct PyPerHour;

#[pymethods]
impl PyPerHour {
    #[new]
    pub fn new(fuel: PyFuel) -> (Self, PyFuelFlow) {
        (
            Self {},
            PyFuelFlow {
                ff: FuelFlow::PerHour(fuel.into()),
            },
        )
    }
}
