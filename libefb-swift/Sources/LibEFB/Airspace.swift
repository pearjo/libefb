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

import CoreLocation
import EFB
import Foundation

extension CLLocationCoordinate2D {
    init(_ coordinate: EFB.Coordinate) {
        self.init(
            latitude: Double(coordinate.latitude),
            longitude: Double(coordinate.longitude)
        )
    }
}

public enum VerticalDistance {
    case agl(Int)
    case altitude(Int)
    case fl(Int)
    case gnd
    case msl(Int)
    case unlimited
}

extension VerticalDistance {
    init(_ value: EFB.VerticalDistance) {
        switch value.tag {
        case EFB.Agl:
            self = .agl(Int(value.agl))
        case EFB.Altitude:
            self = .altitude(Int(value.altitude))
        case EFB.Fl:
            self = .fl(Int(value.fl))
        case EFB.Gnd:
            self = .gnd
        case EFB.Msl:
            self = .msl(Int(value.msl))
        case EFB.Unlimited:
            self = .unlimited
        default:
            self = .unlimited
        }
    }
}

public struct Airspace: Identifiable {
    public let id = UUID()
    public let name: String
    public let classification: AirspaceClassification
    public let coordinates: [CLLocationCoordinate2D]
    public let ceiling: VerticalDistance
    public let floor: VerticalDistance
    public let annotation: AirspaceAnnotation

    init(_ airspace: EFB.Airspace) {
        let polygon = Array(
            UnsafeBufferPointer(
                start: airspace.polygon.data,
                count: airspace.polygon.len))

        self.name = String(cString: airspace.name)
        self.classification = airspace.classification

        self.coordinates = polygon.map { coordinate in
            CLLocationCoordinate2D.init(coordinate)
        }

        self.ceiling = VerticalDistance.init(airspace.ceiling)
        self.floor = VerticalDistance.init(airspace.floor)
        self.annotation = airspace_annotation(airspace.classification)

        airspace_free(airspace)
    }
}
