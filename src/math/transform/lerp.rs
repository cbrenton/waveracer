use glam::DVec3;

use crate::math::TransformFunc;

#[derive(Clone)]
pub struct Lerp {
    start: DVec3,
    end: DVec3,
    scale: f64,
    offset: f64,
}

impl Lerp {
    pub fn new(start: DVec3, end: DVec3) -> Self {
        Self {
            start,
            end,
            scale: 1.0,
            offset: 0.0,
        }
    }

    pub fn hold(start: DVec3) -> Self {
        Self {
            start,
            end: start,
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
}

impl TransformFunc for Lerp {
    fn at(&self, t: f64) -> Option<DVec3> {
        let t = (t - self.offset) * self.scale;
        if t <= 0.0 {
            Some(self.start)
        } else if t >= 1.0 {
            Some(self.end)
        } else {
            Some(self.start + (self.end - self.start) * t)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::almost_eq;

    use super::*;

    #[test]
    fn test_lerp_t_returns_start_and_end_interpolated_by_t() {
        let l = Lerp::new(DVec3::ZERO, DVec3::ONE);
        assert!(almost_eq(l.at(0.0).unwrap(), DVec3::ZERO));
        assert!(almost_eq(l.at(0.5).unwrap(), DVec3::splat(0.5)));
        assert!(almost_eq(l.at(1.0).unwrap(), DVec3::ONE));
    }

    #[test]
    fn test_lerp_negative_t_returns_start() {
        let l = Lerp::new(DVec3::ZERO, DVec3::ONE);
        assert!(almost_eq(l.at(-0.5).unwrap(), DVec3::ZERO));
    }

    #[test]
    fn test_lerp_t_greater_than_one_returns_end() {
        let l = Lerp::new(DVec3::ZERO, DVec3::ONE);
        assert!(almost_eq(l.at(1.5).unwrap(), DVec3::ONE));
    }

    #[test]
    fn test_hold_t_returns_start_regardless_of_value() {
        let l = Lerp::hold(DVec3::ZERO);
        assert!(almost_eq(l.at(-0.5).unwrap(), DVec3::ZERO));
        assert!(almost_eq(l.at(0.0).unwrap(), DVec3::ZERO));
        assert!(almost_eq(l.at(0.5).unwrap(), DVec3::ZERO));
        assert!(almost_eq(l.at(1.0).unwrap(), DVec3::ZERO));
        assert!(almost_eq(l.at(1.5).unwrap(), DVec3::ZERO));
    }

    #[test]
    fn test_end_early_returns_start_and_end_interpolated_by_scaled_t() {
        let l = Lerp::end_early(DVec3::ZERO, DVec3::ONE, 0.8);
        assert!(almost_eq(l.at(0.0).unwrap(), DVec3::ZERO));
        assert!(almost_eq(l.at(0.4).unwrap(), DVec3::splat(0.5)));
        assert!(almost_eq(l.at(0.8).unwrap(), DVec3::ONE));
    }

    #[test]
    fn test_end_early_negative_t_returns_start() {
        let l = Lerp::end_early(DVec3::ZERO, DVec3::ONE, 0.8);
        assert!(almost_eq(l.at(-0.5).unwrap(), DVec3::ZERO));
    }

    #[test]
    fn test_end_early_t_greater_than_stop_time_returns_end() {
        let l = Lerp::end_early(DVec3::ZERO, DVec3::ONE, 0.8);
        assert!(almost_eq(l.at(0.9).unwrap(), DVec3::ONE));
        assert!(almost_eq(l.at(1.0).unwrap(), DVec3::ONE));
        assert!(almost_eq(l.at(1.1).unwrap(), DVec3::ONE));
    }

    #[test]
    fn test_start_late_returns_start_and_end_interpolated_by_t_starting_from_start_time() {
        let l = Lerp::start_late(DVec3::ZERO, DVec3::ONE, 0.2);
        assert!(almost_eq(l.at(0.0).unwrap(), DVec3::ZERO));
        assert!(almost_eq(l.at(0.2).unwrap(), DVec3::ZERO));
        assert!(almost_eq(l.at(0.6).unwrap(), DVec3::splat(0.5)));
        assert!(almost_eq(l.at(1.0).unwrap(), DVec3::ONE));
    }

    #[test]
    fn test_start_late_t_less_than_start_time_returns_start() {
        let l = Lerp::start_late(DVec3::ZERO, DVec3::ONE, 0.2);
        assert!(almost_eq(l.at(-0.5).unwrap(), DVec3::ZERO));
        assert!(almost_eq(l.at(0.0).unwrap(), DVec3::ZERO));
        assert!(almost_eq(l.at(0.1).unwrap(), DVec3::ZERO));
        assert!(almost_eq(l.at(0.2).unwrap(), DVec3::ZERO));
    }

    #[test]
    fn test_start_late_t_greater_than_one_returns_start() {
        let l = Lerp::start_late(DVec3::ZERO, DVec3::ONE, 0.2);
        assert!(almost_eq(l.at(1.1).unwrap(), DVec3::ONE));
    }
}
