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
/// This type is a wrapper around the [Route] with an initial cruise speed,
/// level and all legs along the route.
///
/// The [`efb_route_legs_first`] and [`efb_route_legs_next`] functions return a
/// leg of the route and can be used to iterate over the route:
///
/// ```
/// for (const EfbLeg *leg = efb_route_legs_first(route);
///      leg != NULL;
///      leg = efb_route_legs_next(route))
/// ```
typedef struct EfbRoute EfbRoute;

/// A leg `from` one point `to` another.
typedef struct EfbLeg EfbLeg;

/// An angle in the range from 0° to 360°.
///
/// An angle in degree as [`i16`] or in radians as [`f32`] can be converted into
/// an Angle and it's value will be wrapped into the range from 0° to 360°.
///
/// ```
/// use efb::Angle;
/// let west: Angle = (-90).into();
/// assert_eq!(west.as_degrees(), 270);
/// ```
typedef enum {
  True,
  Magnetic,
} EfbAngle_Tag;

typedef struct {
  EfbAngle_Tag tag;
  union {
    struct {
      float true_;
    };
    struct {
      float magnetic;
    };
  };
} EfbAngle;

/// The speed in either nautical or metrical units.
typedef enum {
  Knots,
  MeterPerSecond,
  Mach,
} EfbSpeed_Tag;

typedef struct {
  EfbSpeed_Tag tag;
  union {
    struct {
      float knots;
    };
    struct {
      float meter_per_second;
    };
    struct {
      float mach;
    };
  };
} EfbSpeed;

/// The wind with a speed and direction
typedef struct {
  /// The direction from which the wind comes.
  EfbAngle direction;
  /// The wind speed.
  EfbSpeed speed;
} EfbWind;

/// A vertical distance.
typedef enum {
  /// Absolute Altitude as distance above ground level in feet.
  Agl,
  /// Altitude in feet with reference to a local air pressure.
  Altitude,
  /// Flight level in hundreds of feet as altitude at standard air pressure.
  Fl,
  /// Ground level.
  Gnd,
  /// True Altitude as distance above mean sea level.
  Msl,
  /// An unlimited vertical distance.
  Unlimited,
} EfbVerticalDistance_Tag;

typedef struct {
  EfbVerticalDistance_Tag tag;
  union {
    struct {
      uint16_t agl;
    };
    struct {
      uint16_t altitude;
    };
    struct {
      uint16_t fl;
    };
    struct {
      uint16_t msl;
    };
  };
} EfbVerticalDistance;

/// A metrical or nautical distance.
typedef enum {
  Meter,
  NauticalMiles,
} EfbDistance_Tag;

typedef struct {
  EfbDistance_Tag tag;
  union {
    struct {
      float meter;
    };
    struct {
      float nautical_miles;
    };
  };
} EfbDistance;

/// A duration measured in hours, minutes and seconds.
typedef struct {
  uint8_t hours;
  uint8_t minutes;
  uint8_t seconds;
} EfbDuration;

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

/// Returns a reference to the FMS route.
///
/// # Safety
///
/// It's up to the caller to unref the returned pointer.
EfbRoute *
efb_fms_route_ref(EfbFms *fms);

/// Decreases the reference count of the route.
void
efb_fms_route_unref(EfbRoute *route);

/// Returns the first leg in the route.
const EfbLeg *
efb_route_legs_first(EfbRoute *route);

/// Returns the next leg in the route.
///
/// When the end of the legs is reached, this function returns a null pointer.
const EfbLeg *
efb_route_legs_next(EfbRoute *route);

/// Returns the ident from where the leg starts.
///
/// # Safety
///
/// The returned value needs to be freed by [`efb_string_free`].
char *
efb_leg_get_from(const EfbLeg *leg);

/// Returns the ident to where the leg ends.
///
/// # Safety
///
/// The returned value needs to be freed by [`efb_string_free`].
char *
efb_leg_get_to(const EfbLeg *leg);

/// Returns the leg's level or null if unknown.
const EfbVerticalDistance *
efb_leg_get_level(const EfbLeg *leg);

/// Returns the wind along the leg or null if unknown.
const EfbWind *
efb_leg_get_wind(const EfbLeg *leg);

/// Returns the leg's true airspeed or null if unknown.
const EfbSpeed *
efb_leg_get_tas(const EfbLeg *leg);

/// Returns the true heading considering the WCA or null if unknown.
const EfbAngle *
efb_leg_get_heading(const EfbLeg *leg);

/// Returns the magnetic heading considering the variation at the start of the
/// leg or null if unknown.
const EfbAngle *
efb_leg_get_mh(const EfbLeg *leg);

/// Returns the bearing between the two points.
const EfbAngle *
efb_leg_get_bearing(const EfbLeg *leg);

/// Returns the magnetic course taking the magnetic variation from the starting
/// point into consideration.
const EfbAngle *
efb_leg_get_mc(const EfbLeg *leg);

/// Returns the distance between the leg's two points.
const EfbDistance *
efb_leg_get_dist(const EfbLeg *leg);

/// Returns the ground speed in knots or null if unknown.
const EfbSpeed *
efb_leg_get_gs(const EfbLeg *leg);

/// Returns the estimated time enroute the leg or null if unknown.
const EfbDuration *
efb_leg_get_ete(const EfbLeg *leg);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* EFB */
