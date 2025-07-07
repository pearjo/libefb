use efb::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum JsFuelType {
    Diesel,
    JetA,
}

impl From<JsFuelType> for FuelType {
    fn from(value: JsFuelType) -> Self {
        match value {
            JsFuelType::Diesel => FuelType::Diesel,
            JsFuelType::JetA => FuelType::JetA,
        }
    }
}
