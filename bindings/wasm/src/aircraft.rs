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

use efb::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = AircraftBuilder)]
pub struct JsAircraftBuilder {
    inner: AircraftBuilder,
}

#[wasm_bindgen(js_class = AircraftBuilder)]
impl JsAircraftBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: AircraftBuilder::new(),
        }
    }

    pub fn build(&self) -> JsValue {
        match self.inner.build() {
            Ok(ac) => serde_wasm_bindgen::to_value(&ac).unwrap(),
            _ => JsValue::undefined(),
        }
    }
}

impl From<JsAircraftBuilder> for AircraftBuilder {
    fn from(value: JsAircraftBuilder) -> Self {
        value.inner
    }
}

impl From<AircraftBuilder> for JsAircraftBuilder {
    fn from(value: AircraftBuilder) -> Self {
        Self { inner: value }
    }
}
