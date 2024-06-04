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

import EFB

extension EFB.Color: Hashable {
    static func == (lhs: EFB.Color, rhs: EFB.Color) -> Bool {
        return lhs.rawValue == rhs.rawValue
    }

    public func hash(into hasher: inout Hasher) {
        hasher.combine(self.rawValue)
    }
}

extension EFB.LineWidth: Hashable {
    static func == (lhs: EFB.LineWidth, rhs: EFB.LineWidth) -> Bool {
        return lhs.rawValue == rhs.rawValue
    }

    public func hash(into hasher: inout Hasher) {
        hasher.combine(self.rawValue)
    }
}

extension EFB.AirspaceClassification: Hashable {
    static func == (lhs: EFB.AirspaceClassification, rhs: EFB.AirspaceClassification) -> Bool {
        return lhs.rawValue == rhs.rawValue
    }

    public func hash(into hasher: inout Hasher) {
        hasher.combine(self.rawValue)
    }
}

public func parseOpenair(path: String) -> [Airspace] {
    let airspaces = parse_openair(path)

    return Array(
        UnsafeBufferPointer(
            start: airspaces.data,
            count: airspaces.len
        )
    ).map { airspace in
        Airspace(airspace)
    }
}
