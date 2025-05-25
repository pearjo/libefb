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

#[derive(Debug, PartialEq)]
pub enum Source<const I: usize> {
    GovernmentSources,
    OtherSources,
    BearingInTrue,
}

impl<const I: usize> Field for Source<I> {}

impl<const I: usize> FromStr for Source<I> {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[I..I + 1] {
            "Y" => Ok(Self::GovernmentSources),
            "N" | " " => Ok(Self::OtherSources),
            "T" => Ok(Self::BearingInTrue),
            _ => Err(FieldError::UnexpectedChar("unexpected source identifier")),
        }
    }
}
