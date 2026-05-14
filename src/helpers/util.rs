use glam::DVec3;
use rand::prelude::*;
use std::ops::Range;

// NOTE: for infinity, use f64::INFINITY
// NOTE: for pi, use f64::consts::PI
// NOTE: for degrees to radians, use f64::to_radians()
pub const ALMOST_ZERO: f64 = 1e-6;

// NOTE: rust inlines small functions automatically. I assume this counts
/// Returns a double between 0.0 and 1.0.
pub fn random_double() -> f64 {
    random_double_range(0.0..1.0)
}

/// Returns a double in the given range (exclusive).
pub fn random_double_range(range: Range<f64>) -> f64 {
    rand::rng().random_range(range)
}

/// Returns a DVec3 with all components between 0.0..1.0
pub fn random_vec3() -> DVec3 {
    random_vec3_range(0.0..1.0)
}

/// Returns a DVec3 with all components in the given range (exclusive)
pub fn random_vec3_range(range: Range<f64>) -> DVec3 {
    DVec3::new(
        random_double_range(range.clone()),
        random_double_range(range.clone()),
        // don't need to clone it the last time since it won't be used again, so can be consumed here
        random_double_range(range),
    )
}

/// Returns a random vector of unit length
pub fn random_unit_vector() -> DVec3 {
    loop {
        let p = random_vec3_range(-1.0..1.0);
        let lensq = p.length_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

/// Returns a random vector on the hemisphere around a vector (generally a normal)
pub fn random_on_hemisphere(normal: DVec3) -> DVec3 {
    let on_unit_sphere = random_unit_vector();
    // if in the same hemisphere as normal (dot product is positive):
    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

/// Generates a random vector in a unit disk
pub fn random_in_unit_disk() -> DVec3 {
    loop {
        let p = DVec3::new(
            random_double_range(-1.0..1.0),
            random_double_range(-1.0..1.0),
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

/// Checks if all elements of a vector are nearly zero
pub fn near_zero(v: DVec3) -> bool {
    (v.x.abs() < ALMOST_ZERO) && (v.y.abs() < ALMOST_ZERO) && (v.z.abs() < ALMOST_ZERO)
}
