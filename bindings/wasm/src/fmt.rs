use efb::prelude::{Angle, Length};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = formatAngle)]
pub fn format_angle(angle: JsValue, precision: usize) -> String {
    if let Ok(angle) = serde_wasm_bindgen::from_value::<Angle>(angle) {
        format!("{0:.1$}", angle, precision)
    } else {
        "".to_string()
    }
}

#[wasm_bindgen(js_name = formatLength)]
pub fn format_length(length: JsValue, precision: usize) -> String {
    if let Ok(length) = serde_wasm_bindgen::from_value::<Length>(length) {
        format!("{0:.1$}", length, precision)
    } else {
        "".to_string()
    }
}
