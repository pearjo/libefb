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

use std::ffi::{c_char, CStr};
use std::path::Path;

use efb::fms::FMS;
use efb::nd::InputFormat;

use super::EfbRoute;

/// The Flight Management System (FMS).
///
/// This type wraps the [FMS] which is the integral system of this library. The
/// FMS holds all information like the navigation data or the route.
pub struct EfbFms {
    inner: FMS,
}

/// Creates and returns a new FMS.
///
/// # Safety
///
/// The caller is responsible to free the allocated FMS by calling efb_fms_free.
#[no_mangle]
pub unsafe extern "C" fn efb_fms_new() -> Box<EfbFms> {
    let fms = EfbFms { inner: FMS::new() };
    Box::new(fms)
}

/// Frees the memory of the allocated FMS.
#[no_mangle]
pub extern "C" fn efb_fms_free(fms: Option<Box<EfbFms>>) {
    drop(fms);
}

/// Reads the string which is in the fmt into the navigation database.
///
/// # Safety
///
/// It is up to the caller to guarantee that `s` points to a valid string.
#[no_mangle]
pub unsafe extern "C" fn efb_fms_nd_read(fms: &mut EfbFms, s: *const c_char, fmt: InputFormat) {
    if let Ok(s) = unsafe { CStr::from_ptr(s).to_str() } {
        let nd = fms.inner.nd();
        let _ = nd.read(s, fmt);
    }
}

/// Reads the file at the path which is in the fmt into the navigation database.
///
/// # Safety
///
/// It is up to the caller to guarantee that `path` points to a valid string.
#[no_mangle]
pub unsafe extern "C" fn efb_fms_nd_read_file(
    fms: &mut EfbFms,
    path: *const c_char,
    fmt: InputFormat,
) {
    if let Ok(path) = unsafe { CStr::from_ptr(path).to_str() } {
        let nd = fms.inner.nd();
        let _ = nd.read_file(Path::new(path), fmt);
    }
}

/// Decodes the route and enters it into the FMS.
///
/// # Safety
///
/// It is up to the caller to guarantee that `route` points to a valid string.
#[no_mangle]
pub unsafe extern "C" fn efb_fms_decode(fms: &mut EfbFms, route: *const c_char) {
    if let Ok(route) = unsafe { CStr::from_ptr(route).to_str() } {
        let _ = fms.inner.decode(route);
    }
}

/// Returns a new route from the FMS.
///
/// # Safety
///
/// It's up to the caller to unref the returned route.
#[no_mangle]
pub unsafe extern "C" fn efb_fms_route_ref(fms: &EfbFms) -> Box<EfbRoute> {
    Box::new(EfbRoute::from(fms.inner.route()))
}

/// Decreases the reference count of the route.
#[no_mangle]
pub extern "C" fn efb_fms_route_unref(route: Option<Box<EfbRoute>>) {
    drop(route);
}
