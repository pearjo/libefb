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

#ifndef EFB
#define EFB

#include <stdlib.h>

typedef enum {
  Arinc424,
  OpenAir,
} EfbInputFormat;

/// The Flight Management System (FMS).
///
/// This type wraps the [FMS] which is the integral system of this library. The
/// FMS holds all information like the navigation data or the route.
typedef struct EfbFms EfbFms;

/// The [Route] to fly.
///
/// This type is a wrapper around the [Route] with an initial cruise speed and
/// level and all legs along the route.
typedef struct EfbRoute EfbRoute;

/// A leg `from` one point `to` another.
typedef struct EfbLeg EfbLeg;

/// An array.
typedef struct {
  /// The pointer to the first element within the array.
  const EfbLeg **data;
  /// The length of the array.
  size_t len;
  /// The capacity of the array.
  size_t capacity;
} EfbArrayLeg;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/// Frees the string `s`.
///
/// # Safety
///
/// The caller must make sure that only strings that are allocated by the libefb
/// are passed to this function. It is unsafe to try freeing any string that was
/// returned by a function of this library.
void
efb_string_free(char *s);

/// Creates and returns a new FMS.
///
/// # Safety
///
/// The caller is responsible to free the allocated FMS by calling efb_fms_free.
EfbFms *
efb_fms_new(void);

/// Frees the memory of the allocated FMS.
void
efb_fms_free(EfbFms *fms);

/// Reads the string which is in the fmt into the navigation database.
///
/// # Safety
///
/// It is up to the caller to guarantee that `s` points to a valid string.
void
efb_fms_nd_read(EfbFms *fms, const char *s, EfbInputFormat fmt);

/// Reads the file at the path which is in the fmt into the navigation database.
///
/// # Safety
///
/// It is up to the caller to guarantee that `path` points to a valid string.
void
efb_fms_nd_read_file(EfbFms *fms, const char *path, EfbInputFormat fmt);

/// Decodes the route and enters it into the FMS.
///
/// # Safety
///
/// It is up to the caller to guarantee that `route` points to a valid string.
void
efb_fms_decode(EfbFms *fms, const char *route);

/// Returns a new route from the FMS.
///
/// # Safety
///
/// It's up to the caller to unref the returned route.
EfbRoute *
efb_fms_route_ref(const EfbFms *fms);

/// Decreases the reference count of the route.
void
efb_fms_route_unref(EfbRoute *route);

/// Returns an array of pointer to the legs.
///
/// # Safety
///
/// It's up to the caller to free the allocated memory of the array by
/// calling [efb_route_legs_free].
///
EfbArrayLeg
efb_route_legs_new(const EfbRoute *route);

/// Frees the memory of the legs array.
void
efb_route_legs_free(EfbArrayLeg *legs);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* EFB */
