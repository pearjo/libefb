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

use efb::VerticalDistance;

#[pyclass(module = "efb", name = "VerticalDistance", subclass)]
#[derive(Clone)]
pub struct PyVerticalDistance {
    vd: VerticalDistance,
}

impl From<PyVerticalDistance> for VerticalDistance {
    fn from(vd: PyVerticalDistance) -> Self {
        vd.vd
    }
}

#[pymethods]
impl PyVerticalDistance {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.vd))
    }
}

/// Vertical distance Above Ground Level (AGL).
///
/// :param int ft: The distance above ground level in feet.
#[pyclass(module ="efb", name = "AGL", extends = PyVerticalDistance)]
pub struct PyAgl;

#[pymethods]
impl PyAgl {
    #[new]
    pub fn new(ft: u16) -> (Self, PyVerticalDistance) {
        (
            Self {},
            PyVerticalDistance {
                vd: VerticalDistance::Agl(ft),
            },
        )
    }
}

/// Altitude in reference to local air pressure.
///
/// :param int ft: The altitude in feet.
#[pyclass(module ="efb", name = "Altitude", extends = PyVerticalDistance)]
pub struct PyAltitude;

#[pymethods]
impl PyAltitude {
    #[new]
    pub fn new(ft: u16) -> (Self, PyVerticalDistance) {
        (
            Self {},
            PyVerticalDistance {
                vd: VerticalDistance::Altitude(ft),
            },
        )
    }
}

/// Flight Level (FL).
///
/// :param int fl: The flight level in hundreds of feet.
#[pyclass(module ="efb", name = "FL", extends = PyVerticalDistance)]
pub struct PyFl;

#[pymethods]
impl PyFl {
    #[new]
    pub fn new(fl: u16) -> (Self, PyVerticalDistance) {
        (
            Self {},
            PyVerticalDistance {
                vd: VerticalDistance::Fl(fl),
            },
        )
    }
}

/// Ground level.
#[pyclass(module ="efb", name = "GND", extends = PyVerticalDistance)]
pub struct PyGnd;

#[pymethods]
impl PyGnd {
    #[new]
    pub fn new() -> (Self, PyVerticalDistance) {
        (
            Self {},
            PyVerticalDistance {
                vd: VerticalDistance::Gnd,
            },
        )
    }
}

/// Vertical distance above Mean Sea Level (MSL).
///
/// :param int ft: The MSL in feet.
#[pyclass(module ="efb", name = "MSL", extends = PyVerticalDistance)]
pub struct PyMsl;

#[pymethods]
impl PyMsl {
    #[new]
    pub fn new(ft: u16) -> (Self, PyVerticalDistance) {
        (
            Self {},
            PyVerticalDistance {
                vd: VerticalDistance::Msl(ft),
            },
        )
    }
}

/// Unlimited vertical distance.
#[pyclass(module ="efb", name = "Unlimited", extends = PyVerticalDistance)]
pub struct PyUnlimited;

#[pymethods]
impl PyUnlimited {
    #[new]
    pub fn new() -> (Self, PyVerticalDistance) {
        (
            Self {},
            PyVerticalDistance {
                vd: VerticalDistance::Unlimited,
            },
        )
    }
}
