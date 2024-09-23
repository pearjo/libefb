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

//! Flight Management System.

use std::path::Path;

use crate::nd::*;
use crate::parser::{Arinc424Parser, OpenAirParser, Parser};

#[derive(Debug)]
pub struct FMSError;

#[repr(C)]
pub struct FMS {
    pub navigation_data: NavigationData,
}

impl FMS {
    pub fn from_arinc424(path: &Path) -> Result<Self, FMSError> {
        match Arinc424Parser::read(path) {
            Ok(navigation_data) => Ok(Self { navigation_data }),
            _ => Err(FMSError),
        }
    }

    pub fn from_openair(path: &Path) -> Result<Self, FMSError> {
        match OpenAirParser::read(path) {
            Ok(navigation_data) => Ok(Self { navigation_data }),
            _ => Err(FMSError),
        }
    }
}
