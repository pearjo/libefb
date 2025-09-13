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

use std::cell::RefCell;
use std::rc::Rc;

use efb::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = NavigationData)]
pub struct JsNavigationData {
    pub(super) inner: Rc<RefCell<FMS>>,
}

#[wasm_bindgen(js_class = NavigationData)]
impl JsNavigationData {
    pub fn find(&self, ident: &str) -> JsValue {
        let fms = self.inner.borrow();
        serde_wasm_bindgen::to_value(&fms.nd().find(ident)).unwrap()
    }

    pub fn read(&self, s: &str, fmt: &str) -> Result<(), JsError> {
        if let Some(fmt) = match fmt {
            "arinc424" => Some(InputFormat::Arinc424),
            "openAir" => Some(InputFormat::OpenAir),
            _ => None,
        } {
            let mut fms = self.inner.borrow_mut();
            fms.nd().read(s, fmt)?;
        }

        Ok(())
    }
}
