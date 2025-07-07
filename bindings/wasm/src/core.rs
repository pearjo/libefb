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
