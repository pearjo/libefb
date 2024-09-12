use super::Leg;
use crate::nd::*;
use crate::{Angle, Speed, Wind};

pub enum RouteError {
    FixNotFound,
}

// TODO add proper docs on how to enter a route
/// # Decoding
///
/// The route can be decoded from a space separated list of fixes, wind values
/// and performance instructions. The instructions
///
///   `135@09 107KT EDDH DHD HLW EDHL`
///
/// would create a route from Hamburg to Luebeck via outbound delta routing and
/// inbound whisky routing with a desired TAS of 107kt and a wind of 9kt from
/// south-east.
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
                tas: Speed::Knots(107.0),
                wind: Wind {
                    direction: Angle::from_deg(0),
                    speed: Speed::Knots(0.0),
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
