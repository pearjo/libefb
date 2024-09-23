use crate::nd::Fix;
use crate::{Angle, Distance, Duration, Speed, Wind};

/// A leg `from` one point `to` another.
pub struct Leg<'a> {
    /// The point from which the leg starts.
    pub from: Fix<'a>,
    /// The point to which the leg is going.
    pub to: Fix<'a>,
    /// The desired true airspeed (TAS).
    pub tas: Speed,
    /// The wind to take into account.
    pub wind: Wind,
}

impl Leg<'_> {
    /// The true heading considering the wind correction angle (WCA).
    pub fn heading(&self) -> Angle {
        self.bearing() + self.wca()
    }

    /// The magnetic heading considering the variation at the start of the leg.
    pub fn mh(&self) -> Angle {
        self.heading() + self.from.var()
    }

    /// The bearing between the two points.
    pub fn bearing(&self) -> Angle {
        self.from.coordinate().bearing(&self.to.coordinate())
    }

    /// The magnetic course taking the magnetic variation from the starting
    /// point into consideration.
    pub fn mc(&self) -> Angle {
        self.bearing() + self.from.var()
    }

    /// The distance between the leg's two points.
    pub fn dist(&self) -> Distance {
        self.from.coordinate().dist(&self.to.coordinate())
    }

    // TODO add test to verify calculation
    /// The ground speed in knots.
    pub fn gs(&self) -> Speed {
        let tas = self.tas.to_kt();
        let ws = self.wind.speed.to_kt();

        Speed::Knots(
            (tas.powi(2) + ws.powi(2)
                - ((2.0 * tas * ws)
                    * (self.bearing() - self.wind.direction + self.wca())
                        .rad
                        .cos()))
            .sqrt(),
        )
    }

    // TODO add test to verify calculation
    /// The wind correction angle based on the wind.
    fn wca(&self) -> Angle {
        let tas = self.tas.to_kt();
        let ws = self.wind.speed.to_kt();

        Angle::from_rad((ws / tas * (self.bearing() - 180 + self.wind.direction).rad.sin()).asin())
    }

    // TODO add test to verify calculation
    /// The estimated time to fly the leg in seconds.
    pub fn time(&self) -> Duration {
        Duration::from_seconds(self.dist() / self.gs())
    }
}
