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

mod aircraft;
use aircraft::register_aircraft_module;

mod core;
use core::register_core_module;

mod fp;
use fp::register_fp_module;

mod fms;
use fms::register_fms_module;

#[pymodule]
fn efb(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_core_module(m)?;
    register_fms_module(m)?;
    register_aircraft_module(m)?;
    register_fp_module(m)?;
    Ok(())
}
