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

#[derive(Copy, Clone)]
pub enum Speed {
    Knots(f32),
    MeterPerSecond(f32),
}

impl Speed {
    pub fn to_kt(self) -> f32 {
        match self {
            Self::Knots(v) => v,
            Self::MeterPerSecond(v) => v * 1.943844,
        }
    }

    pub fn to_ms(self) -> f32 {
        match self {
            Self::Knots(v) => v * 0.514444,
            Self::MeterPerSecond(v) => v,
        }
    }
}
