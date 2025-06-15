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

use efb::aircraft::FuelTank;

use crate::measurements::{PyLength, PyVolume};

/// An aircraft's fuel tank.
///
/// :param Volume capacity: The tank's capacity.
/// :param Distance arm: The tank's distance to the aircraft's reference datum.
#[pyclass(module = "efb.aircraft", name = "FuelTank", frozen)]
#[derive(Clone)]
pub struct PyFuelTank {
    tank: FuelTank,
}

impl From<PyFuelTank> for FuelTank {
    fn from(tank: PyFuelTank) -> Self {
        tank.tank
    }
}

#[pymethods]
impl PyFuelTank {
    #[new]
    pub fn new(capacity: PyVolume, arm: PyLength) -> Self {
        Self {
            tank: FuelTank::new(capacity.into(), arm.into()),
        }
    }
}
