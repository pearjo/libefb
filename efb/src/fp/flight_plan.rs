use super::{Leg, Route};
use crate::nd::NavAid;
use crate::{Speed, VerticalDistance};

/// A flight plan with cruising level and speed.
pub struct FlightPlan<'a> {
    /// The cruising speed.
    pub speed: Speed,

    /// The cruising level.
    pub level: VerticalDistance,

    /// The flight's route.
    pub route: &'a Route,

    /// An optional alternate.
    pub alternate: Option<NavAid>,
}

impl<'a> FlightPlan<'a> {
    /// Returns the route's final leg but with the alternate as destination.
    pub fn alternate(&self) -> Option<Leg> {
        match self.alternate.as_ref() {
            Some(alternate) => Some(self.route.alternate(alternate.clone())),
            None => None,
        }
    }
}
