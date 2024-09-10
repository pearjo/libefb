// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 Joe Pearson
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

/// Creates a [`Coordinate`].
///
/// [`Coordinate`]: crate::geom::Coordinate
#[macro_export]
macro_rules! coord {
    ($latitude:expr, $longitude:expr) => {
        Coordinate {
            latitude: $latitude,
            longitude: $longitude,
        }
    };
}

/// Creates a [`Polygon`] containing the coordinates.
///
/// ```
/// use efb::geom::{Coordinate, Polygon};
/// use efb::polygon;
///
/// let p = polygon![(0.0, 0.0), (10.0, 10.0)];
/// assert_eq!(p[0], Coordinate { latitude: 0.0, longitude: 0.0 });
/// assert_eq!(p[1], Coordinate { latitude: 10.0, longitude: 10.0 });
/// ```
///
/// [`Polygon`]: crate::geom::Polygon
#[macro_export]
macro_rules! polygon {
    ( $( $p:expr ),* ) => {
        {
            let mut coordinates = Polygon::new();
            $(
                coordinates.push(
                    Coordinate {
                        latitude: $p.0,
                        longitude: $p.1,
                    }
                );
            )*
            coordinates
        }
    };
}
