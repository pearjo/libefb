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

use efb::prelude::*;

use crate::measurements::{PyLength, PyMass};

/// A CGLimit that can be loaded with payload.
///
/// :param Mass mass:
/// :param Distance distance:
#[pyclass(module = "efb.aircraft", name = "CGLimit", frozen)]
#[derive(Clone)]
pub struct PyCGLimit {
    limit: CGLimit,
}

impl From<PyCGLimit> for CGLimit {
    fn from(limit: PyCGLimit) -> Self {
        limit.limit
    }
}

#[pymethods]
impl PyCGLimit {
    #[new]
    #[pyo3(signature = (mass, distance))]
    pub fn new(mass: PyMass, distance: PyLength) -> Self {
        Self {
            limit: CGLimit::new(mass.into(), distance.into()),
        }
    }
}
