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

use efb::Distance;

/// A distance measurement.
#[pyclass(module = "efb", name = "Distance", subclass, frozen)]
#[derive(Clone)]
pub struct PyDistance {
    distance: Distance,
}

impl From<PyDistance> for Distance {
    fn from(distance: PyDistance) -> Self {
        distance.distance
    }
}

#[pymethods]
impl PyDistance {
    fn __repr__(&self) -> PyResult<String> {
        PyResult::Ok(format!("{}", self.distance))
    }
}

/// Distance in meter.
///
/// :param m: The distance in meter.
/// :type m: float
#[pyclass(module = "efb", name = "Meter", extends = PyDistance)]
pub struct PyMeter;

#[pymethods]
impl PyMeter {
    #[new]
    pub fn new(m: f32) -> (Self, PyDistance) {
        (
            PyMeter {},
            PyDistance {
                distance: Distance::Meter(m),
            },
        )
    }
}

/// Distance in nautical miles.
///
/// :param nm: The distance in nautical miles.
/// :type nm: float
#[pyclass(module = "efb", name = "NauticalMiles", extends = PyDistance)]
pub struct PyNauticalMiles;

#[pymethods]
impl PyNauticalMiles {
    #[new]
    fn new(nm: f32) -> (Self, PyDistance) {
        (
            PyNauticalMiles {},
            PyDistance {
                distance: Distance::NauticalMiles(nm),
            },
        )
    }
}
