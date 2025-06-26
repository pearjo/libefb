# SPDX-License-Identifier: Apache-2.0
# Copyright 2025 Joe Pearson
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

from efb import *

ARINC_424_RECORDS = """SEURP EDDHEDA        0        N N53374900E009591762E002000053                   P    MWGE    HAMBURG                       124362502
SEURPCEDDHED D     ED0    V     N53333297E010093201                                 WGE           DELTA                    124732502
SEURPCEDHLED L     ED0    V     N53410811E010375478                                 WGE           LIMA                     126532502
SEURP EDHLEDA        0        N N53481800E010430400E002000055                   P    MWGE    LUBECK-BLANKENSEE             127032502"""

fms = FMS()

# read the ARINC database
fms.nd_read(ARINC_424_RECORDS, InputFormat.ARINC_424)

# decode a route from EDDH to EDHL with winds at 7 kt from 260° and cruising
# speed of 107 kt and an altitude of 2200 ft.
fms.decode("26007KT N0107 A0220 EDDH DHD HLL EDHL")

d_eabc = Aircraft(
    registration="D-EABC",
    # the stations where payload can be loaded to
    stations=[
        Station(Meter(0.94), "the front seats"),
        Station(Meter(1.85), "the back seats"),
        Station(Meter(2.41), "the first cargo compartment"),
        Station(Meter(3.12), "the second cargo compartment")
    ],
    empty_mass=Kilogram(807.0),
    empty_balance=Meter(1.0),
    fuel_type=FuelType.DIESEL,
    # the wing tanks are combined as one
    tanks=[FuelTank(Liter(168.8), Meter(1.22))],
    # this defines the limits of our Center of Gravity envelope
    cg_envelope=[
        CGLimit(Kilogram(0.0), Meter(0.89)),
        CGLimit(Kilogram(885.0), Meter(0.89)),
        CGLimit(Kilogram(1111.0), Meter(1.02)),
        CGLimit(Kilogram(1111.0), Meter(1.20)),
        CGLimit(Kilogram(0.0), Meter(1.20))
    ],
    notes=""
)

flight_planning = FlightPlanningBuilder(
    aircraft=d_eabc,
    # we're sitting alone in the front seat
    mass=[Kilogram(80), Kilogram(0), Kilogram(0), Kilogram(0)],
    policy=ManualFuel(Diesel(Liter(80))),
    # for taxiing we add a buffer and plan with 10 liters
    taxi=Diesel(Liter(10)),
    # we want a fuel reserve of 30 minutes
    reserve=ManualReserve(Duration(1800)),
    perf=Performance([
        # this is a very incomplete performance table
        (Altitude(1000), Knots(107), PerHour(Diesel(Liter(21))))
    ]),
)

fms.build_flight_planning(flight_planning)
print(fms.print(40))
