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

use crate::nd::NavigationData;
use std::fs;
use std::io::ErrorKind;

mod arinc424;
mod openair;

pub use arinc424::Arinc424Parser;
pub use openair::OpenAirParser;

pub trait Parser {
    /// Parse the string `s`.
    fn parse(s: &str) -> Result<NavigationData, ParserError>;

    /// Read the file from [`path`] and parses the airspaces from it.
    fn read(path: &str) -> Result<NavigationData, ParserError> {
        match fs::read_to_string(path) {
            Ok(string) => Self::parse(&string),
            Err(err) => Err(match err.kind() {
                ErrorKind::NotFound => ParserError::NotFound,
                ErrorKind::PermissionDenied => ParserError::PermissionDenied,
                _ => ParserError::FileNotRead,
            }),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    FileNotRead,
    UnexpectedLine,
    NotFound,
    PermissionDenied,
}
