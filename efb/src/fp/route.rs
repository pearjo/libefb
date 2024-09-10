use super::Leg;
use crate::fc::Wind;
use crate::geom::Angle;
use crate::nd::*;

pub enum RouteError {
    FixNotFound,
}

pub struct Route<'a> {
    fixes: Vec<Fix<'a>>,
}

impl<'a> Route<'a> {
    pub fn legs(&self) -> Vec<Leg> {
        let mut legs: Vec<Leg> = Vec::new();

        for i in 0..self.fixes.len() - 1 {
            legs.push(Leg {
                from: &self.fixes[i],
                to: &self.fixes[i + 1],
                // TODO decode TAS and wind from input
                tas: 107,
                wind: Wind {
                    direction: Angle::from_deg(0),
                    speed: 0,
                },
            });
        }

        legs
    }

    /// Decodes a `route` that is composed of a space separated list of fix
    /// idents read from the navigation data `nd`.
    pub fn decode(route: &str, nd: &'a NavigationData) -> Result<Self, RouteError> {
        let mut fixes: Vec<Fix<'a>> = Vec::new();
        for ident in route.split_whitespace() {
            match nd.find(ident) {
                Some(fix) => fixes.push(fix),
                _ => return Err(RouteError::FixNotFound),
            }
        }

        Ok(Self { fixes })
    }
}
