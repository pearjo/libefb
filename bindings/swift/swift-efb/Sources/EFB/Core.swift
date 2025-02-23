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

// TODO: Migrate custom Efb* types to Measurement where possible.

public enum Angle {
    case trueNorth(Float)
    case magneticNorth(Float)

    init(_ efbAngle: EfbAngle) {
        switch efbAngle.tag {
        case TrueNorth:
            self = .trueNorth(efbAngle.true_north)
        case MagneticNorth:
            self = .magneticNorth(efbAngle.magnetic_north)
        default:
            fatalError("Unimplemented EfbAngle \(efbAngle.tag)!")
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

public enum Distance {
    case meter(Float)
    case nauticalMiles(Float)

    init(_ efbDistance: EfbDistance) {
        switch efbDistance.tag {
        case Meter:
            self = .meter(efbDistance.meter)
        case NauticalMiles:
            self = .nauticalMiles(efbDistance.nautical_miles)
        default:
            fatalError("Unimplemented EfbDistance \(efbDistance.tag)!")
        }
    }
}

extension Distance: CustomStringConvertible {
    public var description: String {
        withUnsafePointer(to: EfbDistance(self)) {
            let cString = efb_distance_to_string($0)

            defer {
                efb_string_free(cString)
            }

            return String(cString: cString!)
        }
    }
}

extension EfbDistance {
    init(_ distance: Distance) {
        switch distance {
        case .meter(let m):
            self = efb_distance_m(m)
        case .nauticalMiles(let nm):
            self = efb_distance_nm(nm)
        }
    }
}

public struct Duration: CustomStringConvertible {
    let hours: UInt8
    let minuts: UInt8
    let seconds: UInt8
    public let description: String

    init(_ efbDuration: EfbDuration) {
        self.hours = efbDuration.hours
        self.minuts = efbDuration.minutes
        self.seconds = efbDuration.seconds

        self.description = withUnsafePointer(to: efbDuration) {
            let cString = efb_duration_to_string($0)

            defer {
                efb_string_free(cString)
            }

            return String(cString: cString!)
        }
    }
}

public enum FuelType {
    case diesel
    case jetA

    init(_ efbFuelType: EfbFuelType) {
        switch efbFuelType {
        case Diesel:
            self = .diesel
        case JetA:
            self = .jetA
        default:
            fatalError("init(_:) for \(efbFuelType) has not been implemented")
        }
    }
}

extension EfbFuelType {
    init(_ fuelType: FuelType) {
        switch fuelType {
        case .diesel:
            self = Diesel
        case .jetA:
            self = JetA
        }
    }
}

public enum Speed {
    case knots(Float)
    case meterPerSecond(Float)
    case mach(Float)

    init(_ efbSpeed: EfbSpeed) {
        switch efbSpeed.tag {
        case Knots:
            self = .knots(efbSpeed.knots)
        case MeterPerSecond:
            self = .meterPerSecond(efbSpeed.meter_per_second)
        case Mach:
            self = .mach(efbSpeed.mach)
        default:
            fatalError("Unimplemented EfbSpeed \(efbSpeed.tag)!")
        }
    }
}

public enum VerticalDistance {
    case agl(UInt16)
    case altitude(UInt16)
    case fl(UInt16)
    case gnd
    case msl(UInt16)
    case unlimited

    init(_ efbVerticalDistance: EfbVerticalDistance) {
        switch efbVerticalDistance.tag {
        case Agl:
            self = .agl(efbVerticalDistance.agl)
        case Altitude:
            self = .altitude(efbVerticalDistance.altitude)
        case Fl:
            self = .fl(efbVerticalDistance.fl)
        case Gnd:
            self = .gnd
        case Msl:
            self = .msl(efbVerticalDistance.msl)
        case Unlimited:
            self = .unlimited
        default:
            fatalError("Unimplemented EfbVerticalDistance \(efbVerticalDistance.tag)!")
        }
    }
}

public class Wind: CustomStringConvertible {
    let direction: Angle
    let speed: Speed
    public let description: String

    init(_ efbWind: EfbWind) {
        self.direction = Angle(efbWind.direction)
        self.speed = Speed(efbWind.speed)

        self.description = withUnsafePointer(to: efbWind) {
            let cString = efb_wind_to_string($0)

            defer {
                efb_string_free(cString)
            }

            return String(cString: cString!)
        }
    }
}
