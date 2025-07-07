use std::cell::RefCell;
use std::rc::Rc;

use efb::nd::Fix;
use efb::prelude::*;
use efb::route::Leg;
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
        serde_wasm_bindgen::to_value(&self.inner.level()).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn tas(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.tas()).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn wind(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.wind()).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn mc(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.mc()).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn mh(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.mh()).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn dist(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.dist()).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn ete(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.ete()).unwrap()
    }
}

#[wasm_bindgen(js_name = Route)]
pub struct JsRoute {
    pub(super) inner: Rc<RefCell<FMS>>,
}

#[wasm_bindgen(js_class = Route)]
impl JsRoute {
    pub fn legs(&self) -> Vec<JsLeg> {
        let mut fms = self.inner.borrow_mut();
        fms.route()
            .legs()
            .iter()
            .cloned()
            .map(|leg| JsLeg { inner: leg })
            .collect()
    }
}
