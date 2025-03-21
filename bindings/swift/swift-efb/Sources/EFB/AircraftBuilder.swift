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
            .init(efb_station_arm(station).pointee)
        }
        set(newArm) {
            efb_station_set_arm(station, EfbLength(length: newArm))
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

public struct FuelTank: Identifiable {
    let tank: OpaquePointer!

    public let id: UUID = UUID()

    public var arm: Measurement<UnitLength> {
        get {
            .init(efb_fuel_tank_arm(tank).pointee)
        }
        set(newArm) {
            efb_fuel_tank_set_arm(tank, EfbLength(length: newArm))
        }
    }

    public var capacity: Measurement<UnitVolume> {
        get {
            .init(efb_fuel_tank_capacity(tank).pointee)
        }
        set(newCapacity) {
            efb_fuel_tank_set_capacity(tank, EfbVolume(volume: newCapacity))
        }
    }

    init(_ tank: OpaquePointer) {
        self.tank = tank
    }
}

public struct CGLimit: Identifiable {
    let limit: OpaquePointer!

    public let id: UUID = UUID()

    public var mass: Measurement<UnitMass> {
        get {
            .init(efb_cg_limit_mass(limit).pointee)
        }
        set(newMass) {
            efb_cg_limit_set_mass(limit, EfbMass(mass: newMass))
        }
    }

    public var distance: Measurement<UnitLength> {
        get {
            .init(efb_cg_limit_distance(limit).pointee)
        }
        set(newDistance) {
            efb_cg_limit_set_distance(limit, EfbLength(length: newDistance))
        }
    }

    public init(_ limit: OpaquePointer) {
        self.limit = limit
    }
}

public class AircraftBuilder {
    private let builder: OpaquePointer!

    public init() {
        builder = efb_aircraft_builder_new()
    }

    deinit {
        efb_aircraft_builder_free(builder)
    }

    public var registration: String = "" {
        didSet {
            efb_aircraft_builder_registration(builder, registration)
        }
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
                efb_aircraft_builder_empty_balance(builder, EfbLength(length: distance))
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

    public func appendStation(arm: Measurement<UnitLength>, description: String?) -> Station {
        Station(
            efb_aircraft_builder_stations_push(
                builder,
                EfbLength(length: arm),
                description
            )
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

    /// Returns all fuel tanks.
    public func tanks() -> [FuelTank] {
        var tanks: [FuelTank] = []

        if let tank = efb_aircraft_builder_tanks_first(self.builder) {
            tanks.append(FuelTank(tank))

            while let tank = efb_aircraft_builder_tanks_next(self.builder) {
                tanks.append(FuelTank(tank))
            }
        }

        return tanks
    }

    /// Appends a new fuel tank with the arm and capacity and returns it.
    public func appendTank(arm: Measurement<UnitLength>, capacity: Measurement<UnitVolume>)
        -> FuelTank
    {
        FuelTank(
            efb_aircraft_builder_tanks_push(
                builder, EfbVolume(volume: capacity), EfbLength(length: arm))
        )
    }

    /// Removes the fuel tank at the index.
    public func removeTank(at: Int) {
        efb_aircraft_builder_tanks_remove(builder, at)
    }

    // MARK: - Center of Gravity

    /// Returns all Center of Gravity limits.
    public func cgEnvelope() -> [CGLimit] {
        var limits: [CGLimit] = []

        if let limit = efb_aircraft_builder_cg_envelope_first(self.builder) {
            limits.append(CGLimit(limit))

            while let limit = efb_aircraft_builder_cg_envelope_next(self.builder) {
                limits.append(CGLimit(limit))
            }
        }

        return limits
    }

    public func appendCGLimit(mass: Measurement<UnitMass>, distance: Measurement<UnitLength>)
        -> CGLimit
    {
        CGLimit(
            efb_aircraft_builder_cg_envelope_push(
                builder,
                EfbMass(mass: mass),
                EfbLength(length: distance)
            )
        )
    }

    public func removeCGLimit(at: Int) {
        efb_aircraft_builder_cg_envelope_remove(builder, at)
    }

    // MARK: - Notes

    public var notes: String = "" {
        didSet {
            efb_aircraft_builder_notes(builder, notes)
        }
    }
}
