from ctypes import CDLL, Structure, c_float, c_int16, c_uint32

libefb = CDLL('../libefb-c/target/debug/libefb_c.dylib')


class Angle(Structure):
    _fields_ = [("deg", c_int16),
                ("rad", c_float)]

    def __repr__(self):
        return '{angle:03}'.format(angle=self.deg)


class Wind(Structure):
    _fields_ = [("direction", Angle),
                ("speed", c_float)]

    def __repr__(self):
        return '{angle!r}@{speed}'.format(angle=self.direction, speed=self.speed)


class Leg(Structure):
    _fields_ = [("tc", Angle),
                ("dist", c_float),
                ("wind", Wind),
                ("var", Angle),
                ("tas", c_int16),
                ("gs", c_int16),
                ("wca", Angle),
                ("th", Angle),
                ("mc", Angle),
                ("mh", Angle),
                ("time", c_uint32)]


fp_leg = libefb.efb_fp_leg
fp_leg.argtypes = [c_int16, c_float, c_int16, c_int16, c_int16, c_int16]
fp_leg.restype = Leg
