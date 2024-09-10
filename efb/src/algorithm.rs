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

//! Algorithms.

use crate::geom::{Coordinate, Line};

/// Returns `0` if the point is outside the polygon.
/// [ref]: https://web.archive.org/web/20130126163405/http://geomalgorithms.com/a03-_inclusion.html
pub fn winding_number(p: &Coordinate, v: &Vec<Coordinate>) -> i32 {
    let mut wn = 0;

    for i in 0..(v.len() - 1) {
        if v[i].latitude <= p.latitude {
            if v[i + 1].latitude > p.latitude {
                // an upward crossing
                if is_left_of_line(p, &(v[i], v[i + 1])) > 0.0 {
                    wn += 1;
                }
            }
        } else {
            if v[i + 1].latitude <= p.latitude {
                // a downward crossing
                if is_left_of_line(p, &(v[i], v[i + 1])) < 0.0 {
                    wn -= 1;
                }
            }
        }
    }

    wn
}

fn is_left_of_line(point: &Coordinate, line: &Line) -> f32 {
    (line.1.longitude - line.0.longitude) * (point.latitude - line.0.latitude)
        - (point.longitude - line.0.longitude) * (line.1.latitude - line.0.latitude)
}

#[cfg(test)]
mod tests {
    use crate::coord;

    use super::*;

    #[test]
    fn point_is_left_of_line() {
        let line = (coord!(10.0, 10.0), coord!(20.0, 10.0));
        let point = coord!(15.0, 5.0);
        assert!(is_left_of_line(&point, &line) > 0.0);
    }

    #[test]
    fn point_is_right_of_line() {
        let line = (coord!(10.0, 10.0), coord!(20.0, 10.0));
        let point = coord!(15.0, 15.0);
        assert!(is_left_of_line(&point, &line) < 0.0);
    }

    #[test]
    fn point_is_on_line() {
        let line = (coord!(10.0, 10.0), coord!(20.0, 10.0));
        let point = coord!(15.0, 10.0);
        assert_eq!(is_left_of_line(&point, &line), 0.0);
    }
}
