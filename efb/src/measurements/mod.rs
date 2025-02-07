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

use std::fmt;

pub trait UnitOfMeasure {
    fn symbol(&self) -> &str;

    fn from_si(value: f64, to: &Self) -> f64;

    fn to_si(&self, value: f64) -> f64;

    fn convert_to(&self, value: f64, other: &Self) -> f64 {
        Self::from_si(self.to_si(value), other)
    }
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Measurement<T: UnitOfMeasure> {
    pub value: f64,
    pub unit: T,
}

impl<T> Measurement<T>
where
    T: UnitOfMeasure,
{
    pub fn symbol(&self) -> &str {
        self.unit.symbol()
    }

    pub fn from_si(value: f64, to: T) -> Self {
        Self {
            value: T::from_si(value, &to),
            unit: to,
        }
    }

    pub fn to_si(&self) -> f64 {
        self.unit.to_si(self.value)
    }

    pub fn convert_to(&self, other: T) -> Self {
        Self {
            value: self.unit.convert_to(self.value, &other),
            unit: other,
        }
    }
}

impl<T> fmt::Display for Measurement<T>
where
    T: UnitOfMeasure,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Respect the formatting.
        write!(f, "{} {}", self.value, self.symbol())
    }
}

mod density;
mod length;
mod mass;
mod speed;
mod time;

use length::{Length, LengthUnit};
use mass::{Mass, MassUnit};
use speed::{Speed, SpeedUnit};
use time::{Time, TimeUnit};
