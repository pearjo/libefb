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

use std::str::FromStr;

use super::{Field, FieldError};

pub struct RunwayId<const I: usize> {
    pub designator: String,
}

impl<const I: usize> Field for RunwayId<I> {}

impl<const I: usize> FromStr for RunwayId<I> {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[I + 4..I + 5] {
            " " | "C" | "L" | "R" | "W" | "G" | "U" => {
                let designator = s[I + 2..I + 5].trim_end().to_string();
                Ok(Self { designator })
            }
            _ => Err(FieldError::UnexpectedChar("unexpected designation suffix")),
        }
    }
}
