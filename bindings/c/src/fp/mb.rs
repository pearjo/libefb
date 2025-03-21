// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 Joe Pearson
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

use efb::fp::MassAndBalance;
use efb::measurements::{Length, Mass};

#[no_mangle]
pub extern "C" fn efb_mass_and_balance_mass_on_ramp(mb: &MassAndBalance) -> &Mass {
    mb.mass_on_ramp()
}

#[no_mangle]
pub extern "C" fn efb_mass_and_balance_mass_after_landing(mb: &MassAndBalance) -> &Mass {
    mb.mass_after_landing()
}

#[no_mangle]
pub extern "C" fn efb_mass_and_balance_balance_on_ramp(mb: &MassAndBalance) -> &Length {
    mb.balance_on_ramp()
}

#[no_mangle]
pub extern "C" fn efb_mass_and_balance_balance_after_landing(mb: &MassAndBalance) -> &Length {
    mb.balance_after_landing()
}
