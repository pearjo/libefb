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

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::error::Error;

/// ICAO location indicator.
///
/// Indicates the state/territory location of a navigation aid according to ICAO
/// document no. 7910.
///
/// # Examples
///
/// ```
/// # use efb::nd::LocationIndicator;
/// let ed = LocationIndicator::new("ED").unwrap();
/// assert_eq!(ed.as_str(), "ED");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LocationIndicator([u8; 2]);

impl LocationIndicator {
    /// Creates a location indicator from string.
    pub fn new(s: &str) -> Result<Self, Error> {
        if s.len() != 2 || !s.is_ascii() {
            return Err(Error::UnknownLocationIndicator(s.to_string()));
        }
        let bytes = s.as_bytes();
        Ok(LocationIndicator([bytes[0], bytes[1]]))
    }

    pub fn as_str(&self) -> &str {
        // Safe because we ensure ASCII in constructor
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl std::fmt::Display for LocationIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for LocationIndicator {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}
