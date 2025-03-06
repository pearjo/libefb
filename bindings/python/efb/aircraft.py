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

from ctypes import POINTER, c_void_p, c_char_p

from .core import *
from .lib import lib


class FuelTank:
    def __init__(self, capacity, arm):
        self.capacity = capacity
        self.arm = arm


class AircraftBuilder:
    def __init__(self,
                 stations=[],
                 empty_mass=None,
                 empty_balance=None,
                 fuel_type=None,
                 tanks=[]):
        self.stations = stations
        self.empty_mass = empty_mass
        self.empty_balance = empty_balance
        self.fuel_type = fuel_type
        self.tanks = tanks
        self._ptr = None

    def __del__(self):
        if self._ptr is not None:
            lib.efb_aircraft_builder_free(self._ptr)

    def builder(self):
        if self._ptr is None:
            self._ptr = lib.efb_aircraft_builder_new()

        for (arm, description) in self.stations:
            lib.efb_aircraft_builder_stations_push(self._ptr, arm, description)

        if empty_mass := self.empty_mass:
            lib.efb_aircraft_builder_set_empty_mass(self._ptr, empty_mass)

        if empty_balance := self.empty_balance:
            lib.efb_aircraft_builder_set_empty_balance(self._ptr, empty_balance)

        if fuel_type := self.fuel_type:
            lib.efb_aircraft_builder_set_fuel_type(self._ptr, fuel_type)

        return self._ptr


lib.efb_aircraft_builder_new.restype = POINTER(c_void_p)
lib.efb_aircraft_builder_free.argtypes = [POINTER(c_void_p)]
lib.efb_aircraft_builder_stations_push.argtypes = [POINTER(c_void_p), Distance, c_char_p]
lib.efb_aircraft_builder_empty_mass.argtypes = [POINTER(c_void_p), Mass]
lib.efb_aircraft_builder_empty_balance.argtypes = [POINTER(c_void_p), Distance]
lib.efb_aircraft_builder_fuel_type.argtypes = [POINTER(c_void_p), c_int]
lib.efb_aircraft_builder_tanks_push.argtypes = [POINTER(c_void_p), Volume, Distance]
lib.efb_aircraft_builder_cg_envelope_push.argtypes = [POINTER(c_void_p), Mass, Distance]
