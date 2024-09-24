use super::{Field, FieldError};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum MagTrueInd<const I: usize> {
    Magnetic,
    TrueNorth,
    Mixed
}

impl<const I: usize> Field for MagTrueInd<I> {}

impl<const I: usize> FromStr for MagTrueInd<I> {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[I..I + 1] {
            "M" => Ok(Self::Magnetic),
            "T" => Ok(Self::TrueNorth),
            " " => Ok(Self::Mixed),
            _ => Err(FieldError::UnexpectedChar(
                "unexpected magnetic/true north indicator",
            )),
        }
    }
}
