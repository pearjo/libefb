use crate::*;
use crate::nd::*;
use super::perf::Performance;

/// A leg `from` one point `to` another.
pub struct Leg {
    /// The point from which the leg starts.
    pub from: NavAid,
    /// The point to which the leg is going.
    pub to: NavAid,
    /// The vertical distance of the leg.
    pub vd: VerticalDistance,
    /// The desired true airspeed (TAS).
    pub tas: Speed,
    /// The wind to take into account.
    pub wind: Wind,
}

impl Leg {
    /// The true heading considering the wind correction angle (WCA).
    pub fn heading(&self) -> Angle {
        self.bearing() + self.wca()
    }

    /// The magnetic heading considering the variation at the start of the leg.
    pub fn mh(&self) -> Angle {
        self.heading() + self.from.mag_var()
    }

    /// The bearing between the two points.
    pub fn bearing(&self) -> Angle {
        self.from.coordinate().bearing(&self.to.coordinate())
    }

    /// The magnetic course taking the magnetic variation from the starting
    /// point into consideration.
    pub fn mc(&self) -> Angle {
        self.bearing() + self.from.mag_var()
    }

    /// The distance between the leg's two points.
    pub fn dist(&self) -> Distance {
        self.from.coordinate().dist(&self.to.coordinate())
    }

    // TODO add test to verify calculation
    /// The ground speed in knots.
    pub fn gs(&self) -> Speed {
        let tas = self.tas.to_kt().into_inner();
        let ws = self.wind.speed.to_kt().into_inner();

        Speed::Knots(
            (tas.powi(2) + ws.powi(2)
                - ((2.0 * tas * ws)
                    * (self.bearing() - self.wind.direction + self.wca())
                        .as_radians()
                        .cos()))
            .sqrt(),
        )
    }

    // TODO add test to verify calculation
    /// The wind correction angle based on the wind.
    fn wca(&self) -> Angle {
        let tas = self.tas.to_kt().into_inner();
        let ws = self.wind.speed.to_kt().into_inner();

        (ws / tas * (self.bearing() - 180 + self.wind.direction).as_radians().sin())
            .asin()
            .into()
    }

    // TODO add test to verify calculation
    /// The estimated time enroute the leg.
    pub fn ete(&self) -> Duration {
        self.dist() / self.gs()
    }

    /// The [Fuel] consumed on the leg with the given [Performance].
    pub fn fuel<P>(&self, perf: &P) -> Fuel
    where
        P: Performance
    {
        perf.ff(self.vd) * self.ete()
    }
}
