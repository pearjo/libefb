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

use crate::{JsMass, JsVolume};

#[wasm_bindgen(js_name = Fuel)]
pub struct JsFuel {
    inner: Fuel,
}

#[wasm_bindgen(js_class = Fuel)]
impl JsFuel {
    #[wasm_bindgen(constructor)]
    pub fn new(mass: JsMass, fuel_type: JsValue) -> Result<Self, JsError> {
        let fuel_type: FuelType = serde_wasm_bindgen::from_value(fuel_type)?;

        Ok(Self {
            inner: Fuel::new(mass.into(), fuel_type),
        })
    }

    #[wasm_bindgen(js_name = fromVolume)]
    pub fn from_volume(volume: JsVolume, fuel_type: JsValue) -> Result<Self, JsError> {
        let fuel_type: FuelType = serde_wasm_bindgen::from_value(fuel_type)?;

        Ok(Self {
            inner: Fuel::from_volume(volume.into(), fuel_type),
        })
    }
}

impl From<JsFuel> for Fuel {
    fn from(value: JsFuel) -> Self {
        value.inner
    }
}
