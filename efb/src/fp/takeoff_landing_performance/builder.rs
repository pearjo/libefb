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

use crate::core::VerticalDistance;
use crate::measurements::{Length, Temperature};

use super::{AlteringFactor, AlteringFactors, TakeoffLandingPerformance};

pub struct TakeoffLandingPerformanceBuilder {
    table: Vec<(VerticalDistance, Temperature, Length, Length)>,
    factors: Option<AlteringFactors>,
    notes: Option<String>,
}

impl TakeoffLandingPerformanceBuilder {
    pub fn new<I>(table: I) -> Self
    where
        I: IntoIterator<Item = (VerticalDistance, Temperature, Length, Length)>,
    {
        Self {
            table: table.into_iter().collect(),
            factors: None,
            notes: None,
        }
    }

    pub fn build(&self) -> TakeoffLandingPerformance {
        TakeoffLandingPerformance::new(self.table.clone(), self.factors.clone(), self.notes.clone())
    }

    pub fn factors<I>(&mut self, factors: I) -> &mut Self
    where
        I: IntoIterator<Item = AlteringFactor>,
    {
        self.factors = Some(AlteringFactors::new(factors));
        self
    }

    pub fn notes(&mut self, notes: String) -> &mut Self {
        self.notes = Some(notes);
        self
    }
}
