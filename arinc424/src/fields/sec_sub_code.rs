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
pub enum SecCode {
    MORA,
    Navaid,
    Enroute,
    Heliport,
    Airport,
    CompanyRoute,
    Table,
    Airspace,
}

impl Field for SecCode {}

impl FromStr for SecCode {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[4..5] {
            "A" => Ok(Self::MORA),
            "D" => Ok(Self::Navaid),
            "E" => Ok(Self::Enroute),
            "H" => Ok(Self::Heliport),
            "P" => Ok(Self::Airport),
            "R" => Ok(Self::CompanyRoute),
            "T" => Ok(Self::Table),
            "U" => Ok(Self::Airspace),
            _ => Err(FieldError::InvalidValue("unkown SEC CODE")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SubCode<const I: usize> {
    // MORA
    GridMORA,
    // Navaid
    VHFNavaid,
    NDBNavaid,
    // Enroute
    Waypoint,
    // Heliport,
    Pad,
    // Airport
    ReferencePoint,
    Gate,
    // Heliport, Airport
    TerminalWaypoint,
    MSA,
    // CompanyRoute
    CompanyRoute,
    AlternateRecord,
    // Tables
    CruisingTable,
    // Airspace
    ControlledAirspace,
}

impl<const I: usize> Field for SubCode<I> {}

macro_rules! sub_code_error {
    ($sub_code:expr) => {
        Err(FieldError::InvalidValue(concat!(
            "invalid SEC CODE for SUB CODE: ",
            $sub_code
        )))
    };
}

impl<const I: usize> FromStr for SubCode<I> {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sec_code: SecCode = s.parse()?;

        match &s[I..I + 1] {
            " " => match sec_code {
                SecCode::Navaid => Ok(Self::VHFNavaid),
                SecCode::CompanyRoute => Ok(Self::CompanyRoute),
                _ => sub_code_error!("BLANK"),
            },
            "A" => match sec_code {
                SecCode::Enroute => Ok(Self::Waypoint),
                SecCode::Heliport => Ok(Self::Pad),
                SecCode::Airport => Ok(Self::ReferencePoint),
                SecCode::CompanyRoute => Ok(Self::AlternateRecord),
                _ => sub_code_error!("A"),
            },
            "B" => match sec_code {
                SecCode::Navaid => Ok(Self::NDBNavaid),
                SecCode::Airport => Ok(Self::Gate),
                _ => sub_code_error!("B"),
            },
            "C" => match sec_code {
                SecCode::Heliport | SecCode::Airport => Ok(Self::TerminalWaypoint),
                SecCode::Table => Ok(Self::CruisingTable),
                SecCode::Airspace => Ok(Self::ControlledAirspace),
                _ => sub_code_error!("C"),
            },
            "S" => match sec_code {
                SecCode::MORA => Ok(Self::GridMORA),
                SecCode::Heliport | SecCode::Airport => Ok(Self::MSA),
                _ => sub_code_error!("S"),
            },
            _ => todo!("implement missing SUB CODE D..Z"),
        }
    }
}
