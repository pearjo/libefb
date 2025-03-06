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
    Structure,
    Union,
    byref,
    cast,
    c_void_p,
    c_int,
    c_float,
)

from enum import IntEnum

from .lib import lib, string


###############################################################################
# Enums
###############################################################################


class FuelType(IntEnum):
    DIESEL = 0
    JET_A = 1


class InputFormat(IntEnum):
    ARINC_424 = 0
    OPEN_AIR = 1


###############################################################################
# Angle
###############################################################################

class _Angle(Union):
    _fields_ = [("true_north", c_float),
                ("magnetic_north", c_float)]


class Angle(Structure):
    _anonymous_ = ["u"]
    _fields_ = [("tag", c_int),
                ("u", _Angle)]

    def __str__(self) -> str:
        return string(lib.efb_angle_to_string(self))


class TrueNorth(Angle):
    pass


class MagneticNorth(Angle):
    pass


def angle(ptr: POINTER(Angle)) -> Angle:
    tag = ptr.contents.tag

    if tag == 0:
        return cast(ptr, POINTER(TrueNorth)).contents

    if tag == 1:
        return cast(ptr, POINTER(MagneticNorth)).contents


lib.efb_angle_to_string.argtypes = [POINTER(Angle)]
lib.efb_angle_to_string.restype = POINTER(c_void_p)


###############################################################################
# Distance
###############################################################################


class _Distance(Union):
    _fields_ = [("meter", c_float),
                ("nautical_miles", c_float)]


class Distance(Structure):
    _anonymous_ = ["u"]
    _fields_ = [("tag", c_int), ("u", _Distance)]

    def __str__(self):
        return string(lib.efb_distance_to_string(self))


class Meter(Distance):
    pass


class NauticalMiles(Distance):
    pass


def distance(ptr: POINTER(Distance)) -> Distance:
    tag = ptr.contents.tag

    if tag == 0:
        return cast(ptr, POINTER(Meter)).contents

    if tag == 1:
        return cast(ptr, POINTER(NauticalMiles)).contents


def meter(m: float) -> Distance:
    return lib.efb_distance_m(m)


lib.efb_distance_to_string.argtypes = [POINTER(Distance)]
lib.efb_distance_to_string.restype = POINTER(c_void_p)
lib.efb_distance_m.argtypes = [c_float]
lib.efb_distance_m.restype = Distance


###############################################################################
# Duration
###############################################################################


class Duration(Structure):
    _fields_ = [("hours", c_int),
                ("minutes", c_int),
                ("seconds", c_int)]

    def __str__(self):
        return string(lib.efb_duration_to_string(self))


lib.efb_duration_to_string.argtypes = [POINTER(Duration)]
lib.efb_duration_to_string.restype = POINTER(c_void_p)


###############################################################################
# Speed
###############################################################################


class _Speed(Union):
    _fields_ = [("knots", c_float),
                ("meter_per_second", c_float),
                ("mach", c_float)]


class Speed(Structure):
    _anonymous_ = ["u"]
    _fields_ = [("tag", c_int),
                ("u", _Speed)]

    def __str__(self):
        return string(lib.efb_speed_to_string(self))


class Knots(Speed):
    pass


class MeterPerSecond(Speed):
    pass


class Mach(Speed):
    pass


def speed(ptr: POINTER(Speed)) -> Speed:
    tag = ptr.contents.tag

    if tag == 0:
        return cast(ptr, POINTER(Knots)).contents

    if tag == 1:
        return cast(ptr, POINTER(MeterPerSecond)).contents

    if tag == 2:
        return cast(ptr, POINTER(Mach)).contents


lib.efb_speed_to_string.argtypes = [POINTER(Speed)]
lib.efb_speed_to_string.restype = POINTER(c_void_p)


###############################################################################
# Wind
###############################################################################


class Wind(Structure):
    _fields_ = [("direction", Angle),
                ("speed", Speed)]

    def __str__(self):
        return string(lib.efb_wind_to_string(self))


lib.efb_wind_to_string.argtypes = [POINTER(Wind)]
lib.efb_wind_to_string.restype = POINTER(c_void_p)


###############################################################################
# Mass
###############################################################################


class _Mass(Union):
    _fields_ = [("kilogram", c_float)]


class Mass(Structure):
    _anonymous_ = ("u",)
    _fields_ = [("tag", c_int),
                ("u", _Mass)]


class Kilogram(Mass):
    pass


def mass(ptr: POINTER(Mass)) -> Mass:
    tag = ptr.contents.tag

    if tag == 0:
        return cast(ptr, POINTER(Kilogram)).contents


def kilogram(kg: float) -> Mass:
    return lib.efb_mass_kg(kg)


lib.efb_mass_kg.argtypes = [c_float]
lib.efb_mass_kg.restype = Mass


###############################################################################
# Fuel
###############################################################################


class Fuel(Structure):
    _fields_ = [("fuel_type", c_int),
                ("mass", Mass)]


###############################################################################
# VerticalDistance
###############################################################################


class _VerticalDistance(Union):
    _fields_ = [("agl", c_int),
                ("altitude", c_int),
                ("fl", c_int),
                ("msl", c_int)]


class VerticalDistance(Structure):
    _anonymous_ = ["u"]
    _fields_ = [("tag", c_int),
                ("u", _VerticalDistance)]


    def __str__(self):
        return string(lib.efb_vertical_distance_to_string(self))


class Agl(VerticalDistance):
    pass


class Altitude(VerticalDistance):
    pass


class Fl(VerticalDistance):
    pass


class Gnd(VerticalDistance):
    pass


class Msl(VerticalDistance):
    pass


class Unlimited(VerticalDistance):
    pass


def vertical_distance(ptr: POINTER(VerticalDistance)) -> VerticalDistance:
    tag = ptr.contents.tag

    types = {
        0: Agl,
        1: Altitude,
        2: Fl,
        3: Gnd,
        4: Msl,
        5: Unlimited,
    }

    return cast(ptr, POINTER(types[tag])).contents


###############################################################################
# Volume
###############################################################################


class _Volume(Union):
    _fields_ = [("liter", c_float)]


class Volume(Structure):
    _anonymous_ = ["u"]
    _fields_ = [("tag", c_int),
                ("u", _Volume)]


class Liter(Volume):
    pass


def volume(ptr: POINTER(Volume)) -> Volume:
    tag = ptr.contents.tag

    if tag == 0:
        return cast(ptr, POINTER(Liter)).contents


def liter(l: float) -> Volume:
    return lib.efb_volume_l(l)


lib.efb_volume_l.argtypes = [c_float]
lib.efb_volume_l.restype = Volume
