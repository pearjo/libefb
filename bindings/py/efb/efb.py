# SPDX-License-Identifier: Apache-2.0
# Copyright 2024 Joe Pearson
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

from ctypes import CDLL, POINTER, Structure, c_char_p, c_int

lib = CDLL('../libefb-c/target/debug/libefb_c.dylib')


class EfbFMS(Structure):
    pass


class EfbRoute(Structure):
    pass


lib.efb_fms_new.restype = POINTER(EfbFMS)
lib.efb_fms_free.argtypes = (POINTER(EfbFMS),)
lib.efb_fms_nd_read.argtypes = (POINTER(EfbFMS), c_char_p, c_int)
lib.efb_fms_nd_read_file.argtypes = (POINTER(EfbFMS), c_char_p, c_int)
lib.efb_fms_decode.argtypes = (POINTER(EfbFMS), c_char_p)
lib.efb_fms_route_ref.argtypes = (POINTER(EfbFMS),)
lib.efb_fms_route_ref.restype = POINTER(EfbRoute)
lib.efb_fms_route_unref.argtypes = (POINTER(EfbRoute),)


class Route:
    def __init__(self, route: EfbRoute):
        self.route = route

    def __exit__(self, exc_type, exc_value, traceback):
        lib.efb_fms_route_unref(self.route)


class FMS:
    def __init__(self):
        self.fms = lib.efb_fms_new()

    def __exit__(self, exc_type, exc_value, traceback):
        lib.efb_fms_free(self.fms)

    def read(self, s: str, fmt: int):
        lib.efb_fms_nd_read(self.fms, s.encode('utf-8'), fmt)

    def read_file(self, path: str, fmt: int):
        lib.efb_fms_nd_read_file(self.fms, path.encode('utf-8'), fmt)

    def decode(self, route: str):
        lib.efb_fms_decode(self.fms, route.encode('utf-8'))

    def route(self) -> Route:
        return Route(lib.efb_fms_route_ref(self.fms))
