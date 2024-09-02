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

use std::cmp::PartialEq;
use std::fmt;
use std::str;

mod cont_nr;
mod coordinate;
mod cust_area;
mod cycle;
mod datum;
mod fix_ident;
mod frn;
mod icao_code;
mod mag_var;
mod name_desc;
mod name_ind;
mod record_type;
mod regn_code;
mod sec_sub_code;
mod waypoint_type;
mod waypoint_usage;

pub use cont_nr::ContNr;
pub use coordinate::{CardinalDirection, Latitude, Longitude};
pub use cust_area::CustArea;
pub use cycle::Cycle;
pub use datum::Datum;
pub use fix_ident::FixIdent;
pub use frn::FileRecordNumber;
pub use icao_code::IcaoCode;
pub use mag_var::MagVar;
pub use name_desc::NameDesc;
pub use name_ind::NameInd;
pub use record_type::RecordType;
pub use regn_code::RegnCode;
pub use sec_sub_code::{SecCode, SubCode};
pub use waypoint_type::WaypointType;
pub use waypoint_usage::WaypointUsage;

#[derive(Debug)]
pub enum FieldError {
    InvalidLength,
    InvalidValue(&'static str),
    /// An error returned when a field contained an unexpected character.
    UnexpectedChar(&'static str),
    /// A numeric field is, unexpectedly, not a number.
    NotANumber,
    /// The value of a numeric field is, unexpectedly, out of an allowed range.
    NumberOutOfRange,
}

trait Field
where
    Self: Sized + str::FromStr,
{
}

#[derive(Debug)]
pub struct AlphaNumericField<const I: usize, const N: usize>([u8; N]);

impl<const I: usize, const N: usize> Field for AlphaNumericField<I, N> {}

impl<const I: usize, const N: usize> str::FromStr for AlphaNumericField<I, N> {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match <[u8; N]>::try_from(s[I..I + N].as_bytes()) {
            Ok(b) => Ok(Self(b)),
            _ => Err(FieldError::InvalidLength),
        }
    }
}

impl<const I: usize, const N: usize> PartialEq<&str> for AlphaNumericField<I, N> {
    fn eq(&self, other: &&str) -> bool {
        self.0 == other.as_bytes()
    }
}

impl<const I: usize, const N: usize> fmt::Display for AlphaNumericField<I, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            str::from_utf8(self.0.as_slice())
                .map_err(|_| fmt::Error)?
                .trim_end()
        )
    }
}
