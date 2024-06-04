from datetime import timedelta
from itertools import accumulate

from efb import *


class Route():
    def __init__(self, route):
        def leg_from_str(s):
            waypoint, tc, dist, wind, var, tas = s.split('/')
            wind_direction, wind_speed = wind.split('@')
            return fp_leg(int(tc), float(dist), int(wind_direction),
                          int(wind_speed), int(var[:-1]) * (-1 if var[-1] == "E" else 1), int(tas))

        self.legs = list(map(leg_from_str, route.split(' ')))

    def __repr__(self):
        lines = ["MC  | MH  | DIST  | CDIST | WCA |  GS  | TIME   ",
                 "----+-----+-------+-------+-----+------+--------"]
        line = ' | '.join([
            # secondary values
            # '{leg.waypoint:<5}',
            '{leg.mc!r}',
            '{leg.mh!r}',
            '{leg.dist:>5.1f}',
            '{dist_cum:>5.1f}',
            '{leg.wca!r}',
            '{leg.gs:>4}',
            # primary values
            # '{leg.minimum_safe_altitude:>5}',
            # '{leg.altitude:>5}',
            '{time}'
        ])

        for i in range(len(self.legs)):
            leg = self.legs[i]
            lines.append(
                line.format(
                    leg=leg,
                    dist_cum=self.dist_cum[i],
                    time=timedelta(seconds=leg.time)
                )
            )

        return '\n'.join(lines)

    @property
    def dist_cum(self):
        return list(
            accumulate(
                map(lambda leg: leg.dist, self.legs)
            )
        )

    @property
    def time_cum(self):
        return list(
            accumulate(
                map(lambda leg: leg.time, self.legs)
            )
        )
