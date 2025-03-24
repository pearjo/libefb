// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Joe Pearson
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

use std::slice::Iter;

use efb::fp;
use efb::measurements::Speed;
use efb::{FuelFlow, VerticalDistance};

#[derive(Default)]
pub struct PerformanceTable<'a> {
    table: fp::PerformanceTable,
    table_iter: Option<Iter<'a, fp::PerformanceTableRow>>,
}

/// Returns a new performance table
///
/// Use the table to define the performance at different level.
///
/// # Safety
///
/// The memory allocated for the table needs to be freed by calling
/// [`efb_performance_table_free`].
#[no_mangle]
pub unsafe extern "C" fn efb_performance_table_new<'a>() -> Box<PerformanceTable<'a>> {
    Box::new(PerformanceTable::default())
}

/// Frees the performance table.
#[no_mangle]
pub extern "C" fn efb_performance_table_free(table: Box<PerformanceTable>) {
    drop(table);
}

#[no_mangle]
pub extern "C" fn efb_performance_table_push<'a>(
    table: &'a mut PerformanceTable,
    level: VerticalDistance,
    tas: Speed,
    ff: FuelFlow,
) -> Option<&'a fp::PerformanceTableRow> {
    table.table.push(fp::PerformanceTableRow { level, tas, ff });
    table.table.last()
}

#[no_mangle]
pub extern "C" fn efb_performance_table_remove(table: &mut PerformanceTable, at: usize) {
    table.table.remove(at);
}

/// Returns the first performance.
///
/// To iterate over the table, call [`efb_performance_table_next`]
/// until `NULL` is returned:
///
/// ```c
/// for (const EfbPerformanceTableRow *row = efb_performance_table_first(table);
///      row != NULL;
///      row = efb_performance_table_next(table))
/// ```
#[no_mangle]
pub extern "C" fn efb_performance_table_first<'a>(
    table: &'a mut PerformanceTable<'a>,
) -> Option<&'a fp::PerformanceTableRow> {
    table.table_iter.insert(table.table.iter()).next()
}

/// Returns the next performance.
///
/// When the end of the table is reached, this function returns a null pointer.
#[no_mangle]
pub extern "C" fn efb_performance_table_next<'a>(
    table: &'a mut PerformanceTable<'a>,
) -> Option<&'a fp::PerformanceTableRow> {
    table.table_iter.as_mut().and_then(|iter| iter.next())
}

#[no_mangle]
pub extern "C" fn efb_performance_table_row_tas(row: &fp::PerformanceTableRow) -> &Speed {
    &row.tas
}

#[no_mangle]
pub extern "C" fn efb_performance_table_row_set_tas(row: &mut fp::PerformanceTableRow, tas: Speed) {
    row.tas = tas
}

#[no_mangle]
pub extern "C" fn efb_performance_table_row_ff(row: &fp::PerformanceTableRow) -> &FuelFlow {
    &row.ff
}

#[no_mangle]
pub extern "C" fn efb_performance_table_row_set_ff(
    row: &mut fp::PerformanceTableRow,
    ff: FuelFlow,
) {
    row.ff = ff
}
