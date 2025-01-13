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

mod fms;
mod fp;
mod route;

pub use fms::*;
pub use fp::*;
pub use route::*;

use std::ffi::{c_char, CString};
use std::string::ToString;

use efb::diesel;
use efb::{Angle, Distance, Duration, Fuel, FuelType, Mass, Speed, VerticalDistance, Volume, Wind};

/// Returns the value as C string if [`ToString`] is implemented.
fn to_string<T>(value: *const T) -> *mut c_char
where
    T: ToString,
{
    let s = unsafe {
        match value.as_ref() {
            Some(ref v) => v.to_string(),
            None => String::from(""),
        }
    };

    CString::new(s).unwrap().into_raw()
}

/// Frees the string `s`.
///
/// # Safety
///
/// The caller must make sure that only strings that are allocated by the libefb
/// are passed to this function. It is unsafe to try freeing any string that was
/// returned by a function of this library.
#[no_mangle]
pub unsafe extern "C" fn efb_string_free(s: *mut c_char) {
    if !s.is_null() {
        drop(unsafe { CString::from_raw(s) });
    }
}

/// Returns the angle formatted as string.
///
/// # Safety
///
/// The returned string needs to be freed by [`efb_string_free`].
#[no_mangle]
pub unsafe extern "C" fn efb_angle_to_string(angle: *const Angle) -> *mut c_char {
    to_string(angle)
}

/// Returns the distance formatted as string.
///
/// # Safety
///
/// The returned string needs to be freed by [`efb_string_free`].
#[no_mangle]
pub unsafe extern "C" fn efb_distance_to_string(distance: *const Distance) -> *mut c_char {
    to_string(distance)
}

/// Returns the duration formatted as string.
///
/// # Safety
///
/// The returned string needs to be freed by [`efb_string_free`].
#[no_mangle]
pub unsafe extern "C" fn efb_duration_to_string(duration: *const Duration) -> *mut c_char {
    to_string(duration)
}

/// Returns the wind formatted as string.
///
/// # Safety
///
/// The returned string needs to be freed by [`efb_string_free`].
#[no_mangle]
pub unsafe extern "C" fn efb_wind_to_string(wind: *const Wind) -> *mut c_char {
    to_string(wind)
}

/// Returns the speed formatted as string.
///
/// # Safety
///
/// The returned string needs to be freed by [`efb_string_free`].
#[no_mangle]
pub unsafe extern "C" fn efb_speed_to_string(speed: *const Speed) -> *mut c_char {
    to_string(speed)
}

/// Returns a distance in meter.
#[no_mangle]
pub extern "C" fn efb_distance_m(m: f32) -> Distance {
    Distance::Meter(m)
}

/// Returns the seconds `s` as duration.
#[no_mangle]
pub extern "C" fn efb_duration(s: u32) -> Duration {
    Duration::from(s)
}

/// Returns `l` liter of Diesel.
#[no_mangle]
pub extern "C" fn efb_fuel_diesel_l(l: f32) -> Fuel {
    diesel!(Volume::Liter(l))
}

/// Returns a mass in kilogram.
#[no_mangle]
pub extern "C" fn efb_mass_kg(kg: f32) -> Mass {
    Mass::Kilogram(kg)
}

/// Returns a speed in knots.
#[no_mangle]
pub extern "C" fn efb_speed_knots(kt: f32) -> Speed {
    Speed::Knots(kt)
}

/// Returns a speed in m/s.
#[no_mangle]
pub extern "C" fn efb_speed_mps(mps: f32) -> Speed {
    Speed::MeterPerSecond(mps)
}

/// Returns a speed in mach.
#[no_mangle]
pub extern "C" fn efb_speed_mach(mach: f32) -> Speed {
    Speed::Mach(mach)
}

/// Returns true if `a == b`.
#[no_mangle]
pub extern "C" fn efb_vertical_distance_eq(a: &VerticalDistance, b: &VerticalDistance) -> bool {
    a == b
}

/// Returns true if `a != b`.
#[no_mangle]
pub extern "C" fn efb_vertical_distance_neq(a: &VerticalDistance, b: &VerticalDistance) -> bool {
    a != b
}

/// Returns true if `a < b`.
#[no_mangle]
pub extern "C" fn efb_vertical_distance_lt(a: &VerticalDistance, b: &VerticalDistance) -> bool {
    a < b
}

/// Returns true if `a <= b`.
#[no_mangle]
pub extern "C" fn efb_vertical_distance_lte(a: &VerticalDistance, b: &VerticalDistance) -> bool {
    a <= b
}

/// Returns true if `a > b`.
#[no_mangle]
pub extern "C" fn efb_vertical_distance_gt(a: &VerticalDistance, b: &VerticalDistance) -> bool {
    a > b
}

/// Returns true if `a >= b`.
#[no_mangle]
pub extern "C" fn efb_vertical_distance_gte(a: &VerticalDistance, b: &VerticalDistance) -> bool {
    a >= b
}

/// Returns a vertical distance in feet.
#[no_mangle]
pub extern "C" fn efb_vertical_distance_altitude(ft: u16) -> VerticalDistance {
    VerticalDistance::Altitude(ft)
}

/// Returns a volume in liter.
#[no_mangle]
pub extern "C" fn efb_volume_l(l: f32) -> Volume {
    Volume::Liter(l)
}
