use efb::prelude::*;
use wasm_bindgen::prelude::*;

use crate::{JsAircraftBuilder, JsFuel, JsMass};

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
    pub fn set_aircraft(&mut self, aircraft: JsAircraftBuilder) -> Result<(), JsError> {
        let ac = AircraftBuilder::from(aircraft).build()?;
        self.inner.aircraft(ac);
        Ok(())
    }

    #[wasm_bindgen(setter)]
    pub fn set_mass(&mut self, mass: Vec<JsMass>) -> Result<(), JsError> {
        let mass: Vec<Mass> = mass.into_iter().map(|m| m.into()).collect();
        self.inner.mass(mass);
        Ok(())
    }

    #[wasm_bindgen(setter)]
    pub fn set_policy(&mut self, policy: JsFuelPolicy) {
        self.inner.policy(policy.into());
    }

    #[wasm_bindgen(setter)]
    pub fn set_taxi(&mut self, taxi: JsFuel) {
        self.inner.taxi(taxi.into());
    }

    #[wasm_bindgen(setter)]
    pub fn set_reserve(&mut self, reserve: JsReserve) {
        self.inner.reserve(reserve.into());
    }

    #[wasm_bindgen(setter)]
    pub fn set_perf(&mut self, perf: JsPerformance) {
        self.inner.perf(perf.into());
    }

    #[wasm_bindgen(setter, js_name = takeoffPerf)]
    pub fn set_takeoff_perf(&mut self, perf: JsTakeoffLandingPerformance) {
        self.inner.takeoff_perf(perf.into());
    }

    #[wasm_bindgen(setter, js_name = landingPerf)]
    pub fn set_landing_perf(&mut self, perf: JsTakeoffLandingPerformance) {
        self.inner.landing_perf(perf.into());
    }
}

impl From<JsFlightPlanningBuilder> for FlightPlanningBuilder {
    fn from(value: JsFlightPlanningBuilder) -> Self {
        value.inner
    }
}

////////////////////////////////////////////////////////////////////////////////

#[wasm_bindgen(js_name = FuelPolicy)]
pub struct JsFuelPolicy {
    inner: FuelPolicy,
}

#[wasm_bindgen(js_class = FuelPolicy)]
impl JsFuelPolicy {
    #[wasm_bindgen(constructor)]
    pub fn new(policy: String, fuel: Option<JsFuel>) -> Result<Self, JsError> {
        let inner = match (policy.as_str(), fuel) {
            ("MinimumFuel", _) => Ok(FuelPolicy::MinimumFuel),
            ("MaximumFuel", _) => Ok(FuelPolicy::MaximumFuel),
            ("ManualFuel", Some(fuel)) => Ok(FuelPolicy::ManualFuel(fuel.into())),
            ("FuelAtLanding", Some(fuel)) => Ok(FuelPolicy::FuelAtLanding(fuel.into())),
            ("ExtraFuel", Some(fuel)) => Ok(FuelPolicy::ExtraFuel(fuel.into())),
            _ => Err(JsError::new("invalid fuel policy or fuel undefined")),
        }?;

        Ok(Self { inner })
    }
}

impl From<JsFuelPolicy> for FuelPolicy {
    fn from(value: JsFuelPolicy) -> Self {
        value.inner
    }
}

////////////////////////////////////////////////////////////////////////////////

#[wasm_bindgen(js_name = Reserve)]
pub struct JsReserve {
    inner: Reserve,
}

#[wasm_bindgen(js_class = Reserve)]
impl JsReserve {
    pub fn manual(duration: u32) -> Self {
        Self {
            inner: Reserve::Manual(Duration::s(duration)),
        }
    }
}

impl From<JsReserve> for Reserve {
    fn from(value: JsReserve) -> Self {
        value.inner
    }
}

////////////////////////////////////////////////////////////////////////////////

#[wasm_bindgen(js_name = Performance)]
pub struct JsPerformance {
    inner: Performance,
}

impl From<JsPerformance> for Performance {
    fn from(value: JsPerformance) -> Self {
        value.inner
    }
}

impl From<Performance> for JsPerformance {
    fn from(value: Performance) -> Self {
        Self { inner: value }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[wasm_bindgen(js_name = TakeoffLandingPerformance)]
pub struct JsTakeoffLandingPerformance {
    inner: TakeoffLandingPerformance,
}

impl From<JsTakeoffLandingPerformance> for TakeoffLandingPerformance {
    fn from(value: JsTakeoffLandingPerformance) -> Self {
        value.inner
    }
}

impl From<TakeoffLandingPerformance> for JsTakeoffLandingPerformance {
    fn from(value: TakeoffLandingPerformance) -> Self {
        Self { inner: value }
    }
}
