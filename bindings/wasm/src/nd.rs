use std::cell::RefCell;
use std::rc::Rc;

use efb::prelude::*;
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = NavigationData)]
pub struct JsNavigationData {
    pub(super) inner: Rc<RefCell<FMS>>,
}

#[wasm_bindgen(js_class = NavigationData)]
impl JsNavigationData {
    pub fn find(&self, ident: &str) -> JsValue {
        let mut fms = self.inner.borrow_mut();
        serde_wasm_bindgen::to_value(&fms.nd().find(ident)).unwrap()
    }

    pub fn read(&self, s: &str, fmt: &str) {
        if let Some(fmt) = match fmt {
            "arinc424" => Some(InputFormat::Arinc424),
            "openAir" => Some(InputFormat::OpenAir),
            _ => None,
        } {
            let mut fms = self.inner.borrow_mut();
            let _ = fms.nd().read(s, fmt);
        }
    }
}
