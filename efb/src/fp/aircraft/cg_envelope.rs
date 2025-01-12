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

use super::MassAndBalance;
use crate::algorithm;
use crate::{Distance, Mass, Unit};

/// An aircraft's center of gravity (CG) envelope.
///
/// The envelope draws a polygon in a coordinate system with the mass and
/// balance as axis. It contains a CG for a mass if the aircraft is balanced on
/// ramp and after landing.
///
/// # Examples
///
/// ```
/// use efb::{Mass, Distance};
/// use efb::fp::{CGEnvelope, Station, MassAndBalance};
///
/// // This is how an envelope of a C172 might look like:
/// //
/// // M     2--------------3
/// // a    /               |
/// // s   /                |
/// // s  1                 |
/// //    |                 |
/// //    |                 |
/// //    0-----------------4
/// //
/// //               Distance
/// //
/// let cg_envelope = CGEnvelope::new(vec![
///     (Mass::Kilogram(0.0), Distance::Meter(0.89)),    // 0
///     (Mass::Kilogram(885.0), Distance::Meter(0.89)),  // 1
///     (Mass::Kilogram(1111.0), Distance::Meter(1.02)), // 2
///     (Mass::Kilogram(1111.0), Distance::Meter(1.20)), // 3
///     (Mass::Kilogram(0.0), Distance::Meter(1.20)),    // 4
/// ]);
///
/// // now we calculate the mass & balance which we want to check against our envelope
/// let mb = MassAndBalance::new(&vec![
///     // just for this example we simplify our aircraft as one station
///     Station {
///         // we start our journey with the pilot and some fuel on board
///         on_ramp: Mass::Kilogram(897.0),
///         // we burned 10 kg on our little sight seeing trip
///         after_landing: Mass::Kilogram(887.0),
///         // and assumed that we and the fuel had an arm of 1.1 m from the reference datum
///         arm: Distance::Meter(1.1),
///     },
/// ]);
///
/// // now we can check if our CG is within the envelope
/// assert!(cg_envelope.contains(&mb));
/// ```
#[derive(Clone, Debug)]
pub struct CGEnvelope {
    points: Vec<(Mass, Distance)>,
}

impl CGEnvelope {
    /// Creates a new envelope from the points.
    pub fn new(points: Vec<(Mass, Distance)>) -> Self {
        Self { points }
    }

    /// Tests if the mass & balance is within this envelope.
    ///
    /// Returns `false` if one of the points on ramp or after landing is outside
    /// of the envelope.
    pub fn contains(&self, mb: &MassAndBalance) -> bool {
        // We see the envelope as a polygon where the mass describes the y-axis
        // and the balance the x-axis. The M&B on ramp and after landing is
        // considered to be a point within this envelope (polygon).
        let envelope: Vec<algorithm::Point> = self
            .points
            .iter()
            .map(|mb| algorithm::Point {
                x: mb.1.si(),
                y: mb.0.si(),
            })
            .collect();

        let wn = |mass: &Mass, balance: &Distance| -> i32 {
            algorithm::winding_number(
                &algorithm::Point {
                    x: balance.si(),
                    y: mass.si(),
                },
                &envelope,
            )
        };

        let wn_on_ramp = wn(mb.mass_on_ramp(), mb.balance_on_ramp());
        let wn_after_landing = wn(mb.mass_after_landing(), mb.balance_after_landing());

        // The envelope's winding number around the point is 0 if the point is
        // outside the envelope.
        wn_on_ramp != 0 && wn_after_landing != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fp::Station;

    #[test]
    fn contains_point() {
        // Lets test an envelope with the points 1 which is within the envelope
        // and balanced and point 2 which is without the envelope and
        // unbalanced:
        //
        //   2  +--------+
        //     /         |
        //    /          |
        //   +     1     |
        //   |           |
        //   +-----------+
        //
        let envelope = CGEnvelope::new(vec![
            (Mass::Kilogram(0.0), Distance::Meter(0.0)),
            (Mass::Kilogram(0.5), Distance::Meter(0.0)),
            (Mass::Kilogram(1.0), Distance::Meter(0.25)),
            (Mass::Kilogram(1.0), Distance::Meter(1.0)),
            (Mass::Kilogram(0.0), Distance::Meter(1.0)),
        ]);

        let balanced = MassAndBalance::new(&vec![Station {
            on_ramp: Mass::Kilogram(0.5),
            after_landing: Mass::Kilogram(0.5),
            arm: Distance::Meter(0.5),
        }]);

        let unbalanced = MassAndBalance::new(&vec![Station {
            on_ramp: Mass::Kilogram(1.0),
            after_landing: Mass::Kilogram(1.0),
            arm: Distance::Meter(0.0),
        }]);

        assert!(
            envelope.contains(&balanced),
            "envelope didn't contain the balanced M&B"
        );
        assert!(
            !envelope.contains(&unbalanced),
            "envelope contain the unbalanced M&B"
        );
    }
}
