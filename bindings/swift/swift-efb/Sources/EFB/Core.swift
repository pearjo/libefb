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

public enum EFBError: Error {
    case unknownValue
}

public enum Angle {
    case trueNorth(Float)
    case magneticNorth(Float)

    init(_ efbAngle: EfbAngle) throws {
        switch efbAngle.tag {
        case TrueNorth:
            self = .trueNorth(efbAngle.true_north)
        case MagneticNorth:
            self = .magneticNorth(efbAngle.magnetic_north)
        default:
            throw EFBError.unknownValue
        }
    }
}

public enum Distance {
    case meter(Float)
    case nauticalMiles(Float)

    init(_ efbDistance: EfbDistance) throws {
        switch efbDistance.tag {
        case Meter:
            self = .meter(efbDistance.meter)
        case NauticalMiles:
            self = .nauticalMiles(efbDistance.nautical_miles)
        default:
            throw EFBError.unknownValue
        }
    }
}

public struct Duration {
    let hours: UInt8
    let minuts: UInt8
    let seconds: UInt8

    init(_ efbDuration: EfbDuration) {
        self.hours = efbDuration.hours
        self.minuts = efbDuration.minutes
        self.seconds = efbDuration.seconds
    }
}

public enum Speed {
    case knots(Float)
    case meterPerSecond(Float)
    case mach(Float)

    init(_ efbSpeed: EfbSpeed) throws {
        switch efbSpeed.tag {
        case Knots:
            self = .knots(efbSpeed.knots)
        case MeterPerSecond:
            self = .meterPerSecond(efbSpeed.meter_per_second)
        case Mach:
            self = .mach(efbSpeed.mach)
        default:
            throw EFBError.unknownValue
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

    init(_ efbVerticalDistance: EfbVerticalDistance) throws {
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
            throw EFBError.unknownValue
        }
    }
}

public class Wind {
    let direction: Angle
    let speed: Speed

    init(_ efbWind: EfbWind) throws {
        self.direction = try Angle(efbWind.direction)
        self.speed = try Speed(efbWind.speed)
    }
}
