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

import efb

public class Route {
    let route: OpaquePointer!

    public init(route: OpaquePointer!) {
        self.route = route
    }

    /// The route's total distance.
    ///
    /// The distance is none if the route has no legs.
    ///
    /// - Returns: The optional distance of the route.
    public func dist() -> Length? {
        efb_route_totals_dist(self.route).map { (dist) -> Length in
            Length(dist.pointee)
        }
    }

    /// The estimated time enroute (ETE).
    ///
    /// The ETE can only be calculated if a wind, speed and level enroute are known for the leg.
    ///
    /// - Returns: The optional estimated time enroute.
    public func ete() -> Duration? {
        efb_route_totals_ete(self.route).map { (ete) -> Duration in
            Duration(ete.pointee)
        }
    }

    public func legs() -> [Leg] {
        var legs: [Leg] = []

        if let leg = efb_route_legs_first(route) {
            legs.append(Leg(leg: leg))

            while let leg = efb_route_legs_next(route) {
                legs.append(Leg(leg: leg))
            }
        }

        return legs
    }

    deinit {
        efb_fms_route_unref(route)
    }
}

public struct Leg {
    public let from: String
    public let to: String
    public let level: VerticalDistance?
    public let wind: Wind?
    public let tas: Speed?
    public let heading: Angle?
    public let magneticHeading: Angle?
    public let bearing: Angle
    public let magneticCourse: Angle
    public let distance: Length
    public let groundSpeed: Speed?
    public let ete: Duration?

    public init(leg: OpaquePointer!) {
        let cFrom = efb_leg_get_from(leg)
        let cTo = efb_leg_get_to(leg)

        defer {
            efb_string_free(cFrom)
            efb_string_free(cTo)
        }

        from = String(cString: cFrom!)
        to = String(cString: cTo!)

        level = efb_leg_get_level(leg).map { (level) -> VerticalDistance in
            VerticalDistance(level.pointee)
        }

        wind = efb_leg_get_wind(leg).map { (wind) -> Wind in
            Wind(wind.pointee)
        }

        tas = efb_leg_get_tas(leg).map { (tas) -> Speed in
            Speed(tas.pointee)
        }

        heading = efb_leg_get_heading(leg).map { (heading) -> Angle in
            Angle(heading.pointee)
        }

        magneticHeading = efb_leg_get_mh(leg).map { (mh) -> Angle in
            Angle(mh.pointee)
        }

        bearing = Angle(efb_leg_get_bearing(leg).pointee)
        magneticCourse = Angle(efb_leg_get_mc(leg).pointee)
        distance = Length(efb_leg_get_dist(leg).pointee)

        groundSpeed = efb_leg_get_gs(leg).map { (gs) -> Speed in
            Speed(gs.pointee)
        }

        ete = efb_leg_get_ete(leg).map { (ete) -> Duration in
            Duration(ete.pointee)
        }
    }
}
