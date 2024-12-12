use std::convert::TryFrom;

use super::{Leg, Performance, Route, RouteElement};
use crate::error::Error;
use crate::nd::NavAid;
use crate::{Fuel, Speed, VerticalDistance};

/// A flight plan with cruising level and speed.
pub struct FlightPlan {
    /// The cruising speed.
    speed: Speed,

    /// The cruising level.
    level: VerticalDistance,

    /// The flight's route.
    route: Route,

    /// An optional alternate.
    alternate: Option<NavAid>,
}

impl FlightPlan {
    pub fn new(
        speed: Speed,
        level: VerticalDistance,
        mut route: Route,
        alternate: Option<NavAid>,
    ) -> Self {
        route.set_cruise(speed, level);
        Self {
            speed,
            level,
            route,
            alternate,
        }
    }

    pub fn speed(&self) -> &Speed {
        &self.speed
    }

    pub fn level(&self) -> &VerticalDistance {
        &self.level
    }

    pub fn route(&self) -> &Route {
        &self.route
    }

    /// Returns the route's final leg but with the alternate as destination.
    pub fn alternate(&self) -> Option<Leg> {
        match self.alternate.as_ref() {
            Some(alternate) => Some(self.route.alternate(alternate.clone())),
            None => None,
        }
    }

    pub fn fuel<P>(&self, perf: &P) -> Fuel
    where
        P: Performance,
    {
        self.route.fuel(perf).unwrap()
    }
}

impl TryFrom<Route> for FlightPlan {
    type Error = Error;

    fn try_from(route: Route) -> Result<Self, Self::Error> {
        let elements = route.elements();
        match (elements.get(0), elements.get(1)) {
            (Some(RouteElement::Speed(speed)), Some(RouteElement::Level(level)))
            | (Some(RouteElement::Level(level)), Some(RouteElement::Speed(speed))) => Ok(Self {
                speed: speed.clone(),
                level: level.clone(),
                route,
                alternate: None,
            }),
            (Some(RouteElement::Speed(_)), _) => Err(Error::ExpectedLevelOnFPL),
            (Some(RouteElement::Level(_)), _) => Err(Error::ExpectedSpeedOnFPL),
            _ => Err(Error::UnexpectedRouteElement),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nd::NavigationData;

    #[test]
    fn convert_route_to_flight_plan() -> Result<(), Error> {
        let nd = NavigationData::new();
        let route = Route::decode("N0107 A020", &nd)?;
        let _ = FlightPlan::try_from(route)?;
        Ok(())
    }

    #[test]
    fn convert_route_to_flight_plan_without_level_or_speed() {
        let nd = NavigationData::new();

        // test without level
        let route = Route::decode("N0107", &nd).unwrap();
        let fpl = FlightPlan::try_from(route);
        assert!(fpl.is_err());

        // test without speed
        let route = Route::decode("A0250", &nd).unwrap();
        let fpl = FlightPlan::try_from(route);
        assert!(fpl.is_err());
    }
}
