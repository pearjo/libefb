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

from efb.aircraft import AircraftBuilder, FuelTank
from efb.core import *
from efb.fms import FMS, InputFormat

ARINC_424_RECORDS = """SEURP EDDHEDA        0        N N53374900E009591762E002000053                   P    MWGE    HAMBURG                       356462409
SEURPCEDDHED N1    ED0    V     N53482105E010015451                                 WGE           NOVEMBER1                359892409
SEURPCEDDHED N2    ED0    V     N53405701E010000576                                 WGE           NOVEMBER2                359902409
SEURP EDHFEDA        0        N N53593300E009343600E000000082                   P    MWGE    ITZEHOE/HUNGRIGER WOLF        320782409"""

fms = FMS()

# read the ARINC database
fms.nd_read(ARINC_424_RECORDS, InputFormat.ARINC_424)

# decode a route from EDDH to EDHF with winds at 20 kt from 290° and cruising
# speed of 107 kt and an altitude of 2500 ft.
fms.decode("29020KT N0107 A0250 EDDH DHN2 DHN1 EDHF")

aircraft_builder = AircraftBuilder(
    stations=[
        (meter(0.94), "the front seats"),
        (meter(1.85), "the back seats"),
        (meter(2.41), "the first cargo compartment"),
        (meter(3.12), "the second cargo compartment")
    ],
    empty_mass=kilogram(807.0),
    empty_balance=meter(1.0),
    fuel_type=FuelType.DIESEL,
    tanks=[FuelTank(liter(168.8), meter(1.22))]
)

print(fms.print(40))
