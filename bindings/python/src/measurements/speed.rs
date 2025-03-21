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

use efb::measurements::Speed;

#[pyclass(module = "efb", name = "Speed", subclass, frozen)]
#[derive(Clone)]
pub struct PySpeed {
    speed: Speed,
}

impl From<PySpeed> for Speed {
    fn from(speed: PySpeed) -> Self {
        speed.speed
    }
}

#[pymethods]
impl PySpeed {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.speed))
    }
}

/// Speed in knots.
///
/// :param float kt:
#[pyclass(module = "efb", name = "Knots", extends = PySpeed)]
pub struct PyKnots;

#[pymethods]
impl PyKnots {
    #[new]
    pub fn new(kt: f32) -> (Self, PySpeed) {
        (
            Self {},
            PySpeed {
                speed: Speed::kt(kt),
            },
        )
    }
}

/// Speed in meter per second.
///
/// :param float mps:
#[pyclass(module = "efb", name = "MeterPerSecond", extends = PySpeed)]
pub struct PyMeterPerSecond;

#[pymethods]
impl PyMeterPerSecond {
    #[new]
    pub fn new(mps: f32) -> (Self, PySpeed) {
        (
            Self {},
            PySpeed {
                speed: Speed::mps(mps),
            },
        )
    }
}

/// A speed in mach.
///
/// :param float mach:
#[pyclass(module = "efb", name = "Mach", extends = PySpeed)]
pub struct PyMach;

#[pymethods]
impl PyMach {
    #[new]
    pub fn new(mach: f32) -> (Self, PySpeed) {
        (
            Self {},
            PySpeed {
                speed: Speed::mach(mach),
            },
        )
    }
}
