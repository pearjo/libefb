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

use crate::fp::Performance;
use crate::nd::{Fix, NavAid};
use crate::*;

/// A leg `from` one point `to` another.
#[derive(Clone, Debug)]
pub struct Leg {
    from: NavAid,
    to: NavAid,
    level: Option<VerticalDistance>,
    tas: Option<Speed>,
    wind: Option<Wind>,
    heading: Option<Angle>,
    mh: Option<Angle>,
    bearing: Angle,
    mc: Angle,
    dist: Distance,
    gs: Option<Speed>,
    wca: Option<Angle>,
    ete: Option<Duration>,
}

impl Leg {
    pub fn new(
        from: NavAid,
        to: NavAid,
        level: Option<VerticalDistance>,
        tas: Option<Speed>,
        wind: Option<Wind>,
    ) -> Leg {
        let bearing = from.coordinate().bearing(&to.coordinate());
        let mc = bearing + from.mag_var();
        let dist = from.coordinate().dist(&to.coordinate()).to_nm();

        let (gs, wca) = {
            match (tas, wind) {
                (Some(tas), Some(wind)) => {
                    let tas = tas.to_kt().into_inner();
                    let ws = wind.speed.to_kt().into_inner();

                    let wca: Angle = {
                        (ws / tas * (bearing - 180 + wind.direction).as_radians().sin())
                            .asin()
                            .into()
                    };

                    let gs = Speed::Knots(
                        (tas.powi(2) + ws.powi(2)
                            - ((2.0 * tas * ws)
                                * (bearing - wind.direction + wca).as_radians().cos()))
                        .sqrt(),
                    );

                    (Some(gs), Some(wca))
                }
                _ => (None, None),
            }
        };

        let heading = wca.map(|wca| bearing + wca);
        let mh = heading.map(|heading| heading + from.mag_var());
        let ete = gs.map(|gs| dist / gs);

        Self {
            from,
            to,
            level,
            tas,
            wind,
            heading,
            mh,
            bearing,
            mc,
            dist,
            gs,
            wca,
            ete,
        }
    }

    /// The point from which the leg starts.
    pub fn from(&self) -> &NavAid {
        &self.from
    }

    /// The point to which the leg is going.
    pub fn to(&self) -> &NavAid {
        &self.to
    }

    /// The level of the leg.
    pub fn level(&self) -> Option<&VerticalDistance> {
        self.level.as_ref()
    }

    /// The desired true airspeed (TAS).
    pub fn tas(&self) -> Option<&Speed> {
        self.tas.as_ref()
    }

    /// The wind to take into account.
    pub fn wind(&self) -> Option<&Wind> {
        self.wind.as_ref()
    }

    /// The true heading considering the wind correction angle (WCA).
    pub fn heading(&self) -> Option<&Angle> {
        self.heading.as_ref()
    }

    /// The magnetic heading considering the variation at the start of the leg.
    pub fn mh(&self) -> Option<&Angle> {
        self.mh.as_ref()
    }

    /// The bearing between the two points.
    pub fn bearing(&self) -> &Angle {
        &self.bearing
    }

    /// The magnetic course taking the magnetic variation from the starting
    /// point into consideration.
    pub fn mc(&self) -> &Angle {
        &self.mc
    }

    /// The distance between the leg's two points.
    pub fn dist(&self) -> &Distance {
        &self.dist
    }

    // TODO add test to verify calculation
    /// The ground speed in knots.
    pub fn gs(&self) -> Option<&Speed> {
        self.gs.as_ref()
    }

    // TODO add test to verify calculation
    /// The wind correction angle based on the wind.
    pub fn wca(&self) -> Option<&Angle> {
        self.wca.as_ref()
    }

    // TODO add test to verify calculation
    /// The estimated time enroute the leg.
    pub fn ete(&self) -> Option<&Duration> {
        self.ete.as_ref()
    }

    /// The [Fuel] consumed on the leg with the given [Performance].
    pub fn fuel(&self, perf: &Performance) -> Option<Fuel> {
        match (self.level, self.ete) {
            (Some(level), Some(ete)) => Some(perf.ff(&level) * ete),
            _ => None,
        }
    }
}
