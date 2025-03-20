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

mod dist;
mod fuel;
mod mag_var;
mod wind;

pub use dist::VerticalDistance;
pub use fuel::*;
pub use mag_var::*;
pub use wind::*;

/// A trait to convert to and from the International System of Units (SI).
///
/// Implementing this trait on a type that represents a physical value e.g. a
/// distance, allows to convert to and from a float which represents the value
/// in the corresponding International System of Units (SI) e.g. Meter for a
/// distance.
///
/// # Examples
///
/// ```
/// use efb::Unit;
///
/// enum Distance {
///     Meter(f32),
///     Feet(f32),
/// }
///
/// impl Unit for Distance {
///     fn si(&self) -> f32 {
///         match self {
///             Self::Meter(m) => m.clone(),
///             Self::Feet(ft) => ft * 0.3048,
///         }
///     }
///
///     fn from_si(value: f32) -> Self {
///         Self::Meter(value)
///     }
/// }
/// ```
// TODO Add symbol method to implement a format and display method etc on the
// trait or add a derivable macro.
pub trait Unit {
    /// Converts this types to its value in the SI unit.
    fn si(&self) -> f32;

    /// Converts to this type from the value in SI unit.
    fn from_si(value: f32) -> Self;
}
