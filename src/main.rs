use glam::DVec3;

mod helpers;
mod math;

fn main() {
    dbg!(helpers::random_double());
    let foo = math::DInterval::EMPTY;
    dbg!(foo);
    let ray = math::Ray::new(DVec3::ZERO, DVec3::ONE);
    dbg!(ray);
}
