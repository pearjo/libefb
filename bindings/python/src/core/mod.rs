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

mod fuel;
mod vertical_distance;

pub use fuel::*;
pub use vertical_distance::*;

pub(super) fn register_core_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // fuel
    m.add_class::<PyFuelType>()?;
    m.add_class::<PyFuel>()?;
    m.add_class::<PyDiesel>()?;
    m.add_class::<PyJetA>()?;
    m.add_class::<PyFuelFlow>()?;
    m.add_class::<PyPerHour>()?;
    // vertical distance
    m.add_class::<PyVerticalDistance>()?;
    m.add_class::<PyAgl>()?;
    m.add_class::<PyAltitude>()?;
    m.add_class::<PyFl>()?;
    m.add_class::<PyGnd>()?;
    m.add_class::<PyMsl>()?;
    m.add_class::<PyUnlimited>()?;

    Ok(())
}
