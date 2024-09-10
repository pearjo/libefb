use crate::fc::Wind;
use crate::geom::{Angle, Distance};
use crate::nd::Fix;

/// A leg `from` one point `to` another.
pub struct Leg<'a> {
    /// The point from which the leg starts.
    pub from: &'a Fix<'a>,
    /// The point to which the leg is going.
    pub to: &'a Fix<'a>,
    /// The desired true airspeed (TAS).
    pub tas: i16,
    /// The wind to take into account.
    pub wind: Wind,
}

impl Leg<'_> {
    /// The true heading considering the [`wca`].
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

    /// The ground speed in knots.
    pub fn gs(&self) -> i16 {
        (self.tas.pow(2) as f32 + self.wind.speed.pow(2) as f32
            - ((2 * self.tas * self.wind.speed) as f32
                * (self.bearing().rad - self.wind.direction.rad + self.wca().rad).cos()))
        .sqrt()
        .round() as i16
    }

    /// The wind correction angle based on the wind.
    fn wca(&self) -> Angle {
        Angle::from_rad(
            (self.wind.speed as f32 / self.tas as f32
                * Angle::from_deg(self.bearing().deg - 180 + self.wind.direction.deg)
                    .rad
                    .sin())
            .asin(),
        )
    }

    /// The estimated time to fly the leg in seconds.
    pub fn time(&self) -> u32 {
        (self.dist().to_nm().into_inner() / self.gs() as f32 * 3600.0).round() as u32
    }
}
