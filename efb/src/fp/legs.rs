use super::perf::Performance;
use crate::nd::*;
use crate::*;

/// A leg `from` one point `to` another.
#[derive(Clone)]
pub struct Leg {
    /// The point from which the leg starts.
    pub from: NavAid,
    /// The point to which the leg is going.
    pub to: NavAid,
    /// The level of the leg.
    pub level: Option<VerticalDistance>,
    /// The desired true airspeed (TAS).
    pub tas: Option<Speed>,
    /// The wind to take into account.
    pub wind: Option<Wind>,
}

impl Leg {
    /// The true heading considering the wind correction angle (WCA).
    pub fn heading(&self) -> Option<Angle> {
        Some(self.bearing() + self.wca()?)
    }

    /// The magnetic heading considering the variation at the start of the leg.
    pub fn mh(&self) -> Option<Angle> {
        Some(self.heading()? + self.from.mag_var())
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
    pub fn gs(&self) -> Option<Speed> {
        let tas = self.tas?.to_kt().into_inner();
        let ws = self.wind?.speed.to_kt().into_inner();

        Some(Speed::Knots(
            (tas.powi(2) + ws.powi(2)
                - ((2.0 * tas * ws)
                    * (self.bearing() - self.wind?.direction + self.wca()?)
                        .as_radians()
                        .cos()))
            .sqrt(),
        ))
    }

    // TODO add test to verify calculation
    /// The wind correction angle based on the wind.
    fn wca(&self) -> Option<Angle> {
        let tas = self.tas?.to_kt().into_inner();
        let ws = self.wind?.speed.to_kt().into_inner();

        Some(
            (ws / tas
                * (self.bearing() - 180 + self.wind?.direction)
                    .as_radians()
                    .sin())
            .asin()
            .into(),
        )
    }

    // TODO add test to verify calculation
    /// The estimated time enroute the leg.
    pub fn ete(&self) -> Option<Duration> {
        Some(self.dist() / self.gs()?)
    }

    /// The [Fuel] consumed on the leg with the given [Performance].
    pub fn fuel(&self, perf: &Performance) -> Option<Fuel> {
        Some(perf.ff(&self.level?) * self.ete()?)
    }
}
