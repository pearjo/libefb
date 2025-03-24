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

// MARK: - Length

extension Measurement<UnitLength> {
    init(_ efbLength: EfbLength) {
        switch efbLength.unit {
        case Meters:
            self.init(value: Double(efbLength.value), unit: .meters)
        case NauticalMiles:
            self.init(value: Double(efbLength.value), unit: .nauticalMiles)
        default:
            fatalError("init(_:) for \(efbLength.unit) has not been implemented")
        }
    }
}

extension EfbLength {
    init(length: Measurement<UnitLength>) {
        switch length.unit {
        case .meters:
            self = efb_length_m(Float(length.value))
        case .nauticalMiles:
            self = efb_length_nm(Float(length.value))
        default:
            self.init(length: length.converted(to: .meters))
        }
    }
}
    
// MARK: - Mass

extension Measurement<UnitMass> {
    init(_ efbMass: EfbMass) {
        switch efbMass.unit {
        case Kilograms:
            self.init(value: Double(efbMass.value), unit: .kilograms)
        default:
            fatalError("init(_:) for \(efbMass.unit) has not been implemented")
        }
    }
}

extension EfbMass {
    init(mass: Measurement<UnitMass>) {
        switch mass.unit {
        case .kilograms:
            self = efb_mass_kg(Float(mass.value))
        default:
            self.init(mass: mass.converted(to: .kilograms))
        }
    }
}

// MARK: - Volume

extension Measurement<UnitVolume> {
    init(_ efbVolume: EfbVolume) {
        switch efbVolume.unit {
        case Liter:
            self.init(value: Double(efbVolume.value), unit: .liters)
        default:
            fatalError("init(_:) for \(efbVolume.unit) has not been implemented")
        }
    }
}

extension EfbVolume {
    init(volume: Measurement<UnitVolume>) {
        switch volume.unit {
        case .liters:
            self = efb_volume_l(Float(volume.value))
        default:
            self.init(volume: volume.converted(to: .liters))
        }
    }
}
