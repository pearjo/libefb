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

public struct Station: Identifiable {
    let station: OpaquePointer!

    public let id: UUID = UUID()

    public var arm: Measurement<UnitLength> {
        get {
            Measurement<UnitLength>(
              efb_station_arm(station).pointee
            )
        }
        set(newArm) {
            efb_station_set_arm(station, EfbDistance(length: newArm))
        }
    }

    public var description: String {
        get {
            efb_station_description(station).map { description in
                defer {
                    efb_string_free(description)
                }

                return String(cString: description)
            } ?? ""
        }
        set(newDescription) {
            efb_station_set_description(station, newDescription)
        }
    }

    init(_ station: OpaquePointer!) {
        self.station = station
    }
}

public struct Tank {
    let arm: Measurement<UnitLength>
    let capacity: Measurement<UnitVolume>
}

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

    public func stations() -> [Station] {
        var stations: [Station] = []

        if let station = efb_aircraft_builder_stations_first(self.builder) {
            stations.append(Station(station))

            while let station = efb_aircraft_builder_stations_next(self.builder) {
                stations.append(Station(station))
            }
        }

        return stations
    }

    public func appendStation(arm: Measurement<UnitLength>, description: String?) {
        efb_aircraft_builder_stations_push(
          builder,
          EfbDistance(length: arm),
          description
        )
    }

    public func removeStation(at: Int) {
        efb_aircraft_builder_stations_remove(builder, at)
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

    public func appendCGEnvelope(mass: Measurement<UnitMass>, distance: Distance) {
        efb_aircraft_builder_cg_envelope_push(builder, EfbMass(mass: mass), EfbDistance(distance))
    }

    public func removeCGEnvelope(at: Int) {
        efb_aircraft_builder_cg_envelope_remove(builder, at)
    }

    public func replaceCGEnvelope(withMass: Measurement<UnitMass>, withDistance: Distance, at: Int)
    {
        efb_aircraft_builder_cg_envelope_edit(
            builder, EfbMass(mass: withMass), EfbDistance(withDistance), at)
    }
}
