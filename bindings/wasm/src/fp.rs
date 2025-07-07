use efb::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Performance)]
pub struct JsPerformance {
    inner: Performance,
}

impl From<Performance> for JsPerformance {
    fn from(value: Performance) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen(js_name = TakeoffLandingPerformance)]
pub struct JsTakeoffLandingPerformance {
    inner: TakeoffLandingPerformance,
}

impl From<TakeoffLandingPerformance> for JsTakeoffLandingPerformance {
    fn from(value: TakeoffLandingPerformance) -> Self {
        Self { inner: value }
    }
}
