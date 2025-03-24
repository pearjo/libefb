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

// MARK: - Fuel

public struct Fuel {
    let fuelType: FuelType
    let mass: Measurement<UnitMass>
}

extension Fuel {
    init(_ efbFuel: EfbFuel) {
        self.fuelType = .init(efbFuel.fuel_type)
        self.mass = .init(efbFuel.mass)
    }
}

extension EfbFuel {
    init(_ fuel: Fuel) {
        self.init(fuel_type: EfbFuelType(fuel.fuelType), mass: EfbMass(mass: fuel.mass))
    }
}

public enum FuelFlow {
    case perHour(Fuel)
}

extension FuelFlow {
    init(_ efbFuelFlow: EfbFuelFlow) {
        switch efbFuelFlow.tag {
        case PerHour:
            self = .perHour(.init(efbFuelFlow.per_hour))
        default:
            fatalError("init(_:) for \(efbFuelFlow) has not been implemented")
        }
    }
}

extension EfbFuelFlow {
    init(_ fuelFlow: FuelFlow) {
        switch fuelFlow {
        case .perHour(let fuel):
            self = efb_fuel_flow_per_hour(EfbFuel(fuel))
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

// MARK: - Vertical Distance

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

extension EfbVerticalDistance {
    init(_ verticalDistance: VerticalDistance) {
        switch verticalDistance {
        case .altitude(let ft):
            self = efb_vertical_distance_altitude(ft)
        default:
            fatalError("init(_:) for \(verticalDistance) has not been implemented")
        }
    }
}

// MARK: - Wind

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
