use glam::DVec3;

/// A Color is a DVec3 where each component is a double of range [0.0 -> 1.0]
pub type Color = DVec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt().clamp(0.0, 1.0)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_linear_to_gamma_negative_value_returns_zero() {
        assert_relative_eq!(linear_to_gamma(-1.0), 0.0);
    }

    #[test]
    fn test_linear_to_gamma_positive_value_returns_sqrt() {
        assert_relative_eq!(linear_to_gamma(0.09), 0.3);
    }

    #[test]
    fn test_linear_to_gamma_positive_value_clamps_result() {
        assert_relative_eq!(linear_to_gamma(4.0), 1.0);
    }
}
