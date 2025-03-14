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

use efb::Duration;

/// A time duration.
///
/// :param int hours:
/// :param int minutes:
/// :param int seconds:
#[pyclass(module = "efb", name = "Duration", frozen)]
#[derive(Clone)]
pub struct PyDuration {
    duration: Duration,
}

impl From<PyDuration> for Duration {
    fn from(duration: PyDuration) -> Self {
        duration.duration
    }
}

#[pymethods]
impl PyDuration {
    #[new]
    pub fn new(hours: u8, minutes: u8, seconds: u8) -> Self {
        Self {
            duration: efb::Duration {
                hours,
                minutes,
                seconds,
            },
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.duration))
    }
}
