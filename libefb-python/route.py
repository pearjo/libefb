from dataclasses import dataclass
from functools import cached_property
from itertools import accumulate
import math


class Angle(float):
    def __repr__(self):
        return '{angle:03}'.format(angle=self.as_int() % 360)

    @classmethod
    def from_dms(cls, degree, minutes=0, seconds=0):
        return cls(degree + minutes / 60.0 + seconds / 3600.0)

    @classmethod
    def from_rad(cls, rad):
        return cls(rad * 180 / math.pi)

    def rad(self):
        return self * math.pi / 180

    def as_int(self):
        return int(round(self, 0))


@dataclass
class Wind():
    direction: Angle
    speed: int

    def __repr__(self):
        return '{angle!r}@{speed}'.format(angle=self.direction, speed=self.speed)

    @classmethod
    def from_str(cls, string):
        # 210@5
        direction, speed = string.split('@')
        return cls(Angle(direction), int(speed))


@dataclass
class Leg():
    waypoint: str
    true_course: Angle
    distance: float
    wind: Wind
    variation: Angle
    true_air_speed: int
    minimum_safe_altitude: int = 0
    altitude: int = 0

    @classmethod
    def from_str(cls, string):
        # "DHN2/9/3.2/210@5/3E/105"
        waypoint, tc, dist, wind, var, tas = string.split('/')
        return cls(
            waypoint,
            Angle(tc),
            float(dist),
            Wind.from_str(wind),
            Angle(var[:-1]) * (-1 if var[-1] == "E" else 1),
            int(tas)
        )

    @cached_property
    def wind_correction_angle(self) -> Angle:
        # sin(WCA) / WS == sin(wind angle) / TAS
        return Angle.from_rad(
            math.asin(
                self.wind.speed / self.true_air_speed * math.sin(
                    # wind angle
                    Angle(self.true_course - 180 + self.wind.direction).rad()
                )
            )
        )

    @cached_property
    def true_heading(self) -> Angle:
        return Angle(self.true_course + self.wind_correction_angle)

    @cached_property
    def ground_speed(self) -> int:
        ground_speed = math.sqrt(
            self.true_air_speed**2
            + self.wind.speed**2
            - (
                2 * self.true_air_speed * self.wind.speed * math.cos(
                    self.true_course.rad()
                    - self.wind.direction.rad()
                    + self.wind_correction_angle.rad()
                )
            )
        )
        return int(round(ground_speed, 0))

    @cached_property
    def magnetic_course(self) -> Angle:
        return Angle(self.true_course + self.variation)

    @cached_property
    def magnetic_heading(self) -> Angle:
        return Angle(self.true_heading + self.variation)

    @cached_property
    def time(self) -> float:
        return self.distance / self.ground_speed


class Route():
    def __init__(self, route):
        self.legs = list(map(Leg.from_str, route.split(' ')))

    def __repr__(self):
        lines = []
        line = ' | '.join([
            # secondary values
            '{leg.waypoint:<5}',
            '{leg.magnetic_course!r}',
            '{leg.magnetic_heading!r}',
            '{leg.distance:>5.1f}',
            '{dist_cum:>5.1f}',
            '{leg.wind_correction_angle!r}',
            '{leg.ground_speed:>4}',
            # primary values
            '{leg.minimum_safe_altitude:>5}',
            '{leg.altitude:>5}',
            # '{leg.time:5}'
        ])

        for i in range(len(self.legs)):
            lines.append(
                line.format(
                    leg=self.legs[i],
                    dist_cum=self.dist_cum[i],
                )
            )

        return '\n'.join(lines)

    @property
    def dist_cum(self):
        return list(
            accumulate(
                map(lambda leg: leg.distance, self.legs)
            )
        )

    @property
    def time_cum(self):
        return list(
            accumulate(
                map(lambda leg: leg.time, self.legs)
            )
        )
