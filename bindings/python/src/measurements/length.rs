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

use efb::measurements::Length;

/// A distance measurement.
#[pyclass(module = "efb", name = "Length", subclass, frozen)]
#[derive(Clone)]
pub struct PyLength {
    distance: Length,
}

impl From<PyLength> for Length {
    fn from(distance: PyLength) -> Self {
        distance.distance
    }
}

#[pymethods]
impl PyLength {
    fn __repr__(&self) -> PyResult<String> {
        PyResult::Ok(format!("{}", self.distance))
    }
}

/// Length in meter.
///
/// :param m: The distance in meter.
/// :type m: float
#[pyclass(module = "efb", name = "Meter", extends = PyLength)]
pub struct PyMeter;

#[pymethods]
impl PyMeter {
    #[new]
    pub fn new(m: f32) -> (Self, PyLength) {
        (
            PyMeter {},
            PyLength {
                distance: Length::m(m),
            },
        )
    }
}

/// Length in feet.
///
/// :param ft: The distance in feet.
/// :type ft: float
#[pyclass(module = "efb", name = "Feet", extends = PyLength)]
pub struct PyFeet;

#[pymethods]
impl PyFeet {
    #[new]
    pub fn new(ft: f32) -> (Self, PyLength) {
        (
            PyFeet {},
            PyLength {
                distance: Length::ft(ft),
            },
        )
    }
}

/// Length in nautical miles.
///
/// :param nm: The distance in nautical miles.
/// :type nm: float
#[pyclass(module = "efb", name = "NauticalMiles", extends = PyLength)]
pub struct PyNauticalMiles;

#[pymethods]
impl PyNauticalMiles {
    #[new]
    fn new(nm: f32) -> (Self, PyLength) {
        (
            PyNauticalMiles {},
            PyLength {
                distance: Length::nm(nm),
            },
        )
    }
}
