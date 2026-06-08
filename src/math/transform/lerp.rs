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

    fn scaled_t(&self, t: f64) -> f64 {
        ((t - self.offset) * self.scale).clamp(0.0, 1.0)
    }
}

impl TransformFunc for Lerp {
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
    use approx::assert_relative_eq;

    use crate::math::almost_eq;

    use super::*;

    #[test]
    fn test_scaled_t_returns_correct_t() {
        let l = Lerp::new(DVec3::ZERO, DVec3::ONE);
        assert_relative_eq!(l.scaled_t(0.0), 0.0);
        assert_relative_eq!(l.scaled_t(0.5), 0.5);
        assert_relative_eq!(l.scaled_t(1.0), 1.0);

        let early_l = Lerp::end_early(DVec3::ZERO, DVec3::ONE, 0.8);
        assert_relative_eq!(early_l.scaled_t(0.0), 0.0);
        assert_relative_eq!(early_l.scaled_t(0.4), 0.5);
        assert_relative_eq!(early_l.scaled_t(0.8), 1.0);
        assert_relative_eq!(early_l.scaled_t(1.0), 1.0);

        let late_l = Lerp::start_late(DVec3::ZERO, DVec3::ONE, 0.2);
        assert_relative_eq!(late_l.scaled_t(0.0), 0.0);
        assert_relative_eq!(late_l.scaled_t(0.2), 0.0);
        assert_relative_eq!(late_l.scaled_t(0.6), 0.5);
        assert_relative_eq!(late_l.scaled_t(1.0), 1.0);
    }

    #[test]
    fn test_scaled_t_clamps_to_zero_one() {
        let l = Lerp::new(DVec3::ZERO, DVec3::ONE);
        assert_relative_eq!(l.scaled_t(-0.5), 0.0);
        assert_relative_eq!(l.scaled_t(1.5), 1.0);
    }

    #[test]
    fn test_at_invalid_t_returns_none() {
        let l = Lerp::new(DVec3::ZERO, DVec3::ONE);
        assert!(l.at(-0.5).is_none());
        assert!(l.at(1.5).is_none());
    }

    #[test]
    fn test_lerp_valid_t_returns_start_and_end_interpolated_by_t() {
        let l = Lerp::new(DVec3::ZERO, DVec3::ONE);
        assert!(almost_eq(l.at(0.0).unwrap(), DVec3::ZERO));
        assert!(almost_eq(l.at(0.5).unwrap(), DVec3::splat(0.5)));
        assert!(almost_eq(l.at(1.0).unwrap(), DVec3::ONE));
    }

    #[test]
    fn test_lerp_invalid_t_returns_none() {
        let l = Lerp::new(DVec3::ZERO, DVec3::ONE);
        assert!(l.at(-0.5).is_none());
        assert!(l.at(1.5).is_none());
    }

    #[test]
    fn test_hold_valid_t_returns_start_regardless_of_value() {
        let l = Lerp::hold(DVec3::ZERO);
        assert!(almost_eq(l.at(0.0).unwrap(), DVec3::ZERO));
        assert!(almost_eq(l.at(0.5).unwrap(), DVec3::ZERO));
        assert!(almost_eq(l.at(1.0).unwrap(), DVec3::ZERO));
    }

    #[test]
    fn test_hold_invalid_t_returns_none() {
        let l = Lerp::hold(DVec3::ZERO);
        assert!(l.at(-0.5).is_none());
        assert!(l.at(1.5).is_none());
    }

    #[test]
    fn test_end_early_valid_t_returns_start_and_end_interpolated_by_scaled_t() {
        let l = Lerp::end_early(DVec3::ZERO, DVec3::ONE, 0.8);
        assert!(almost_eq(l.at(0.0).unwrap(), DVec3::ZERO));
        assert!(almost_eq(l.at(0.4).unwrap(), DVec3::splat(0.5)));
        assert!(almost_eq(l.at(0.8).unwrap(), DVec3::ONE));
        assert!(almost_eq(l.at(0.9).unwrap(), DVec3::ONE));
        assert!(almost_eq(l.at(1.0).unwrap(), DVec3::ONE));
    }

    #[test]
    fn test_end_early_invalid_t_returns_none() {
        let l = Lerp::end_early(DVec3::ZERO, DVec3::ONE, 0.8);
        assert!(l.at(-0.5).is_none());
        assert!(l.at(1.5).is_none());
    }

    #[test]
    fn test_start_late_valid_t_returns_start_and_end_interpolated_by_t_starting_from_start_time() {
        let l = Lerp::start_late(DVec3::ZERO, DVec3::ONE, 0.2);
        assert!(almost_eq(l.at(0.0).unwrap(), DVec3::ZERO));
        assert!(almost_eq(l.at(0.1).unwrap(), DVec3::ZERO));
        assert!(almost_eq(l.at(0.2).unwrap(), DVec3::ZERO));
        assert!(almost_eq(l.at(0.6).unwrap(), DVec3::splat(0.5)));
        assert!(almost_eq(l.at(1.0).unwrap(), DVec3::ONE));
    }

    #[test]
    fn test_start_late_invalid_t_returns_none() {
        let l = Lerp::start_late(DVec3::ZERO, DVec3::ONE, 0.2);
        assert!(l.at(-0.5).is_none());
        assert!(l.at(1.5).is_none());
    }
}
