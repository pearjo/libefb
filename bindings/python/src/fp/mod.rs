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

mod fuel_planning;
pub(crate) use fuel_planning::*;

mod perf;
pub(crate) use perf::*;

pub(super) fn register_fp_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // fuel planning
    m.add_class::<PyFuelPolicy>()?;
    m.add_class::<PyMinimumFuel>()?;
    m.add_class::<PyMaximumFuel>()?;
    m.add_class::<PyManualFuel>()?;
    m.add_class::<PyFuelAtLanding>()?;
    m.add_class::<PyExtraFuel>()?;
    m.add_class::<PyReserve>()?;
    m.add_class::<PyManualReserve>()?;
    // performance
    m.add_class::<PyPerformance>()?;

    Ok(())
}
