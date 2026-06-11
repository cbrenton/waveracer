use std::f64::consts::PI;

use glam::{DMat4, DQuat, DVec3};

use crate::math::TransformFunc;

#[derive(Clone)]
pub struct Rotate {
    start: DVec3,
    pivot: DVec3,
    angle_radians: f64,
    scale: f64,
    offset: f64,
    smoothed: bool,
}

impl Rotate {
    pub fn new(start: DVec3, pivot: DVec3, angle_radians: f64, smoothed: bool) -> Self {
        Self {
            start,
            pivot,
            angle_radians,
            offset: 0.0,
            scale: 1.0,
            smoothed,
        }
    }

    pub fn end_early(
        start: DVec3,
        pivot: DVec3,
        angle_radians: f64,
        stop_time: f64,
        smoothed: bool,
    ) -> Self {
        Self {
            start,
            pivot,
            angle_radians,
            scale: 1.0 / stop_time,
            offset: 0.0,
            smoothed,
        }
    }

    pub fn start_late(
        start: DVec3,
        pivot: DVec3,
        angle_radians: f64,
        start_time: f64,
        smoothed: bool,
    ) -> Self {
        Self {
            start,
            pivot,
            angle_radians,
            scale: 1.0 / (1.0 - start_time),
            offset: start_time,
            smoothed,
        }
    }

    fn scaled_t(&self, t: f64) -> f64 {
        // NOTE: I think everything should be slerped. if that's the case is there a good way to
        // reuse this?
        let t = ((t - self.offset) * self.scale).clamp(0.0, 1.0);
        match self.smoothed {
            true => (-((PI * t).cos() - 1.0) / 2.0).clamp(0.0, 1.0),
            false => t,
        }
    }
}

impl TransformFunc for Rotate {
    fn at(&self, t: f64) -> Option<DVec3> {
        match t {
            val if !(0.0..=1.0).contains(&val) => None,
            _ => {
                let t = self.scaled_t(t);
                let rot = DQuat::from_rotation_y(2.0 * PI * t);

                let offset = self.start - self.pivot;
                let r = rot * offset;
                Some(r + self.pivot)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::{assert_relative_eq, assert_relative_ne};

    use crate::{assert_vec_almost_eq, math::almost_eq};

    use super::*;

    #[test]
    fn test_at_rotates_around_pivot() {
        let start = DVec3::new(0.0, 0.0, 2.0);
        let l = Rotate::new(start, DVec3::new(0.0, 0.0, 1.0), 2.0 * PI, false);
        assert_vec_almost_eq!(l.at(0.0).unwrap(), start);
        assert_vec_almost_eq!(l.at(0.25).unwrap(), DVec3::new(1.0, 0.0, 1.0));
        assert_vec_almost_eq!(l.at(0.5).unwrap(), DVec3::new(0.0, 0.0, 0.0));
        assert_vec_almost_eq!(l.at(0.75).unwrap(), DVec3::new(-1.0, 0.0, 1.0));
        assert_vec_almost_eq!(l.at(1.0).unwrap(), start);
    }
}
