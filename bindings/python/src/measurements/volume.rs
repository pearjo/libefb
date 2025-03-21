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

use efb::measurements::Volume;

#[pyclass(module = "efb", name = "Volume", subclass, frozen)]
#[derive(Clone)]
pub struct PyVolume {
    volume: Volume,
}

impl From<PyVolume> for Volume {
    fn from(volume: PyVolume) -> Self {
        volume.volume
    }
}

#[pymethods]
impl PyVolume {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.volume))
    }
}

/// Volume in liter.
///
/// :param float l: The volume in liter.
#[pyclass(module = "efb", name = "Liter", extends = PyVolume)]
pub struct PyLiter;

#[pymethods]
impl PyLiter {
    #[new]
    fn new(l: f32) -> (Self, PyVolume) {
        (
            Self {},
            PyVolume {
                volume: Volume::l(l),
            },
        )
    }
}
