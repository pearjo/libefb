use super::Leg;
use crate::nd::*;
use crate::{Speed, Wind};

enum RouteElement<'a> {
    Tas(Speed),
    Wind(Wind),
    Fix(Fix<'a>),
}

#[derive(Debug)]
pub enum RouteError {
    /// A true airspeed is expected but not provided.
    NoSpeed,
    /// Wind along the route is expected but not provided.
    NoWind,
    /// Neither a fix nor a performance element is found.
    UnexpectedElement,
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
/// 135@09 107KT EDDH DHD 180@09 HLW EDHL
/// ```
///
/// we would have wind from south-east (135°) on the leg from EDDH to DHD, but
/// the wind would turn to south (180°) for the remaining legs.
///
/// [`leg`]: crate::fp::legs::Leg
/// [`fixes`]: crate::nd::Fix
pub struct Route<'a> {
    elements: Vec<RouteElement<'a>>,
    legs: Vec<Leg<'a>>,
}

impl<'a> Route<'a> {
    /// Decodes a `route` that is composed of a space separated list of fix
    /// idents read from the navigation data `nd`.
    pub fn decode(route: &str, nd: &'a NavigationData) -> Result<Self, RouteError> {
        let mut elements: Vec<RouteElement> = Vec::new();

        for element in route.split_whitespace() {
            if let Some(fix) = nd.find(element) {
                elements.push(RouteElement::Fix(fix));
            } else if let Ok(value) = element.parse::<Speed>() {
                elements.push(RouteElement::Tas(value));
            } else if let Ok(value) = element.parse::<Wind>() {
                elements.push(RouteElement::Wind(value));
            } else {
                return Err(RouteError::UnexpectedElement);
            }
        }

        let legs = Self::legs_from_elements(&elements)?;

        Ok(Self { elements, legs })
    }

    pub fn legs(&self) -> &Vec<Leg<'a>> {
        &self.legs
    }

    fn legs_from_elements(elements: &Vec<RouteElement<'a>>) -> Result<Vec<Leg<'a>>, RouteError> {
        let mut tas: Option<Speed> = None;
        let mut wind: Option<Wind> = None;
        let mut from: Option<Fix<'a>> = None;
        let mut to: Option<Fix<'a>> = None;
        let mut legs: Vec<Leg> = Vec::new();

        for element in elements {
            match *element {
                RouteElement::Tas(value) => tas = Some(value),
                RouteElement::Wind(value) => wind = Some(value),
                RouteElement::Fix(fix) => {
                    if from.is_none() {
                        from = Some(fix);
                    } else if to.is_none() {
                        to = Some(fix);
                    }
                }
            }

            match (tas, wind, from, to) {
                (Some(tas), Some(wind), Some(from), Some(to)) => {
                    legs.push(Leg {
                        from,
                        to,
                        tas,
                        wind,
                    });
                }
                (None, _, Some(_), Some(_)) => return Err(RouteError::NoSpeed),
                (_, None, Some(_), Some(_)) => return Err(RouteError::NoWind),
                _ => continue,
            }

            from = to;
            to = None;
        }

        Ok(legs)
    }
}
