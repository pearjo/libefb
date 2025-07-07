use efb::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::console;

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

    #[wasm_bindgen(setter)]
    pub fn set_registration(&mut self, registration: String) {
        self.inner.registration(registration);
    }

    #[wasm_bindgen(setter)]
    pub fn set_stations(&mut self, stations: JsValue) {
        if let Ok(stations) = serde_wasm_bindgen::from_value(stations) {
            self.inner.stations(stations);
        } else {
            console::error_1(&"Unexpected stations!".into());
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
