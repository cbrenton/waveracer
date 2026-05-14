use glam::DVec3;

use rt2::math::DInterval;
use rt2::math::Ray;
use rt2::math::random;

fn main() {
    dbg!(random::random_double());
    let foo = DInterval::EMPTY;
    dbg!(foo);
    let ray = Ray::new(DVec3::ZERO, DVec3::ONE);
    dbg!(ray);
}
