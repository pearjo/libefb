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

use super::{Measurement, UnitOfMeasure};

pub enum TimeUnit {
    Seconds,
}

impl UnitOfMeasure for TimeUnit {
    fn symbol(&self) -> &str {
        "s"
    }

    fn from_si(value: f64, _: &Self) -> f64 {
        value
    }

    fn to_si(&self, value: f64) -> f64 {
        value
    }
}

pub type Time = Measurement<TimeUnit>;
