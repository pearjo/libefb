use std::cell::RefCell;
use std::rc::Rc;

use efb::prelude::*;
use wasm_bindgen::prelude::*;

use crate::{JsAircraftBuilder, JsNavigationData, JsRoute};

#[wasm_bindgen(js_name = FMS)]
pub struct JsFMS {
    inner: Rc<RefCell<FMS>>,
}

#[wasm_bindgen(js_class = FMS)]
impl JsFMS {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(FMS::new())),
        }
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

    pub fn decode(&mut self, route: &str) {
        let _ = self.inner.borrow_mut().decode(route);
    }
}

#[wasm_bindgen(js_name = FlightPlanningBuilder)]
pub struct JsFlightPlanningBuilder {
    inner: FlightPlanningBuilder,
}

#[wasm_bindgen(js_class = FlightPlanningBuilder)]
impl JsFlightPlanningBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: FlightPlanningBuilder::new(),
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_aircraft(&mut self, aircraft: JsAircraftBuilder) {
        let ac = AircraftBuilder::from(aircraft)
            .build()
            .expect("Aircraft should be fully defined.");
        self.inner.set_aircraft(ac);
    }

    #[wasm_bindgen(setter)]
    pub fn set_mass(&mut self, mass: JsValue) {}
}

impl From<FlightPlanningBuilder> for JsFlightPlanningBuilder {
    fn from(value: FlightPlanningBuilder) -> Self {
        Self { inner: value }
    }
}
