use glam::DVec3;

use rt2::helpers;
use rt2::math;

fn main() {
    dbg!(helpers::random_double());
    let foo = math::DInterval::EMPTY;
    dbg!(foo);
    let ray = math::Ray::new(DVec3::ZERO, DVec3::ONE);
    dbg!(ray);
}
