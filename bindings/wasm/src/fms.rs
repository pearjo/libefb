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

extern crate console_error_panic_hook;

use efb::prelude::*;
use wasm_bindgen::prelude::*;

use crate::{JsFlightPlanningBuilder, JsNavigationData, JsRoute};

#[wasm_bindgen(js_name = FMS)]
#[derive(Default)]
pub struct JsFMS {
    inner: Rc<RefCell<FMS>>,
}

impl JsFMS {
    /// Returns a new pointer to the FMS.
    pub fn fms(&self) -> Rc<RefCell<FMS>> {
        self.inner.clone()
    }
}

impl From<Rc<RefCell<FMS>>> for JsFMS {
    fn from(value: Rc<RefCell<FMS>>) -> Self {
        Self {
            inner: value.clone(),
        }
    }
}

#[wasm_bindgen(js_class = FMS)]
impl JsFMS {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // since the FMS is our entry, set the panic hook here
        console_error_panic_hook::set_once();
        Self::default()
    }

    pub fn nd(&self) -> JsNavigationData {
        JsNavigationData {
            inner: Rc::clone(&self.inner),
        }
    }

    pub fn route(&self) -> JsRoute {
        JsRoute {
            inner: Rc::clone(&self.inner),
        }
    }

    pub fn decode(&mut self, route: String) -> Result<(), JsError> {
        self.inner.borrow_mut().decode(route)?;
        Ok(())
    }

    #[wasm_bindgen(getter, js_name = flightPlanning)]
    pub fn flight_panning(&self) -> JsValue {
        match self.inner.borrow().flight_planning() {
            Some(fp) => serde_wasm_bindgen::to_value(&fp).unwrap(),
            None => JsValue::undefined(),
        }
    }

    #[wasm_bindgen(setter, js_name = flightPlanning)]
    pub fn set_flight_planning(&mut self, builder: JsFlightPlanningBuilder) -> Result<(), JsError> {
        self.inner
            .borrow_mut()
            .set_flight_planning(builder.into())?;
        Ok(())
    }

    pub fn print(&self, line_length: Option<usize>) -> String {
        self.inner.borrow().print(line_length.unwrap_or(80))
    }
}
