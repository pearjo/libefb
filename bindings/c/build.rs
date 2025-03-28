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

extern crate cbindgen;

use std::env;
use std::process::Command;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate(crate_dir)
        .expect("Unable to generate bindings")
        .write_to_file("include/efb.h");

    if cfg!(target_os = "macos") {
        Command::new("xcrun")
            .args(["clang-format", "-i", "examples/**/*.c", "include/efb.h"])
            .output()
            .expect("Failed to run clang-format");
    } else {
        Command::new("clang-format")
            .args(["-i", "examples/**/*.c", "include/efb.h"])
            .output()
            .expect("Failed to run clang-format");
    }
}
