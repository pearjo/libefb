use crate::fc::Wind;
use crate::geom::Angle;

fn ground_speed(tc: &Angle, tas: i16, wind: &Wind, wca: &Angle) -> i16 {
    (tas.pow(2) as f32 + wind.speed.pow(2) as f32
        - ((2 * tas * wind.speed) as f32 * (tc.rad - wind.direction.rad + wca.rad).cos()))
    .sqrt()
    .round() as i16
}

fn wind_correction_angle(wind: &Wind, tas: i16, tc: &Angle) -> Angle {
    Angle::from_rad(
        (wind.speed as f32 / tas as f32
            * Angle::from_deg(tc.deg - 180 + wind.direction.deg).rad.sin())
        .asin(),
    )
}

#[repr(C)]
pub struct Leg {
    pub tc: Angle,
    pub dist: f32,
    pub wind: Wind,
    pub var: Angle,
    pub tas: i16,
    pub gs: i16,
    pub wca: Angle,
    pub th: Angle,
    pub mc: Angle,
    pub mh: Angle,
    pub time: u32,
}

impl Leg {
    pub fn new(tc: Angle, dist: f32, wind: Wind, var: Angle, tas: i16) -> Self {
        let wca = wind_correction_angle(&wind, tas, &tc);
        let gs = ground_speed(&tc, tas, &wind, &wca);
        let th = tc + wca;

        Self {
            tc,
            dist,
            wind,
            var,
            tas,
            gs,
            wca,
            th,
            mc: tc + var,
            mh: th + var,
            time: (dist as f32 / gs as f32 * 3600.0).round() as u32,
        }
    }
}
