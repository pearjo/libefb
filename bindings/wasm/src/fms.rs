use std::cell::RefCell;
use std::rc::Rc;

extern crate console_error_panic_hook;

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
        console_error_panic_hook::set_once();

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

    pub fn decode(&mut self, route: &str) -> Result<(), JsError> {
        self.inner.borrow_mut().decode(route)?;
        Ok(())
    }

    #[wasm_bindgen(setter, js_name = flightPlanning)]
    pub fn set_flight_planning(&mut self, builder: JsFlightPlanningBuilder) -> Result<(), JsError> {
        self.inner
            .borrow_mut()
            .set_flight_planning(builder.into())?;
        Ok(())
    }

    pub fn print(&self, line_length: Option<usize>) -> String {
        self.inner.borrow().print(line_length.unwrap_or(80))
    }
}
