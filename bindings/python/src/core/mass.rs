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

use efb::Mass;

#[pyclass(module = "efb", name = "Mass", subclass, frozen)]
#[derive(Clone)]
pub struct PyMass {
    mass: Mass,
}

impl From<PyMass> for Mass {
    fn from(mass: PyMass) -> Self {
        mass.mass
    }
}

#[pymethods]
impl PyMass {
    fn __repr__(&self) -> PyResult<String> {
        PyResult::Ok(format!("{}", self.mass))
    }
}

/// Mass in kilogram.
///
/// :param float kg: The mass in kilogram.
#[pyclass(module = "efb", name = "Kilogram", extends = PyMass)]
pub struct PyKilogram;

#[pymethods]
impl PyKilogram {
    #[new]
    pub fn new(kg: f32) -> (Self, PyMass) {
        (
            Self {},
            PyMass {
                mass: Mass::Kilogram(kg),
            },
        )
    }
}
