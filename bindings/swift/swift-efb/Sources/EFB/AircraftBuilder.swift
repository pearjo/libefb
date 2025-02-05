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
import Observation
import efb

public struct Station: Identifiable {
    public var id: UUID = UUID()
    public var description: String
    public var arm: Measurement<UnitLength>

    public init(description: String, arm: Measurement<UnitLength>) {
        self.description = description
        self.arm = arm
    }
}

public struct Tank {
    let arm: Measurement<UnitLength>
    let capacity: Measurement<UnitVolume>
}

@available(iOS 17.0, macOS 14.0, *)
@Observable
public class AircraftBuilder {
    private let builder: OpaquePointer!

    public init() {
        builder = efb_aircraft_builder_new()
    }

    deinit {
        efb_aircraft_builder_free(builder)
    }

    // MARK: - Empty Weight & Balance

    public var emptyMass: Measurement<UnitMass>? {
        didSet {
            if let m = emptyMass {
                efb_aircraft_builder_empty_mass(builder, EfbMass(mass: m))
            }
        }
    }

    public var emptyBalance: Measurement<UnitLength>? {
        didSet {
            if let distance = emptyBalance {
                efb_aircraft_builder_empty_balance(builder, EfbDistance(length: distance))
            }
        }
    }

    // MARK: - Stations

    public private(set) var stations: [Station] = []

    public func appendStation(station: Station) {
        efb_aircraft_builder_station_arms_push(builder, EfbDistance(length: station.arm))
        stations.append(station)
    }

    public func removeStation(at: Int) {
        efb_aircraft_builder_station_arms_remove(builder, at)
        stations.remove(at: at)
    }

    public func replaceStation(with: Station, at: Int) {
        efb_aircraft_builder_station_arms_edit(builder, EfbDistance(length: with.arm), at)
        stations[at] = with
    }

    // MARK: - Fuel and Tanks

    public var fuelType: FuelType? {
        didSet {
            if let fuelType = fuelType {
                efb_aircraft_builder_fuel_type(builder, EfbFuelType(fuelType))
            }
        }
    }

    public private(set) var tanks: [Tank] = []

    public func appendTank(tank: Tank) {
        efb_aircraft_builder_tanks_push(
            builder, EfbVolume(volume: tank.capacity), EfbDistance(length: tank.arm))
        tanks.append(tank)
    }

    public func removeTank(at: Int) {
        efb_aircraft_builder_tanks_remove(builder, at)
        tanks.remove(at: at)
    }

    public func replaceTank(with: Tank, at: Int) {
        efb_aircraft_builder_tanks_edit(
            builder, EfbVolume(volume: with.capacity), EfbDistance(length: with.arm), at)
        tanks[at] = with
    }

    // MARK: - Center of Gravity

    public private(set) var cgEnvelope: [(mass: Measurement<UnitMass>, distance: Distance)] = []

    public func appendCGEnvelope(mass: Measurement<UnitMass>, distance: Distance) {
        efb_aircraft_builder_cg_envelope_push(builder, EfbMass(mass: mass), EfbDistance(distance))
        cgEnvelope.append((mass, distance))
    }

    public func removeCGEnvelope(at: Int) {
        efb_aircraft_builder_cg_envelope_remove(builder, at)
        cgEnvelope.remove(at: at)
    }

    public func replaceCGEnvelope(withMass: Measurement<UnitMass>, withDistance: Distance, at: Int)
    {
        efb_aircraft_builder_cg_envelope_edit(
            builder, EfbMass(mass: withMass), EfbDistance(withDistance), at)
        cgEnvelope[at] = (withMass, withDistance)
    }
}

#if DEBUG
    @available(iOS 17.0, macOS 14.0, *)
    extension AircraftBuilder {
        static public let testAircraftBuilder = {
            let builder = AircraftBuilder()

            builder.appendStation(
                station: Station(
                    description: "Front seats", arm: Measurement(value: 94, unit: .centimeters)))
            builder.appendStation(
                station: Station(
                    description: "Back seats", arm: Measurement(value: 185, unit: .centimeters)))
            builder.appendStation(
                station: Station(
                    description: "Baggage compartment",
                    arm: Measurement(value: 241, unit: .centimeters)))

            return builder
        }()
    }
#endif
