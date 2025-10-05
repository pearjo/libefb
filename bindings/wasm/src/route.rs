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

use efb::nd::Fix;
use efb::prelude::*;
use efb::route::Leg;
use serde::ser::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Leg)]
pub struct JsLeg {
    inner: Leg,
}

#[wasm_bindgen(js_class = Leg)]
impl JsLeg {
    #[wasm_bindgen(getter)]
    pub fn from(&self) -> String {
        self.inner.from().ident()
    }

    #[wasm_bindgen(getter)]
    pub fn to(&self) -> String {
        self.inner.to().ident()
    }

    #[wasm_bindgen(getter)]
    pub fn level(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.level()).unwrap_or_default()
    }

    #[wasm_bindgen(getter)]
    pub fn tas(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.tas()).unwrap_or_default()
    }

    #[wasm_bindgen(getter)]
    pub fn wind(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.wind()).unwrap_or_default()
    }

    #[wasm_bindgen(getter)]
    pub fn mc(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.mc()).unwrap_or_default()
    }

    #[wasm_bindgen(getter)]
    pub fn mh(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.mh()).unwrap_or_default()
    }

    #[wasm_bindgen(getter)]
    pub fn dist(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.dist()).unwrap_or_default()
    }

    #[wasm_bindgen(getter)]
    pub fn ete(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.ete()).unwrap_or_default()
    }
}

#[wasm_bindgen(js_name = Route)]
pub struct JsRoute {
    pub(super) inner: Rc<RefCell<FMS>>,
}

#[wasm_bindgen(js_class = Route)]
impl JsRoute {
    #[wasm_bindgen(getter)]
    pub fn origin(&self) -> JsValue {
        let fms = self.inner.borrow();
        serde_wasm_bindgen::to_value(&fms.route().origin()).unwrap_or_default()
    }

    #[wasm_bindgen(getter)]
    pub fn destination(&self) -> JsValue {
        let fms = self.inner.borrow();
        serde_wasm_bindgen::to_value(&fms.route().destination()).unwrap_or_default()
    }

    #[wasm_bindgen(getter)]
    pub fn dist(&self) -> JsValue {
        let fms = self.inner.borrow();
        serde_wasm_bindgen::to_value(&fms.route().dist()).unwrap_or_default()
    }

    #[wasm_bindgen(getter)]
    pub fn ete(&self) -> JsValue {
        let fms = self.inner.borrow();
        serde_wasm_bindgen::to_value(&fms.route().ete()).unwrap_or_default()
    }

    pub fn legs(&self) -> Vec<JsLeg> {
        let fms = self.inner.borrow();
        fms.route()
            .legs()
            .iter()
            .cloned()
            .map(|leg| JsLeg { inner: leg })
            .collect()
    }

    #[wasm_bindgen(js_name = toGeojson)]
    pub fn to_geojson(&self) -> Result<JsValue, JsValue> {
        let fms = self.inner.borrow();
        let value = fms.route().to_geojson();
        let serializer = Serializer::new().serialize_maps_as_objects(true);
        Ok(value.serialize(&serializer)?)
    }
}
