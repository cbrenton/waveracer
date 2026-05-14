use std::ops::{Add, Mul, Sub};

pub type DInterval = Interval<f64>;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Interval<T> {
    pub min: T,
    pub max: T,
}

impl<T: Add<Output = T> + Sub<Output = T> + PartialOrd + Copy> Interval<T> {
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }

    /// Returns the distance between min and max
    pub fn size(&self) -> T {
        self.max - self.min
    }

    /// Whether the range contains the given value (inclusive)
    pub fn contains(&self, x: T) -> bool {
        self.min <= x && x <= self.max
    }

    /// Whether the range surrounds the given value (exclusive)
    pub fn surrounds(&self, x: T) -> bool {
        self.min < x && x < self.max
    }

    /// Clamp the given value to the range's min and max
    pub fn clamp(&self, x: T) -> T {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

impl DInterval {
    pub const EMPTY: Self = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    pub const UNIVERSE: Self = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    /// Converts range (0.0..1.0) to (min..max)
    pub fn scale(&self, x: f64) -> f64 {
        x * self.size() + self.min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_returns_max_minus_min() {
        let ival = DInterval::new(-1.0, 1.0);
        assert_eq!(ival.size(), 2.0);
    }

    #[test]
    fn test_contains_val_inside_range_returns_true() {
        let ival = DInterval::new(-1.0, 1.0);
        assert!(ival.contains(0.0));
    }

    #[test]
    fn test_contains_val_outside_range_returns_false() {
        let ival = DInterval::new(-1.0, 1.0);
        assert!(!ival.contains(2.0));
    }

    #[test]
    fn test_contains_val_on_boundary_returns_true() {
        let ival = DInterval::new(-1.0, 1.0);
        assert!(ival.contains(1.0));
    }

    #[test]
    fn test_surrounds_val_inside_range_returns_true() {
        let ival = DInterval::new(-1.0, 1.0);
        assert!(ival.surrounds(0.0));
    }

    #[test]
    fn test_surrounds_val_outside_range_returns_false() {
        let ival = DInterval::new(-1.0, 1.0);
        assert!(!ival.surrounds(2.0));
    }

    #[test]
    fn test_surrounds_val_on_boundary_returns_false() {
        let ival = DInterval::new(-1.0, 1.0);
        assert!(!ival.surrounds(1.0));
    }
    #[test]
    fn test_clamp_below_min_returns_min() {
        let ival = DInterval::new(-1.0, 1.0);
        assert_eq!(ival.clamp(-1.1), -1.0);
    }

    #[test]
    fn test_clamp_above_max_returns_max() {
        let ival = DInterval::new(-1.0, 1.0);
        assert_eq!(ival.clamp(1.1), 1.0);
    }

    #[test]
    fn test_clamp_in_range_returns_val() {
        let ival = DInterval::new(-1.0, 1.0);
        assert_eq!(ival.clamp(0.1), 0.1);
    }

    #[test]
    fn test_dinterval_scale_converts_0_to_min() {
        let ival = DInterval::new(-1.0, 1.0);
        assert_eq!(ival.scale(0.0), ival.min);
    }

    #[test]
    fn test_dinterval_scale_converts_1_to_max() {
        let ival = DInterval::new(-1.0, 1.0);
        assert_eq!(ival.scale(1.0), ival.max);
    }

    #[test]
    fn test_dinterval_scale_converts_point_5_to_midpoint() {
        let ival = DInterval::new(-1.0, 1.0);
        assert_eq!(ival.scale(0.5), ival.min + (ival.max - ival.min) / 2.0);
    }
}
