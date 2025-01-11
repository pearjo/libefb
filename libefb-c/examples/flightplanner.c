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

/* This is the C version of the flightplanner example. */
#include <stdio.h>
#include "../include/efb.h"

const char *ARINC_424_RECORDS =
        "SEURP EDDHEDA        0        N N53374900E009591762E002000053                   P    MWGE    HAMBURG                       356462409\n"
        "SEURPCEDDHED N1    ED0    V     N53482105E010015451                                 WGE           NOVEMBER1                359892409\n"
        "SEURPCEDDHED N2    ED0    V     N53405701E010000576                                 WGE           NOVEMBER2                359902409\n"
        "SEURP EDHFEDA        0        N N53593300E009343600E000000082                   P    MWGE    ITZEHOE/HUNGRIGER WOLF        320782409";

int main(int argc, char *argv[]) {
    EfbFms *fms = efb_fms_new();
    EfbRoute *route = efb_fms_route_ref(fms);

    // read the ARINC database
    efb_fms_nd_read(fms, ARINC_424_RECORDS, Arinc424);

    // decode a route from EDDH to EDHF with winds at 20 kt from 290° and
    // cruising speed of 107 kt and an altitude of 2500 ft.
    efb_fms_decode(fms, "29020KT N0107 A0250 EDDH DHN2 DHN1 EDHF");

    printf("\n");
    printf("   Route\n");
    printf("\n");

    for (const EfbLeg *leg = efb_route_legs_first(route);
         leg != NULL;
         leg = efb_route_legs_next(route)) {

        char *from = efb_leg_get_from(leg);
        char *to = efb_leg_get_to(leg);
        char *bearing = efb_angle_to_string(efb_leg_get_bearing(leg));
        char *dist = efb_distance_to_string(efb_leg_get_dist(leg));
        char *mc = efb_angle_to_string(efb_leg_get_mc(leg));
        char *mh = efb_angle_to_string(efb_leg_get_mh(leg));
        char *ete = efb_duration_to_string(efb_leg_get_ete(leg));

        printf("%s - %s: TC: %s, dist: %s, MC: %s, MH: %s, ETE: %s\n",
               from, to, bearing, dist, mc, mh, ete);

        efb_string_free(from);
        efb_string_free(to);
        efb_string_free(bearing);
        efb_string_free(dist);
        efb_string_free(mc);
        efb_string_free(mh);
        efb_string_free(ete);
    }

    efb_fms_route_unref(route);
    efb_fms_free(fms);

    return 0;
}
