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

#[derive(Debug, PartialEq)]
pub enum CustArea {
    Blank,
    Custom,
    PreferredRoute,
    AFR,
    CAN,
    EEU,
    EUR,
    LAM,
    MES,
    PAC,
    SAM,
    SPA,
    USA,
}

impl Field for CustArea {}

impl FromStr for CustArea {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match &s[1..4] {
            "AFR" => Self::AFR,
            "CAN" => Self::CAN,
            "EEU" => Self::EEU,
            "EUR" => Self::EUR,
            "LAM" => Self::LAM,
            "MES" => Self::MES,
            "PAC" => Self::PAC,
            "SAM" => Self::SAM,
            "SPA" => Self::SPA,
            "USA" => Self::USA,
            "PDR" => Self::PreferredRoute,
            "   " => Self::Blank,
            _ => Self::Custom,
        })
    }
}
