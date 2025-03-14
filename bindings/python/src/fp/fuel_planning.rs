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

use efb::fp::{FuelPolicy, Reserve};

use crate::core::{PyDuration, PyFuel};

#[pyclass(module = "efb", name = "FuelPolicy", subclass)]
#[derive(Clone)]
pub struct PyFuelPolicy {
    policy: FuelPolicy,
}

impl From<PyFuelPolicy> for FuelPolicy {
    fn from(policy: PyFuelPolicy) -> Self {
        policy.policy
    }
}

#[pyclass(module = "efb", name = "MinimumFuel", extends = PyFuelPolicy)]
pub struct PyMinimumFuel;

#[pymethods]
impl PyMinimumFuel {
    #[new]
    pub fn new() -> (Self, PyFuelPolicy) {
        (
            Self {},
            PyFuelPolicy {
                policy: FuelPolicy::MinimumFuel,
            },
        )
    }
}

#[pyclass(module = "efb", name = "MaximumFuel", extends = PyFuelPolicy)]
pub struct PyMaximumFuel;

#[pymethods]
impl PyMaximumFuel {
    #[new]
    pub fn new() -> (Self, PyFuelPolicy) {
        (
            Self {},
            PyFuelPolicy {
                policy: FuelPolicy::MaximumFuel,
            },
        )
    }
}

/// :param Fuel fuel: The fuel that should be planned with.
#[pyclass(module = "efb", name = "ManualFuel", extends = PyFuelPolicy)]
pub struct PyManualFuel;

#[pymethods]
impl PyManualFuel {
    #[new]
    pub fn new(fuel: PyFuel) -> (Self, PyFuelPolicy) {
        (
            Self {},
            PyFuelPolicy {
                policy: FuelPolicy::ManualFuel(fuel.into()),
            },
        )
    }
}

/// :param Fuel fuel: The remaining fuel at landing.
#[pyclass(module = "efb", name = "FuelAtLanding", extends = PyFuelPolicy)]
pub struct PyFuelAtLanding;

#[pymethods]
impl PyFuelAtLanding {
    #[new]
    pub fn new(fuel: PyFuel) -> (Self, PyFuelPolicy) {
        (
            Self {},
            PyFuelPolicy {
                policy: FuelPolicy::FuelAtLanding(fuel.into()),
            },
        )
    }
}

/// :param Fuel fuel: The fuel that should be available as extra.
#[pyclass(module = "efb", name = "ExtraFuel", extends = PyFuelPolicy)]
pub struct PyExtraFuel;

#[pymethods]
impl PyExtraFuel {
    #[new]
    pub fn new(fuel: PyFuel) -> (Self, PyFuelPolicy) {
        (
            Self {},
            PyFuelPolicy {
                policy: FuelPolicy::ExtraFuel(fuel.into()),
            },
        )
    }
}

////////////////////////////////////////////////////////////////////////////////

/// A fuel reserve.
#[pyclass(module = "efb", name = "Reserve", subclass)]
#[derive(Clone)]
pub struct PyReserve {
    reserve: Reserve,
}

impl From<PyReserve> for Reserve {
    fn from(reserve: PyReserve) -> Self {
        reserve.reserve
    }
}

/// :param Duration duration: The duration for which a reserve should be available.
#[pyclass(module = "efb", name = "ManualReserve", extends = PyReserve)]
pub struct PyManualReserve;

#[pymethods]
impl PyManualReserve {
    #[new]
    pub fn new(duration: PyDuration) -> (Self, PyReserve) {
        (
            Self {},
            PyReserve {
                reserve: Reserve::Manual(duration.into()),
            },
        )
    }
}
