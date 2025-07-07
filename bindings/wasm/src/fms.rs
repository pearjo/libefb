use std::cell::RefCell;
use std::rc::Rc;

use efb::prelude::*;
use wasm_bindgen::prelude::*;

use crate::{JsFlightPlanningBuilder, JsNavigationData, JsRoute};

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

    #[wasm_bindgen(setter, js_name = flightPlanning)]
    pub fn set_flight_planning(&mut self, builder: JsFlightPlanningBuilder) -> Result<(), JsError> {
        self.inner
            .borrow_mut()
            .set_flight_planning(&builder.into())?;
        Ok(())
    }
}
