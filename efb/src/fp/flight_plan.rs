use super::{Leg, Route};
use crate::nd::NavAid;
use crate::{Speed, VerticalDistance};

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
}
