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

from ctypes import (
    POINTER,
    c_char_p,
    c_int,
    c_void_p,
)

from .lib import lib
from .core import InputFormat
from .route import Route
# from .flight_planning import FlightPlanningBuilder, FlightPlanning


lib.efb_fms_new.restype = POINTER(c_void_p)
lib.efb_fms_free.argtypes = [POINTER(c_void_p)]
lib.efb_fms_nd_read.argtypes = [POINTER(c_void_p), c_char_p, c_int]
lib.efb_fms_nd_read_file.argtypes = [POINTER(c_void_p), c_char_p, c_int]
lib.efb_fms_decode.argtypes = [POINTER(c_void_p), c_char_p]
lib.efb_fms_route_ref.argtypes = [POINTER(c_void_p)]
lib.efb_fms_route_ref.restype = POINTER(Route)
lib.efb_fms_route_unref.argtypes = [POINTER(Route)]
lib.efb_fms_print.argtypes = [POINTER(c_void_p), c_int]
lib.efb_fms_print.restype = c_char_p


class FMS():
    def __init__(self):
        self._ptr = lib.efb_fms_new()

    def __del__(self):
        lib.efb_fms_free(self._ptr)

    def nd_read(self, s: str, fmt: InputFormat):
        lib.efb_fms_nd_read(self._ptr, s.encode("utf-8"), fmt)

    def nd_read_file(self, path: str, fmt: InputFormat):
        lib.efb_fms_nd_read_file(self._ptr, path.encode("utf-8"), fmt)

    def decode(self, route: str):
        lib.efb_fms_decode(self._ptr, route.encode("utf-8"))

    def route(self) -> Route:
        return lib.efb_fms_route_ref(self._ptr).contents

    def print(self, line_length):
        return lib.efb_fms_print(self._ptr, line_length).decode("utf-8")

    # def build_flight_planning(self, flight_planning_builder):
    #     lib.efb_fms_flight_planning_build(
    #         self._ptr, flight_planning_builder.builder
    #     )

    # def flight_planning(self) -> FlightPlanning:
    #     return FlightPlanning(
    #         lib.efb_fms_flight_planning(self._ptr)
    #     )
