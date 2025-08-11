// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Joe Pearson
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::Coordinate;

/// Minimum bounding box around coordinates.
///
/// The box tightly fits all coordinates it was created with the outer points
/// south-west to north-east.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct BBox {
    sw: Coordinate,
    ne: Coordinate,
}

impl BBox {
    /// Creates a new bounding box for the coordinates.
    ///
    /// If the coordinates are empty, [`None`] is returned.
    pub fn new(coords: &[Coordinate]) -> Option<Self> {
        let mut iter = coords.iter();
        let first = iter.next()?;

        let (mut south, mut north) = (first.latitude, first.latitude);
        let (mut west, mut east) = (first.longitude, first.longitude);

        for coord in iter {
            south = south.min(coord.latitude);
            north = north.max(coord.latitude);
            west = west.min(coord.longitude);
            east = east.max(coord.longitude);
        }

        Some(Self {
            sw: Coordinate::new(south, west),
            ne: Coordinate::new(north, east),
        })
    }

    /// Returns the south-west bound.
    pub fn sw(&self) -> &Coordinate {
        &self.sw
    }

    /// Returns the north-east bound.
    pub fn ne(&self) -> &Coordinate {
        &self.ne
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bbox_fits_coords() {
        // Coordinates within the bbox:
        //
        //    +-----3----+ NE
        //    | 2        |
        //    |          4
        //    1     5    |
        //    |          |
        // SW +-----6----+
        let bbox = BBox::new(&vec![
            coord!(20.0, -20.0), // 1
            coord!(30.0, -10.0), // 2
            coord!(40.0, 0.0),   // 3
            coord!(25.0, 20.0),  // 4
            coord!(20.0, 0.0),   // 5
            coord!(0.0, 0.0),    // 6
        ])
        .expect("bbox should be some");

        let sw = coord!(0.0, -20.0);
        let ne = coord!(40.0, 20.0);

        assert_eq!(bbox.sw(), &sw);
        assert_eq!(bbox.ne(), &ne);
    }
}
