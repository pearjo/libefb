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

import Foundation
import efb

// TODO: Use custom types but add formatter for them.

// MARK: - Angle

public enum Angle {
    case trueNorth(Float)
    case magneticNorth(Float)

    init(_ efbAngle: EfbAngle) {
        switch efbAngle.unit {
        case TrueNorth:
            self = .trueNorth(efbAngle.value)
        case MagneticNorth:
            self = .magneticNorth(efbAngle.value)
        default:
            fatalError("Unimplemented EfbAngle \(efbAngle.unit)!")
        }
    }
}

extension Angle: CustomStringConvertible {
    public var description: String {
        withUnsafePointer(to: EfbAngle(self)) {
            let cString = efb_angle_to_string($0)

            defer {
                efb_string_free(cString)
            }

            return String(cString: cString!)
        }
    }
}

extension EfbAngle {
    init(_ angle: Angle) {
        switch angle {
        case .trueNorth(let radians):
            self = efb_angle_true_north(radians)
        case .magneticNorth(let radians):
            self = efb_angle_magnetic_north(radians)
        }
    }
}

// MARK: - Length

public enum Length {
    case meter(Float)
    case nauticalMiles(Float)

    init(_ efbLength: EfbLength) {
        switch efbLength.unit {
        case Meters:
            self = .meter(efbLength.value)
        case NauticalMiles:
            self = .nauticalMiles(efbLength.value)
        default:
            fatalError("Unimplemented EfbLength \(efbLength.unit)!")
        }
    }
}

extension Length: CustomStringConvertible {
    public var description: String {
        withUnsafePointer(to: EfbLength(self)) {
            let cString = efb_length_to_string($0)

            defer {
                efb_string_free(cString)
            }

            return String(cString: cString!)
        }
    }
}

extension EfbLength {
    init(_ length: Length) {
        switch length {
        case .meter(let m):
            self = efb_length_m(m)
        case .nauticalMiles(let nm):
            self = efb_length_nm(nm)
        }
    }
}

// MARK: - Duration

public struct Duration: CustomStringConvertible {
    let hours: UInt32
    let minuts: UInt32
    let seconds: UInt32
    public let description: String

    init(_ efbDuration: EfbDuration) {
        self.hours = withUnsafePointer(to: efbDuration) {
             efb_duration_hours($0)
        }

        self.minuts = withUnsafePointer(to: efbDuration) {
            efb_duration_minutes($0)
        }
        
        self.seconds = withUnsafePointer(to: efbDuration) {
            efb_duration_seconds($0)
        }

        self.description = withUnsafePointer(to: efbDuration) {
            let cString = efb_duration_to_string($0)

            defer {
                efb_string_free(cString)
            }

            return String(cString: cString!)
        }
    }
}

// MARK: - Speed

public enum Speed {
    case knots(Float)
    case meterPerSecond(Float)
    case mach(Float)

    init(_ efbSpeed: EfbSpeed) {
        switch efbSpeed.unit {
        case Knots:
            self = .knots(efbSpeed.value)
        case MetersPerSecond:
            self = .meterPerSecond(efbSpeed.value)
        case Mach:
            self = .mach(efbSpeed.value)
        default:
            fatalError("Unimplemented EfbSpeed \(efbSpeed.unit)!")
        }
    }
}

extension EfbSpeed {
    init(_ speed: Speed) {
        switch speed {
        case .knots(let kt):
            self = efb_speed_knots(kt)
        case .meterPerSecond(let mps):
            self = efb_speed_mps(mps)
        case .mach(let mach):
            self = efb_speed_mach(mach)
        }
    }
}
