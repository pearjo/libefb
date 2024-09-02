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

use super::{Field, FieldError};
use std::str::FromStr;

#[derive(Debug)]
pub struct FileRecordNumber(u32);

impl Field for FileRecordNumber {}

impl PartialEq<u32> for FileRecordNumber {
    fn eq(&self, other: &u32) -> bool {
        &self.0 == other
    }
}

impl FromStr for FileRecordNumber {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s[123..128].parse::<u32>() {
            Ok(frn) => Ok(Self(frn)),
            _ => Err(FieldError::NotANumber),
        }
    }
}
