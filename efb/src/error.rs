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
    // Errors that can occur while decoding a route:
    //
    /// The entered flight plan does not include a cruise speed as one of the
    /// first two elements.
    ExpectedSpeedOnFPL,
    /// The entered flight plan does not include a cruise level as one of the
    /// first two elements.
    ExpectedLevelOnFPL,
    /// The route to decode includes an element that was not expected.
    UnexpectedRouteElement,
    /// The route includes a runway at a position that is not next to an
    /// airport.
    UnexpectedRunwayInRoute,

    // Errors that are related to parsing of input data:
    //
    /// The string that should be parsed to create some type is malformed.
    UnexpectedString,

    // Errors that relate to navigation data:
    //
    /// The requested identifier is not know.
    UnknownIdent,

    // Errors that originate from the mass & balance planning:
    //
    /// The number of masses doesn't match the number of stations to which the
    /// masses are assigned.
    UnexpectedMassesForStations,
    /// The number of provided fuel stations doesn't match the aircraft's fuel
    /// stations.
    UnexpectedNumberOfFuelStations,
    /// The planned fuel on ramp exceeds the tank's capacity.
    ExceededFuelCapacityOnRamp,
    /// The planned fuel after landing exceeds the tank's capacity.
    ExceededFuelCapacityAfterLanding,

    // Errors that can occur while building an aircraft:
    //
    /// The aircraft's registration is not set.
    ExpectedRegistration,
    /// The aircraft's empty mass is not set.
    ExpectedEmptyMass,
    /// The aircraft's empty balance is not set.
    ExpectedEmptyBalance,
    /// The aircraft's fuel type is not set.
    ExpectedFuelType,
}
