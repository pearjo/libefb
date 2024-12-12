use super::{Leg, Performance};
use crate::error::Error;
use crate::nd::*;
use crate::{Duration, Fuel, Speed, VerticalDistance, Wind};

enum RouteElement {
    Tas(Speed),
    Level(VerticalDistance),
    Wind(Wind),
    NavAid(NavAid),
}

/// A route that goes from an origin to a destination.
///
/// The route is composed of legs where each [`leg`] describes path between two
/// [`fixes`].
///
/// # Decoding
///
/// The route can be decoded from a space separated list of fixes, wind values
/// and performance elements. The route elements
///
/// ```text
/// 135@09 107KT EDDH DHD HLW EDHL
/// ```
///
/// would create a route from Hamburg to Luebeck via outbound delta routing and
/// inbound whisky routing with a desired TAS of 107kt and a wind of 9kt from
/// south-east. Performance elements can be add at any point but latest before
/// the first leg is defined (we have from and to fix).
///
/// Thus, each leg is computed based on the latest performance elements defined
/// on the route. Extending our route to
///
/// ```text
/// 13509KT N0107 EDDH DHD 18009KT HLW EDHL
/// ```
///
/// we would have wind from south-east (135°) on the leg from EDDH to DHD, but
/// the wind would turn to south (180°) for the remaining legs.
///
/// [`leg`]: crate::fp::legs::Leg
/// [`fixes`]: crate::nd::Fix
pub struct Route {
    _elements: Vec<RouteElement>,
    legs: Vec<Leg>,
}

impl Route {
    /// Decodes a `route` that is composed of a space separated list of fix
    /// idents read from the navigation data `nd`.
    pub fn decode(route: &str, nd: &NavigationData) -> Result<Self, Error> {
        let mut elements: Vec<RouteElement> = Vec::new();

        for element in route.split_whitespace() {
            if let Some(navaid) = nd.find(element) {
                elements.push(RouteElement::NavAid(navaid));
            } else if let Ok(value) = element.parse::<VerticalDistance>() {
                elements.push(RouteElement::Level(value));
            } else if let Ok(value) = element.parse::<Speed>() {
                elements.push(RouteElement::Tas(value));
            } else if let Ok(value) = element.parse::<Wind>() {
                elements.push(RouteElement::Wind(value));
            } else {
                return Err(Error::UnexpectedRouteElement);
            }
        }

        let legs = Self::legs_from_elements(&elements);

        Ok(Self { _elements: elements, legs })
    }

    /// Returns the legs of the route.
    pub fn legs(&self) -> &Vec<Leg> {
        &self.legs
    }

    /// Returns the final leg but going to the alternate.
    pub fn alternate(&self, alternate: NavAid) -> Leg {
        let final_leg = self.legs[self.legs.len() - 1].clone();
        Leg {
            from: final_leg.from,
            to: alternate,
            level: final_leg.level,
            tas: final_leg.tas,
            wind: final_leg.wind,
        }
    }

    /// Returns the fuel consumption en-route for the given performance.
    pub fn fuel<P>(&self, perf: &P) -> Option<Fuel>
    where
        P: Performance,
    {
        self.legs
            .iter()
            .filter_map(|leg| leg.fuel(perf))
            .reduce(|acc, fuel| acc + fuel)
    }

    /// Returns the estimated time en-route.
    pub fn ete(&self) -> Option<Duration> {
        self.legs
            .iter()
            .filter_map(|leg| leg.ete())
            .reduce(|acc, ete| acc + ete)
    }

    fn legs_from_elements(elements: &Vec<RouteElement>) -> Vec<Leg> {
        let mut level: Option<VerticalDistance> = None;
        let mut tas: Option<Speed> = None;
        let mut wind: Option<Wind> = None;
        let mut from: Option<NavAid> = None;
        let mut to: Option<NavAid> = None;
        let mut legs: Vec<Leg> = Vec::new();

        for element in elements {
            match element {
                RouteElement::Level(value) => level = Some(value.clone()),
                RouteElement::Tas(value) => tas = Some(value.clone()),
                RouteElement::Wind(value) => wind = Some(value.clone()),
                RouteElement::NavAid(navaid) => {
                    if from.is_none() {
                        from = Some(navaid.clone());
                    } else if to.is_none() {
                        to = Some(navaid.clone());
                    }
                }
            }

            match (from.clone(), to.clone()) {
                (Some(from), Some(to)) => {
                    legs.push(Leg {
                        from,
                        to,
                        level,
                        tas,
                        wind,
                    });
                }
                _ => continue,
            }

            (from, to) = (to, None);
        }

        legs
    }
}
