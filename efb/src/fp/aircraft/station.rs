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

use crate::{Distance, Mass};

/// A mass at a defined point within an aircraft.
///
/// An [`Aircraft`] can be loaded with _stations_ that hold a mass at a defined
/// distance from the aircraft's reference datum. The mass may differ from when
/// on ramp to after landing (burned fuel or a skydiver).
///
/// [`Aircraft`]: crate::fp::Aircraft
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Station {
    /// The mass on ramp.
    pub on_ramp: Mass,

    /// The mass after landing.
    pub after_landing: Mass,

    /// The lever's arm from the reference datum.
    pub arm: Distance,
}
