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

use efb::fp::{Performance, PerformanceTableRow};

use crate::core::{PyFuelFlow, PyVerticalDistance};
use crate::measurements::PySpeed;

#[derive(FromPyObject)]
pub struct PyPerformanceTableRow(PyVerticalDistance, PySpeed, PyFuelFlow);

/// A aircraft's performance table.
///
/// :param table: A list of performance values for different altitudes.
/// :type table: list[tuple(Altitude, Speed, FuelFlow)]
#[pyclass(module = "efb.fp", name = "Performance", frozen)]
#[derive(Clone)]
pub struct PyPerformance {
    perf: Performance,
}

impl From<PyPerformance> for Performance {
    fn from(perf: PyPerformance) -> Self {
        perf.perf
    }
}

#[pymethods]
impl PyPerformance {
    #[new]
    pub fn new(table: Vec<PyPerformanceTableRow>) -> Self {
        Self {
            perf: Performance::new(
                table
                    .into_iter()
                    .map(|row| PerformanceTableRow {
                        level: row.0.into(),
                        tas: row.1.into(),
                        ff: row.2.into(),
                    })
                    .collect(),
            ),
        }
    }
}
