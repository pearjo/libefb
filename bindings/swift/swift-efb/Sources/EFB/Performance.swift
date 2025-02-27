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

public struct PerformanceTableRow {
    let row: OpaquePointer!

    init(_ row: OpaquePointer!) {
        self.row = row
    }

    public var trueAirspeed: Speed {
        get {
            .init(efb_performance_table_row_tas(row).pointee)
        }
        set(tas) {
            efb_performance_table_row_set_tas(row, EfbSpeed(tas))
        }
    }

    public var fuelFlow: FuelFlow {
        get {
            .init(efb_performance_table_row_ff(row).pointee)
        }
        set(ff) {
            efb_performance_table_row_set_ff(row, EfbFuelFlow(ff))
        }
    }
}

public class PerformanceTable {
    let table: OpaquePointer!

    public func appendRow(level: VerticalDistance, tas: Speed, ff: FuelFlow)
        -> PerformanceTableRow
    {
        PerformanceTableRow(
            efb_performance_table_push(
                table, EfbVerticalDistance(level), EfbSpeed(tas), EfbFuelFlow(ff))
        )
    }

    public func removeRow(at: Int) {
        efb_performance_table_remove(table, at)
    }

    init() {
        self.table = efb_performance_table_new()
    }

    deinit {
        efb_performance_table_free(table)
    }
}
