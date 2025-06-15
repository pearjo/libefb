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

use efb::aircraft::{CGEnvelope, CGLimit};

use crate::measurements::{PyLength, PyMass};

#[derive(FromPyObject)]
pub struct PyCGLimit(PyMass, PyLength);

/// A Center of Gravity (CG) envelope.
///
/// :param limits: A list of mass and distance tuple pairs that define the
///     limits of the envelope.
/// :type limits: list[tuple(Mass, Distance)]
#[pyclass(module = "efb.aircraft", name = "CGEnvelope", frozen)]
#[derive(Clone)]
pub struct PyCGEnvelope {
    envelope: CGEnvelope,
}

impl From<PyCGEnvelope> for CGEnvelope {
    fn from(envelope: PyCGEnvelope) -> Self {
        envelope.envelope
    }
}

#[pymethods]
impl PyCGEnvelope {
    #[new]
    pub fn new(limits: Vec<PyCGLimit>) -> Self {
        Self {
            envelope: CGEnvelope::new(
                limits
                    .into_iter()
                    .map(|limit| CGLimit::new(limit.0.into(), limit.1.into()))
                    .collect(),
            ),
        }
    }
}
