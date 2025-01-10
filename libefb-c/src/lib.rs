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
mod route;

pub use fms::*;
pub use route::*;

use std::ffi::{c_char, CString};
use std::mem::ManuallyDrop;
use std::ptr::NonNull;
use std::string::ToString;

use efb::{Speed, Wind};

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

/// An array.
#[repr(C)]
pub struct EfbArray<T> {
    /// The pointer to the first element within the array.
    data: *mut T,

    /// The length of the array.
    len: usize,

    /// The capacity of the array.
    capacity: usize,
}

impl<T> EfbArray<T> {
    fn into_vec(&mut self) -> Vec<T> {
        let vec = unsafe { Vec::<T>::from_raw_parts(self.data, self.len, self.capacity) };
        self.data = NonNull::dangling().as_ptr();
        self.len = 0;
        self.capacity = 0;
        vec
    }
}

impl<T> From<Vec<T>> for EfbArray<T> {
    fn from(value: Vec<T>) -> Self {
        let mut v = ManuallyDrop::new(value);

        Self {
            data: v.as_mut_ptr(),
            len: v.len(),
            capacity: v.capacity(),
        }
    }
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
