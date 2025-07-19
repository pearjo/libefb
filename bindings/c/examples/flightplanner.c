/* SPDX-License-Identifier: Apache-2.0 */
/* Copyright 2024 Joe Pearson
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/* This is the C version of the flightplanner example. */
#include "../include/efb.h"
#include <stdio.h>

const char *ARINC_424_RECORDS =
    "SEURP EDDHEDA        0        N N53374900E009591762E002000053             "
    "      P    MWGE    HAMBURG                       356462409\n"
    "SEURPCEDDHED N1    ED0    V     N53482105E010015451                       "
    "          WGE           NOVEMBER1                359892409\n"
    "SEURPCEDDHED N2    ED0    V     N53405701E010000576                       "
    "          WGE           NOVEMBER2                359902409\n"
    "SEURP EDHFEDA        0        N N53593300E009343600E000000082             "
    "      P    MWGE    ITZEHOE/HUNGRIGER WOLF        320782409";

// Performance setting with 65% load in cruise. This is the performance
// profile of a Cessna C172 with an TAE125-02-114 Diesel engine.
EfbPerformanceAtLevel
c172_tae125_02_114_at_65_percent_load(const EfbVerticalDistance *level) {
  EfbSpeed tas;

  const EfbVerticalDistance alt_10000 = efb_vertical_distance_altitude(10000);
  const EfbVerticalDistance alt_8000 = efb_vertical_distance_altitude(8000);
  const EfbVerticalDistance alt_6000 = efb_vertical_distance_altitude(6000);
  const EfbVerticalDistance alt_4000 = efb_vertical_distance_altitude(4000);

  if (efb_vertical_distance_gte(level, &alt_10000)) {
    tas = efb_speed_knots(114.0);
  } else if (efb_vertical_distance_gte(level, &alt_8000)) {
    tas = efb_speed_knots(112.0);
  } else if (efb_vertical_distance_gte(level, &alt_6000)) {
    tas = efb_speed_knots(110.0);
  } else if (efb_vertical_distance_gte(level, &alt_4000)) {
    tas = efb_speed_knots(109.0);
  } else {
    tas = efb_speed_knots(107.0);
  };

  EfbFuelFlow ff = {.tag = PerHour, .per_hour = efb_fuel_diesel_l(21.0)};
  EfbPerformanceAtLevel at_level = {.tas = tas, .ff = ff};

  return at_level;
}

int
main(int argc, char *argv[]) {
  EfbFMS *fms = efb_fms_new();

  // read the ARINC database
  efb_fms_nd_read(fms, ARINC_424_RECORDS, Arinc424);

  // decode a route from EDDH to EDHF with winds at 20 kt from 290° and
  // cruising speed of 107 kt and an altitude of 2500 ft.
  efb_fms_decode(fms, "29020KT N0107 A0250 EDDH DHN2 DHN1 EDHF");

  // Loading the database and decoding a route was simple so far. Now we get to
  // the part of the flight planning. This needs some more definitions like an
  // aircraft and performance data about how we want to plan the flight. Thus, a
  // lot of verbose definitions follow.

  // now we'll build a C172
  EfbAircraftBuilder *aircraft_builder = efb_aircraft_builder_new();

  efb_aircraft_builder_registration(aircraft_builder, "N12345");

  efb_aircraft_builder_stations_push(aircraft_builder, efb_length_m(0.94),
                                     "front seats");
  efb_aircraft_builder_stations_push(aircraft_builder, efb_length_m(1.85),
                                     "back seats");
  efb_aircraft_builder_stations_push(aircraft_builder, efb_length_m(2.41),
                                     "first cargo compartment");
  efb_aircraft_builder_stations_push(aircraft_builder, efb_length_m(3.12),
                                     "second cargo compartment");

  efb_aircraft_builder_empty_mass(aircraft_builder, efb_mass_kg(807.0));

  efb_aircraft_builder_empty_balance(aircraft_builder, efb_length_m(1.0));

  efb_aircraft_builder_fuel_type(aircraft_builder, Diesel);

  efb_aircraft_builder_tanks_push(aircraft_builder, efb_volume_l(168.8),
                                  efb_length_m(1.22));

  efb_aircraft_builder_cg_envelope_push(aircraft_builder, efb_mass_kg(0.0),
                                        efb_length_m(0.89));
  efb_aircraft_builder_cg_envelope_push(aircraft_builder, efb_mass_kg(885.0),
                                        efb_length_m(0.89));
  efb_aircraft_builder_cg_envelope_push(aircraft_builder, efb_mass_kg(1111.0),
                                        efb_length_m(1.02));
  efb_aircraft_builder_cg_envelope_push(aircraft_builder, efb_mass_kg(1111.0),
                                        efb_length_m(1.20));
  efb_aircraft_builder_cg_envelope_push(aircraft_builder, efb_mass_kg(0.0),
                                        efb_length_m(1.20));

  // Now we can enter some data into the flight planning to get a fuel planning
  // and mass & balance calculation.
  EfbFlightPlanningBuilder *builder = efb_flight_planning_builder_new();

  efb_flight_planning_builder_set_aircraft(builder, aircraft_builder);

  EfbMass mass[] = {
      // we're in the front
      efb_mass_kg(80.0),
      // and no mass on the other stations
      efb_mass_kg(0.0),
      efb_mass_kg(0.0),
      efb_mass_kg(0.0)
  };
  efb_flight_planning_builder_set_mass(builder, mass, 4);

  EfbFuelPolicy policy = {.tag = ManualFuel,
                          .manual_fuel = efb_fuel_diesel_l(80.0)};
  efb_flight_planning_builder_set_policy(builder, policy);
  efb_flight_planning_builder_set_taxi(builder, efb_fuel_diesel_l(10.0));

  EfbReserve reserve = {.tag = Manual, .manual = efb_duration(1800)}; // 30 min
  efb_flight_planning_builder_set_reserve(builder, reserve);

  efb_flight_planning_builder_set_perf(builder,
                                       c172_tae125_02_114_at_65_percent_load,
                                       // The data end at 10000 ft so we don't
                                       // need to create the Performance with
                                       // more values.
                                       efb_vertical_distance_altitude(10000));

  // now that all data are entered, we can build our planning
  efb_fms_set_flight_planning(fms, builder);

  // finally we can print out the result of our planning
  char *printout = efb_fms_print(fms, 40);
  printf("%s", printout);
  efb_string_free(printout);

  efb_flight_planning_builder_free(builder);
  efb_aircraft_builder_free(aircraft_builder);
  efb_fms_free(fms);

  return 0;
}
