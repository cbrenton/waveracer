use glam::DVec3;

// NOTE: for infinity, use f64::INFINITY
// NOTE: for pi, use f64::consts::PI
// NOTE: for degrees to radians, use f64::to_radians()
pub const ALMOST_ZERO: f64 = 1e-6;

/// Checks if all elements of a vector are nearly zero
pub fn near_zero(v: DVec3) -> bool {
    v.abs().max_element() < ALMOST_ZERO
}

/// Checks if two vectors are almost equivalent. Used to handle floating point precision errors
pub fn almost_eq(lhs: DVec3, rhs: DVec3) -> bool {
    lhs.abs_diff_eq(rhs, ALMOST_ZERO)
}

#[macro_export]
macro_rules! assert_vec_almost_eq {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val.abs_diff_eq(*right_val, 1e-6)) {
                    panic!(
                        "vector comparison failed\n  left: {:?}\n right: {:?}",
                        left_val, right_val
                    )
                }
            }
        }
    };
}
