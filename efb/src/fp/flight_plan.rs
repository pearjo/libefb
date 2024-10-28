use super::Route;
use crate::nd::NavAid;
use crate::{Speed, VerticalDistance};

/// A flight plan with cruising level and speed.
pub struct FlightPlan {
    /// The cruising speed.
    pub speed: Speed,

    /// The cruising level.
    pub level: VerticalDistance,

    /// The flight's route.
    pub route: Route,

    /// An optional alternate.
    pub alternate: Option<NavAid>,
}
