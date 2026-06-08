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
}

impl Rotate {
    pub fn new(start: DVec3, pivot: DVec3, angle_radians: f64) -> Self {
        Self {
            start,
            pivot,
            angle_radians,
            offset: 0.0,
            scale: 1.0,
        }
    }

    pub fn end_early(start: DVec3, pivot: DVec3, angle_radians: f64, stop_time: f64) -> Self {
        Self {
            start,
            pivot,
            angle_radians,
            scale: 1.0 / stop_time,
            offset: 0.0,
        }
    }

    pub fn start_late(start: DVec3, pivot: DVec3, angle_radians: f64, start_time: f64) -> Self {
        Self {
            start,
            pivot,
            angle_radians,
            scale: 1.0 / (1.0 - start_time),
            offset: start_time,
        }
    }

    fn scaled_t(&self, t: f64) -> f64 {
        // NOTE: I think everything should be slerped. if that's the case is there a good way to
        // reuse this?
        let t = ((t - self.offset) * self.scale).clamp(0.0, 1.0);
        (-((PI * t).cos() - 1.0) / 2.0).clamp(0.0, 1.0)
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

    use crate::math::almost_eq;

    use super::*;

    #[test]
    fn test_scaled_t_returns_correct_t() {
        /*
        let l = Rotate::new(DVec3::ZERO, DVec3::ONE);
        assert_relative_eq!(l.scaled_t(0.0), 0.0);
        assert_relative_eq!(l.scaled_t(0.5), 0.5);
        assert_relative_eq!(l.scaled_t(1.0), 1.0);

        let early_l = Rotate::end_early(DVec3::ZERO, DVec3::ONE, 0.8);
        assert_relative_eq!(early_l.scaled_t(0.0), 0.0);
        assert_relative_eq!(early_l.scaled_t(0.4), 0.5);
        assert_relative_eq!(early_l.scaled_t(0.8), 1.0);
        assert_relative_eq!(early_l.scaled_t(1.0), 1.0);

        let late_l = Rotate::start_late(DVec3::ZERO, DVec3::ONE, 0.2);
        assert_relative_eq!(late_l.scaled_t(0.0), 0.0);
        assert_relative_eq!(late_l.scaled_t(0.2), 0.0);
        assert_relative_eq!(late_l.scaled_t(0.6), 0.5);
        assert_relative_eq!(late_l.scaled_t(1.0), 1.0);
        */
    }
}
