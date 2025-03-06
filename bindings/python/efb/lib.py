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
    CDLL,
    POINTER,
    cast,
    c_char_p,
    c_void_p,
)

# TODO: Find the lib since this is relative from where you run the script.
lib = CDLL('../../target/debug/libefb_c.dylib')


def string(ptr):
    """Returns the C pointer `char *ptr` as an UTF-8 string."""
    try:
        return cast(ptr, c_char_p).value.decode('utf8', 'replace')
    finally:
        lib.efb_string_free(ptr)


lib.efb_string_free.argtypes = [POINTER(c_void_p)]
