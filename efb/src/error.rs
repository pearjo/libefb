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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Error {
    /// The entered flight plan does not include a cruise speed as one of the
    /// first two elements.
    ExpectedSpeedOnFPL,

    /// The entered flight plan does not include a cruise level as one of the
    /// first two elements.
    ExpectedLevelOnFPL,

    ExpectedRoute,

    ExpectedAircraft,
    ExpectedFuelPlanning,

    UnexpectedRouteElement,
    UnexpectedRunwayInRoute,

    UnexpectedString,

    UnexpectedInput,

    // navigation data
    /// The requested identifier is not know.
    UnknownIdent,

    // mass & balance
    /// The number of masses doesn't match the number of stations to which the
    /// masses are assigned.
    UnexpectedMassesForStations,

    UnexpectedNumberOfFuelStations,

    /// The planned fuel on ramp exceeds the tank's capacity.
    ExceededFuelCapacityOnRamp,

    /// The planned fuel after landing exceeds the tank's capacity.
    ExceededFuelCapacityAfterLanding,
}
