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

mod duration;
mod length;
mod mass;
mod speed;
mod volume;

pub use duration::*;
pub use length::*;
pub use mass::*;
pub use speed::*;
pub use volume::*;

pub(super) fn register_measurements_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // length
    m.add_class::<PyLength>()?;
    m.add_class::<PyMeter>()?;
    m.add_class::<PyNauticalMiles>()?;
    // duration
    m.add_class::<PyDuration>()?;
    // mass
    m.add_class::<PyMass>()?;
    m.add_class::<PyKilogram>()?;
    // volume
    m.add_class::<PyVolume>()?;
    m.add_class::<PyLiter>()?;
    // speed
    m.add_class::<PySpeed>()?;
    m.add_class::<PyKnots>()?;
    m.add_class::<PyMeterPerSecond>()?;
    m.add_class::<PyMach>()?;

    Ok(())
}
