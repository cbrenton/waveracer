use std::f64::consts::PI;

use glam::DVec3;

use crate::math::TransformFunc;

#[derive(Clone)]
pub struct Slerp {
    start: DVec3,
    end: DVec3,
    scale: f64,
    offset: f64,
}

impl Slerp {
    pub fn new(start: DVec3, end: DVec3) -> Self {
        Self {
            start,
            end,
            scale: 1.0,
            offset: 0.0,
        }
    }

    pub fn end_early(start: DVec3, end: DVec3, stop_time: f64) -> Self {
        Self {
            start,
            end,
            scale: 1.0 / stop_time,
            offset: 0.0,
        }
    }

    pub fn start_late(start: DVec3, end: DVec3, start_time: f64) -> Self {
        Self {
            start,
            end,
            scale: 1.0 / (1.0 - start_time),
            offset: start_time,
        }
    }

    fn scaled_t(&self, t: f64) -> f64 {
        let t = ((t - self.offset) * self.scale).clamp(0.0, 1.0);
        (-((PI * t).cos() - 1.0) / 2.0).clamp(0.0, 1.0)
    }
}

impl TransformFunc for Slerp {
    fn at(&self, t: f64) -> Option<DVec3> {
        match t {
            val if !(0.0..=1.0).contains(&val) => None,
            _ => {
                let t = self.scaled_t(t);
                Some(self.start.lerp(self.end, t))
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
        let l = Slerp::new(DVec3::ZERO, DVec3::ONE);
        assert_relative_eq!(l.scaled_t(0.0), 0.0);
        assert_relative_ne!(l.scaled_t(0.1), 0.1);
        assert_relative_eq!(l.scaled_t(0.5), 0.5);
        assert_relative_ne!(l.scaled_t(0.9), 0.9);
        assert_relative_eq!(l.scaled_t(1.0), 1.0);

        let early_l = Slerp::end_early(DVec3::ZERO, DVec3::ONE, 0.8);
        assert_relative_eq!(early_l.scaled_t(0.0), 0.0);
        assert_relative_eq!(early_l.scaled_t(0.4), 0.5);
        assert_relative_eq!(early_l.scaled_t(0.8), 1.0);
        assert_relative_eq!(early_l.scaled_t(1.0), 1.0);

        let late_l = Slerp::start_late(DVec3::ZERO, DVec3::ONE, 0.2);
        assert_relative_eq!(late_l.scaled_t(0.0), 0.0);
        assert_relative_eq!(late_l.scaled_t(0.2), 0.0);
        assert_relative_eq!(late_l.scaled_t(0.6), 0.5);
        assert_relative_eq!(late_l.scaled_t(1.0), 1.0);
    }
}
