use efb::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Duration)]
pub struct JsDuration {
    inner: Duration,
}

#[wasm_bindgen(js_class = Duration)]
impl JsDuration {
    #[wasm_bindgen(constructor)]
    pub fn new(s: u32) -> Self {
        Self {
            inner: Duration::s(s),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn hours(&self) -> u32 {
        self.inner.hours()
    }

    #[wasm_bindgen(getter)]
    pub fn minutes(&self) -> u32 {
        self.inner.minutes()
    }

    #[wasm_bindgen(getter)]
    pub fn seconds(&self) -> u32 {
        self.inner.seconds()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[wasm_bindgen(js_name = Length)]
pub struct JsLength {
    inner: Length,
}

#[wasm_bindgen(js_class = Length)]
impl JsLength {
    #[wasm_bindgen(constructor)]
    pub fn new(value: Option<f32>, unit: Option<String>) -> Self {
        let unit = match unit.as_deref() {
            Some("m") => LengthUnit::Meters,
            Some("NM") => LengthUnit::NauticalMiles,
            Some("in") => LengthUnit::Inches,
            Some("ft") => LengthUnit::Feet,
            _ => LengthUnit::si(),
        };

        Self {
            inner: Length::new(value.unwrap_or_default(), unit),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> f32 {
        *self.inner.value()
    }

    #[wasm_bindgen(getter)]
    pub fn unit(&self) -> String {
        self.inner.symbol().to_string()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[wasm_bindgen(js_name = Measurement)]
pub struct JsMeasurement {
    value: JsValue,
    unit: JsValue,
}

#[wasm_bindgen(js_class = Measurement)]
impl JsMeasurement {
    #[wasm_bindgen(constructor)]
    pub fn new(value: JsValue, unit: JsValue) -> Self {
        Self { value, unit }
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> JsValue {
        self.value.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn unit(&self) -> JsValue {
        self.unit.clone()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[wasm_bindgen(js_name = Mass)]
pub struct JsMass {
    inner: Mass,
}

#[wasm_bindgen(js_class = Mass)]
impl JsMass {
    #[wasm_bindgen(constructor)]
    pub fn new(value: Option<f32>, unit: Option<String>) -> Self {
        let unit = match unit.as_deref() {
            Some("kg") => MassUnit::Kilograms,
            Some("lb") => MassUnit::Pounds,
            _ => MassUnit::si(),
        };

        Self {
            inner: Mass::new(value.unwrap_or_default(), unit),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> f32 {
        *self.inner.value()
    }

    #[wasm_bindgen(getter)]
    pub fn unit(&self) -> String {
        self.inner.symbol().to_string()
    }
}

impl From<JsMass> for Mass {
    fn from(value: JsMass) -> Self {
        value.inner
    }
}

////////////////////////////////////////////////////////////////////////////////

#[wasm_bindgen(js_name = Volume)]
pub struct JsVolume {
    inner: Volume,
}

#[wasm_bindgen(js_class = Volume)]
impl JsVolume {
    #[wasm_bindgen(constructor)]
    pub fn new(value: Option<f32>, unit: Option<String>) -> Self {
        let unit = match unit.as_deref() {
            Some("m³") => VolumeUnit::CubicMeters,
            Some("L") => VolumeUnit::Liter,
            _ => VolumeUnit::si(),
        };

        Self {
            inner: Volume::new(value.unwrap_or_default(), unit),
        }
    }
}

impl From<JsVolume> for Volume {
    fn from(value: JsVolume) -> Self {
        value.inner
    }
}
