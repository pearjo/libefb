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

use efb::aircraft::Station;

use crate::measurements::PyLength;

/// A station that can be loaded with payload.
///
/// :param Distance arm: The distance to the aircraft's reference datum.
/// :param str | None description: A description of the station.
#[pyclass(module = "efb.aircraft", name = "Station", frozen)]
#[derive(Clone)]
pub struct PyStation {
    station: Station,
}

impl From<PyStation> for Station {
    fn from(station: PyStation) -> Self {
        station.station
    }
}

#[pymethods]
impl PyStation {
    #[new]
    #[pyo3(signature = (arm, description=None))]
    pub fn new(arm: PyLength, description: Option<String>) -> Self {
        Self {
            station: Station::new(arm.into(), description),
        }
    }
}
