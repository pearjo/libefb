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

from ctypes import POINTER, Structure, c_void_p

from .core import (
    Angle,
    Distance,
    Duration,
    Speed,
    VerticalDistance,
    Wind,
    angle,
    distance,
    speed,
    vertical_distance,
)

from .lib import lib, string


class Leg(Structure):

    @property
    def from_fix(self) -> str:
        return string(lib.efb_leg_get_from(self))

    @property
    def to_fix(self) -> str:
        return string(lib.efb_leg_get_to(self))

    @property
    def level(self) -> VerticalDistance:
        return vertical_distance(lib.efb_leg_get_level(self))

    @property
    def wind(self) -> Wind:
        return lib.efb_leg_get_wind(self).contents

    @property
    def tas(self) -> Speed:
        return speed(lib.efb_leg_get_tas(self))

    @property
    def heading(self) -> Angle:
        return angle(lib.efb_leg_get_heading(self))

    @property
    def mh(self) -> Angle:
        return angle(lib.efb_leg_get_mh(self))

    @property
    def bearing(self) -> Angle:
        return angle(lib.efb_leg_get_bearing(self))

    @property
    def mc(self) -> Angle:
        return angle(lib.efb_leg_get_mc(self))

    @property
    def dist(self) -> Distance:
        return distance(lib.efb_leg_get_dist(self))

    @property
    def gs(self) -> Speed:
        return speed(lib.efb_leg_get_gs(self))

    @property
    def ete(self) -> Duration:
        return lib.efb_leg_get_ete(self).contents


class Route(Structure):
    def __del__(self):
        lib.efb_fms_route_unref(self)

    def ete(self) -> Duration:
        return lib.efb_route_ete(self).contents

    def legs(self):
        legs = []

        if leg := lib.efb_route_legs_first(self):
            legs.append(leg.contents)

            while leg := lib.efb_route_legs_next(self):
                legs.append(leg.contents)

        return legs


lib.efb_leg_get_from.argtypes = [POINTER(Leg)]
lib.efb_leg_get_from.restype = POINTER(c_void_p)
lib.efb_leg_get_to.argtypes = [POINTER(Leg)]
lib.efb_leg_get_to.restype = POINTER(c_void_p)
lib.efb_leg_get_level.argtypes = [POINTER(Leg)]
lib.efb_leg_get_level.restype = POINTER(VerticalDistance)
lib.efb_leg_get_wind.argtypes = [POINTER(Leg)]
lib.efb_leg_get_wind.restype = POINTER(Wind)
lib.efb_leg_get_tas.argtypes = [POINTER(Leg)]
lib.efb_leg_get_tas.restype = POINTER(Speed)
lib.efb_leg_get_heading.argtypes = [POINTER(Leg)]
lib.efb_leg_get_heading.restype = POINTER(Angle)
lib.efb_leg_get_mh.argtypes = [POINTER(Leg)]
lib.efb_leg_get_mh.restype = POINTER(Angle)
lib.efb_leg_get_bearing.argtypes = [POINTER(Leg)]
lib.efb_leg_get_bearing.restype = POINTER(Angle)
lib.efb_leg_get_mc.argtypes = [POINTER(Leg)]
lib.efb_leg_get_mc.restype = POINTER(Angle)
lib.efb_leg_get_dist.argtypes = [POINTER(Leg)]
lib.efb_leg_get_dist.restype = POINTER(Distance)
lib.efb_leg_get_gs.argtypes = [POINTER(Leg)]
lib.efb_leg_get_gs.restype = POINTER(Speed)
lib.efb_leg_get_ete.argtypes = [POINTER(Leg)]
lib.efb_leg_get_ete.restype = POINTER(Duration)
lib.efb_route_legs_first.argtypes = [POINTER(Route)]
lib.efb_route_legs_first.restype = POINTER(Leg)
lib.efb_route_legs_next.argtypes = [POINTER(Route)]
lib.efb_route_legs_next.restype = POINTER(Leg)
lib.efb_route_ete.argtypes = [POINTER(Route)]
lib.efb_route_ete.restype = POINTER(Duration)
